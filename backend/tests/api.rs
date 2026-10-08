use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;
use vocadb_notification_reader::service::Database;
use vocadb_notification_reader::session::{Session, SessionStore};
use vocadb_notification_reader::web::{AppState, router};
use wiremock::matchers::{body_string_contains, header as header_eq, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const USER_ID: i32 = 42;
const SESSION_COOKIE: &str = ".AspNetCore.Cookies=session";

struct TestApp {
    vocadb: MockServer,
    sessions: SessionStore,
    router: Router,
}

impl TestApp {
    async fn new() -> Self {
        let vocadb = MockServer::start().await;
        let sessions = SessionStore::memory();
        let urls = Database::ALL
            .into_iter()
            .map(|db| (db, vocadb.uri()))
            .collect();
        let router = router(AppState::new(sessions.clone(), urls).unwrap());

        TestApp {
            vocadb,
            sessions,
            router,
        }
    }

    async fn token(&self) -> String {
        let session = Session {
            user_id: USER_ID,
            database: Database::VocaDb,
            cookies: vec![SESSION_COOKIE.to_string()],
        };
        self.sessions.create(&session).await.unwrap()
    }

    async fn post(&self, uri: &str, token: Option<&str>, body: Value) -> (StatusCode, Value) {
        let mut request = Request::post(uri).header(header::CONTENT_TYPE, "application/json");
        if let Some(token) = token {
            request = request.header(header::AUTHORIZATION, format!("Bearer {token}"));
        }
        let request = request.body(Body::from(body.to_string())).unwrap();
        self.send(request).await
    }

    async fn send(&self, request: Request<Body>) -> (StatusCode, Value) {
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into()));
        (status, body)
    }
}

fn user_json() -> Value {
    json!({
        "id": USER_ID,
        "name": "miku",
        "active": true,
        "memberSince": "2020-01-01T00:00:00",
        "verifiedArtist": false,
        "groupId": "Regular",
        "mainPicture": { "urlThumb": "https://example.com/thumb.png" }
    })
}

fn message_json(id: i32, subject: &str, body: &str) -> Value {
    json!({
        "id": id,
        "subject": subject,
        "body": body,
        "createdFormatted": "2/25/2022 2:29 PM",
        "highPriority": false,
        "inbox": "Notifications",
        "read": false,
        "receiver": user_json()
    })
}

async fn mock_current_user(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/api/users/current"))
        .and(header_eq("cookie", SESSION_COOKIE))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_json()))
        .mount(server)
        .await;
}

#[tokio::test]
async fn health() {
    let app = TestApp::new().await;
    let request = Request::get("/health").body(Body::empty()).unwrap();
    assert_eq!(app.send(request).await, (StatusCode::OK, json!("OK")));
}

#[tokio::test]
async fn login_returns_token_with_session_cookies() {
    let app = TestApp::new().await;
    Mock::given(method("POST"))
        .and(path("/User/Login"))
        .and(body_string_contains("UserName=miku"))
        .and(body_string_contains("Password=secret"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("location", "/")
                .append_header("set-cookie", "unrelated=1; path=/")
                .append_header("set-cookie", format!("{SESSION_COOKIE}; path=/; httponly")),
        )
        .expect(1)
        .mount(&app.vocadb)
        .await;
    mock_current_user(&app.vocadb).await;

    let (status, body) = app
        .post(
            "/api/login",
            None,
            json!({ "username": "miku", "password": "secret", "database": "UtaiteDb" }),
        )
        .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    let session = app
        .sessions
        .get(body["token"].as_str().unwrap())
        .await
        .unwrap()
        .expect("session must be stored");
    assert_eq!(
        session,
        Session {
            user_id: USER_ID,
            database: Database::UtaiteDb,
            cookies: vec![SESSION_COOKIE.to_string()],
        }
    );
}

#[tokio::test]
async fn login_with_bad_credentials_is_unauthorized() {
    let app = TestApp::new().await;
    Mock::given(method("POST"))
        .and(path("/User/Login"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html>login form</html>"))
        .mount(&app.vocadb)
        .await;

    let (status, body) = app
        .post(
            "/api/login",
            None,
            json!({ "username": "miku", "password": "wrong", "database": "VocaDb" }),
        )
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["code"], 401);
}

#[tokio::test]
async fn malformed_payload_is_rejected() {
    let app = TestApp::new().await;

    let (status, body) = app
        .post(
            "/api/login",
            None,
            json!({ "username": "miku", "database": "Nope" }),
        )
        .await;

    assert!(status.is_client_error(), "{status}");
    assert_eq!(body["code"], status.as_u16());
}

#[tokio::test]
async fn protected_endpoints_require_valid_token() {
    let app = TestApp::new().await;
    let deleted = app.token().await;
    app.sessions.delete(&deleted).await.unwrap();

    for token in [None, Some("garbage"), Some(deleted.as_str())] {
        for uri in ["/api/users/current", "/api/notifications/delete"] {
            let (status, _) = app.post(uri, token, json!({ "ids": [] })).await;
            assert_eq!(status, StatusCode::UNAUTHORIZED, "{uri} with {token:?}");
        }
    }
}

#[tokio::test]
async fn current_user_is_proxied() {
    let app = TestApp::new().await;
    mock_current_user(&app.vocadb).await;

    let (status, body) = app
        .post("/api/users/current", Some(&app.token().await), json!({}))
        .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], USER_ID);
    assert_eq!(body["name"], "miku");
    assert_eq!(
        body["mainPicture"]["urlThumb"],
        "https://example.com/thumb.png"
    );
}

#[tokio::test]
async fn expired_upstream_session_is_unauthorized() {
    let app = TestApp::new().await;
    Mock::given(method("GET"))
        .and(path("/api/users/current"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&app.vocadb)
        .await;

    let (status, _) = app
        .post("/api/users/current", Some(&app.token().await), json!({}))
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn upstream_failure_is_bad_gateway() {
    let app = TestApp::new().await;
    Mock::given(method("GET"))
        .and(path("/api/users/current"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&app.vocadb)
        .await;

    let (status, body) = app
        .post("/api/users/current", Some(&app.token().await), json!({}))
        .await;

    assert_eq!(status, StatusCode::BAD_GATEWAY);
    assert_eq!(body["code"], 502);
}

#[tokio::test]
async fn fetch_notifications() {
    let app = TestApp::new().await;
    let server = &app.vocadb;

    Mock::given(method("GET"))
        .and(path(format!("/api/users/{USER_ID}/messages")))
        .and(query_param("inbox", "Notifications"))
        .and(query_param("start", "10"))
        .and(query_param("maxResults", "3"))
        .and(header_eq("cookie", SESSION_COOKIE))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [
                message_json(1, "New song tagged", ""),
                message_json(2, "New artist", ""),
                message_json(3, "Hello", ""),
            ],
            "totalCount": 13
        })))
        .mount(server)
        .await;

    for (id, subject, body) in [
        (
            1,
            "New song tagged with Miku",
            "Song: https://vocadb.net/S/100",
        ),
        (2, "New artist", "Artist: https://vocadb.net/Ar/5"),
        (3, "Hello", "Plain message"),
    ] {
        Mock::given(method("GET"))
            .and(path(format!("/api/users/messages/{id}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(message_json(id, subject, body)))
            .mount(server)
            .await;
    }

    Mock::given(method("GET"))
        .and(path("/api/songs/100"))
        .and(query_param("lang", "Romaji"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 100,
            "name": "Melt",
            "artistString": "ryo feat. Hatsune Miku",
            "publishDate": "2007-12-07T00:00:00Z",
            "songType": "Original",
            "pvServices": "NicoNicoDouga, Piapro",
            "tags": [
                { "count": 5, "tag": { "id": 1, "name": "rock", "categoryName": "Genres" } },
                { "count": 1, "tag": { "id": 2, "name": "pop", "categoryName": null } },
                { "count": 5, "tag": { "id": 3, "name": "kawaii" } }
            ],
            "pvs": [
                {
                    "id": 11, "pvType": "Reprint", "service": "Piapro",
                    "url": "https://piapro.jp/t/x", "name": "piapro", "disabled": false,
                    "pvId": "x", "length": 100,
                    "extendedMetadata": { "json": "{\"Timestamp\":\"20071207\"}" }
                },
                {
                    "id": 10, "pvType": "Original", "service": "NicoNicoDouga",
                    "url": "https://nicovideo.jp/watch/sm1715919", "name": "nico", "disabled": false,
                    "pvId": "sm1715919", "length": 100
                }
            ]
        })))
        .mount(server)
        .await;

    let (status, body) = app
        .post(
            "/api/notifications/fetch",
            Some(&app.token().await),
            json!({ "startOffset": 10, "maxResults": 3, "language": "Romaji" }),
        )
        .await;

    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["totalCount"], 13);

    let notifications = body["notifications"].as_array().unwrap();
    assert_eq!(notifications.len(), 3);

    let song = &notifications[0];
    assert_eq!(song["notificationType"], "SongNotification");
    assert_eq!(song["id"], 1);
    assert_eq!(song["originalSubject"], "New song tagged with Miku");
    assert_eq!(song["originalBody"], "Song: https://vocadb.net/S/100");
    assert_eq!(song["created_date"], "2022-02-25T14:29:00Z");
    assert_eq!(song["type"], "Tagged");
    assert_eq!(song["songId"], 100);
    assert_eq!(song["songType"], "Original");
    assert_eq!(song["title"], "Melt");
    assert_eq!(song["artist"], "ryo feat. Hatsune Miku");
    assert_eq!(song["releaseDate"], "2007-12-07T00:00:00Z");

    let tags: Vec<&str> = song["tags"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(tags, ["pop", "kawaii", "rock"]);
    assert_eq!(song["tags"][2]["categoryName"], "Genres");

    let pvs = song["pvs"].as_array().unwrap();
    assert_eq!(pvs[0]["service"], "NicoNicoDouga");
    assert_eq!(pvs[0]["pvType"], "Original");
    assert_eq!(pvs[0]["timestamp"], Value::Null);
    assert_eq!(pvs[1]["service"], "Piapro");
    assert_eq!(pvs[1]["timestamp"], "20071207");

    assert_eq!(notifications[1]["notificationType"], "ArtistNotification");
    assert_eq!(notifications[1]["id"], 2);
    assert_eq!(notifications[2]["notificationType"], "UnknownNotification");
    assert_eq!(notifications[2]["originalBody"], "Plain message");
}

#[tokio::test]
async fn fetch_notifications_validates_paging() {
    let app = TestApp::new().await;
    let token = app.token().await;

    for (start, max) in [(-1, 10), (0, -1), (0, 101)] {
        let (status, body) = app
            .post(
                "/api/notifications/fetch",
                Some(&token),
                json!({ "startOffset": start, "maxResults": max, "language": "Default" }),
            )
            .await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{start} {max}");
        assert_eq!(body["code"], 400);
    }
}

#[tokio::test]
async fn delete_notifications() {
    let app = TestApp::new().await;
    Mock::given(method("DELETE"))
        .and(path(format!("/api/users/{USER_ID}/messages")))
        .and(query_param("messageId", "1"))
        .and(query_param("messageId", "2"))
        .and(header_eq("cookie", SESSION_COOKIE))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&app.vocadb)
        .await;

    let (status, body) = app
        .post(
            "/api/notifications/delete",
            Some(&app.token().await),
            json!({ "ids": [1, 2] }),
        )
        .await;

    assert_eq!((status, body), (StatusCode::OK, Value::Null));
}

#[tokio::test]
async fn cors_preflight_allows_configured_origins() {
    use vocadb_notification_reader::config::{Config, default_database_urls};
    use vocadb_notification_reader::web::app;

    let app = app(Config {
        listen_addr: "127.0.0.1:0".parse().unwrap(),
        redis_url: None,
        cors_allowed_origins: vec!["https://reader.example.com".to_string()],
        database_urls: default_database_urls(),
    })
    .await
    .unwrap();

    let preflight = |origin: &str| {
        Request::options("/api/notifications/fetch")
            .header(header::ORIGIN, origin)
            .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
            .header(
                header::ACCESS_CONTROL_REQUEST_HEADERS,
                "authorization,content-type",
            )
            .body(Body::empty())
            .unwrap()
    };

    let response = app
        .clone()
        .oneshot(preflight("https://reader.example.com"))
        .await
        .unwrap();
    let headers = response.headers();
    assert_eq!(
        headers[header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "https://reader.example.com"
    );
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_CREDENTIALS], "true");

    let response = app
        .oneshot(preflight("https://evil.example.com"))
        .await
        .unwrap();
    assert!(
        !response
            .headers()
            .contains_key(header::ACCESS_CONTROL_ALLOW_ORIGIN)
    );
}

#[tokio::test]
async fn logout_invalidates_session() {
    let app = TestApp::new().await;
    mock_current_user(&app.vocadb).await;
    let token = app.token().await;

    let (status, _) = app
        .post("/api/users/current", Some(&token), json!({}))
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = app.post("/api/logout", Some(&token), json!({})).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(app.sessions.get(&token).await.unwrap(), None);

    let (status, _) = app
        .post("/api/users/current", Some(&token), json!({}))
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
