use axum::Router;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;
use vocadb_notification_reader::cache::{Cache, MESSAGE_TTL};
use vocadb_notification_reader::kv::Kv;
use vocadb_notification_reader::service::Database;
use vocadb_notification_reader::session::{Session, SessionStore};
use vocadb_notification_reader::web::{AppState, SESSION_COOKIE, router};
use wiremock::matchers::{body_string_contains, header as header_eq, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const USER_ID: i32 = 42;
const VOCADB_COOKIE: &str = ".AspNetCore.Cookies=session";
const FRONTEND: &str = "https://foobar.com";

struct TestApp {
    vocadb: MockServer,
    kv: Kv,
    sessions: SessionStore,
    router: Router,
}

struct Response {
    status: StatusCode,
    headers: axum::http::HeaderMap,
    body: Value,
}

impl Response {
    /// Value of the session cookie set by the response.
    fn session_cookie(&self) -> Option<String> {
        self.headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .find(|v| v.starts_with(&format!("{SESSION_COOKIE}=")))
            .map(String::from)
    }
}

impl TestApp {
    async fn new() -> Self {
        let vocadb = MockServer::start().await;
        let kv = Kv::memory();
        let sessions = SessionStore::new(kv.clone());
        let urls = Database::ALL
            .into_iter()
            .map(|db| (db, vocadb.uri()))
            .collect();
        let state = AppState::new(kv.clone(), urls, vec![FRONTEND.to_string()]).unwrap();

        TestApp {
            vocadb,
            kv,
            sessions,
            router: router(state),
        }
    }

    async fn session(&self) -> String {
        let session = Session {
            user_id: USER_ID,
            database: Database::VocaDb,
            cookies: vec![VOCADB_COOKIE.to_string()],
            expires_at: None,
        };
        self.sessions.create(&session).await.unwrap()
    }

    fn request(method: Method, uri: &str, session: Option<&str>) -> axum::http::request::Builder {
        let mut request = Request::builder()
            .method(method)
            .uri(uri)
            .header(header::HOST, "api.foobar.com")
            .header(header::ORIGIN, FRONTEND);
        if let Some(session) = session {
            request = request.header(header::COOKIE, format!("{SESSION_COOKIE}={session}"));
        }
        request
    }

    async fn get(&self, uri: &str, session: Option<&str>) -> Response {
        self.send(
            Self::request(Method::GET, uri, session)
                .body(Body::empty())
                .unwrap(),
        )
        .await
    }

    async fn json(
        &self,
        method: Method,
        uri: &str,
        session: Option<&str>,
        body: Value,
    ) -> Response {
        self.send(
            Self::request(method, uri, session)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
    }

    async fn send(&self, request: Request<Body>) -> Response {
        let response = self.router.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body = serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into()));
        Response {
            status,
            headers,
            body,
        }
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
        .and(header_eq("cookie", VOCADB_COOKIE))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_json()))
        .mount(server)
        .await;
}

#[tokio::test]
async fn health() {
    let app = TestApp::new().await;
    let response = app.get("/health", None).await;
    assert_eq!(
        (response.status, response.body),
        (StatusCode::OK, json!("OK"))
    );
}

#[tokio::test]
async fn login_sets_session_cookie() {
    let app = TestApp::new().await;
    Mock::given(method("POST"))
        .and(path("/User/Login"))
        .and(body_string_contains("UserName=miku"))
        .and(body_string_contains("Password=secret"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("location", "/")
                .append_header("set-cookie", "unrelated=1; path=/")
                .append_header(
                    "set-cookie",
                    format!(
                        "{VOCADB_COOKIE}; expires=Fri, 01 Jan 2100 00:00:00 GMT; path=/; httponly"
                    ),
                ),
        )
        .expect(1)
        .mount(&app.vocadb)
        .await;
    mock_current_user(&app.vocadb).await;

    let response = app
        .json(
            Method::POST,
            "/api/session",
            None,
            json!({ "username": "miku", "password": "secret", "database": "UtaiteDb" }),
        )
        .await;

    assert_eq!(response.status, StatusCode::OK, "{}", response.body);
    assert_eq!(response.body["database"], "UtaiteDb");
    assert_eq!(response.body["user"]["name"], "miku");

    let cookie = response
        .session_cookie()
        .expect("session cookie must be set");
    for attribute in [
        "HttpOnly",
        "Secure",
        "SameSite=Strict",
        "Path=/",
        "Max-Age=",
    ] {
        assert!(cookie.contains(attribute), "{cookie} lacks {attribute}");
    }

    let id = cookie
        .split(';')
        .next()
        .and_then(|pair| pair.split_once('='))
        .map(|(_, value)| value)
        .unwrap();
    let session = app.sessions.get(id).await.unwrap().expect("session stored");
    assert_eq!(
        session,
        Session {
            user_id: USER_ID,
            database: Database::UtaiteDb,
            cookies: vec![VOCADB_COOKIE.to_string()],
            expires_at: "2100-01-01T00:00:00Z".parse().ok(),
        }
    );
    // The token never appears in the body.
    assert!(!response.body.to_string().contains(id));
}

#[tokio::test]
async fn login_with_bad_credentials_is_unauthorized() {
    let app = TestApp::new().await;
    Mock::given(method("POST"))
        .and(path("/User/Login"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<html>login form</html>"))
        .mount(&app.vocadb)
        .await;

    let response = app
        .json(
            Method::POST,
            "/api/session",
            None,
            json!({ "username": "miku", "password": "wrong", "database": "VocaDb" }),
        )
        .await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
    assert_eq!(response.body["code"], 401);
    assert_eq!(response.session_cookie(), None);
}

#[tokio::test]
async fn malformed_requests_are_rejected() {
    let app = TestApp::new().await;
    let session = app.session().await;

    let response = app
        .json(
            Method::POST,
            "/api/session",
            None,
            json!({ "username": "miku", "database": "Nope" }),
        )
        .await;
    assert!(response.status.is_client_error(), "{}", response.status);
    assert_eq!(response.body["code"], response.status.as_u16());

    let response = app
        .get("/api/notifications?offset=x&limit=1", Some(&session))
        .await;
    assert!(response.status.is_client_error(), "{}", response.status);
    assert_eq!(response.body["code"], response.status.as_u16());
}

#[tokio::test]
async fn protected_endpoints_require_a_session() {
    let app = TestApp::new().await;
    let ended = app.session().await;
    app.sessions.delete(&ended).await.unwrap();

    for session in [None, Some("garbage"), Some(ended.as_str())] {
        let response = app.get("/api/me", session).await;
        assert_eq!(response.status, StatusCode::UNAUTHORIZED, "{session:?}");

        let response = app
            .json(
                Method::DELETE,
                "/api/notifications",
                session,
                json!({ "ids": [1] }),
            )
            .await;
        assert_eq!(response.status, StatusCode::UNAUTHORIZED, "{session:?}");
    }
}

#[tokio::test]
async fn me_returns_account_and_extends_cookie() {
    let app = TestApp::new().await;
    mock_current_user(&app.vocadb).await;
    let session = app.session().await;

    let response = app.get("/api/me", Some(&session)).await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body["database"], "VocaDb");
    assert_eq!(response.body["user"]["id"], USER_ID);
    assert_eq!(
        response.body["user"]["mainPicture"]["urlThumb"],
        "https://example.com/thumb.png"
    );
    let cookie = response.session_cookie().expect("cookie re-issued");
    assert!(cookie.starts_with(&format!("{SESSION_COOKIE}={session};")));
}

#[tokio::test]
async fn logout_ends_session_and_clears_cookie() {
    let app = TestApp::new().await;
    mock_current_user(&app.vocadb).await;
    let session = app.session().await;

    assert_eq!(
        app.get("/api/me", Some(&session)).await.status,
        StatusCode::OK
    );

    let response = app
        .json(Method::DELETE, "/api/session", Some(&session), json!(null))
        .await;
    assert_eq!(response.status, StatusCode::OK);
    let cookie = response.session_cookie().expect("cookie cleared");
    assert!(cookie.contains("Max-Age=0"), "{cookie}");
    assert_eq!(app.sessions.get(&session).await.unwrap(), None);

    assert_eq!(
        app.get("/api/me", Some(&session)).await.status,
        StatusCode::UNAUTHORIZED
    );

    // Logging out without a session is a no-op.
    let response = app
        .json(Method::DELETE, "/api/session", None, json!(null))
        .await;
    assert_eq!(response.status, StatusCode::OK);
}

#[tokio::test]
async fn cross_site_requests_are_forbidden() {
    let app = TestApp::new().await;
    let session = app.session().await;

    let request = |origin: &str, host: &str| {
        Request::delete("/api/session")
            .header(header::HOST, host)
            .header(header::ORIGIN, origin)
            .header(header::COOKIE, format!("{SESSION_COOKIE}={session}"))
            .body(Body::empty())
            .unwrap()
    };

    // A sibling subdomain is same-site, but not an allowed origin.
    let response = app
        .send(request("https://evil.foobar.com", "api.foobar.com"))
        .await;
    assert_eq!(response.status, StatusCode::FORBIDDEN);
    assert!(app.sessions.get(&session).await.unwrap().is_some());

    // Same origin requests are fine (e.g. the dev server proxy).
    let response = app
        .send(request("http://localhost:5173", "localhost:5173"))
        .await;
    assert_eq!(response.status, StatusCode::OK);
    assert!(app.sessions.get(&session).await.unwrap().is_none());
}

#[tokio::test]
async fn reads_are_not_origin_checked() {
    let app = TestApp::new().await;
    mock_current_user(&app.vocadb).await;
    let session = app.session().await;

    let response = app
        .send(
            Request::get("/api/me")
                .header(header::HOST, "api.foobar.com")
                .header(header::ORIGIN, "https://other.example")
                .header(header::COOKIE, format!("{SESSION_COOKIE}={session}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await;
    // CORS keeps other origins from reading the response.
    assert_eq!(response.status, StatusCode::OK);
}

#[tokio::test]
async fn ended_upstream_session_ends_the_session() {
    let app = TestApp::new().await;
    Mock::given(method("GET"))
        .and(path("/api/users/current"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&app.vocadb)
        .await;
    let session = app.session().await;

    let response = app.get("/api/me", Some(&session)).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
    assert_eq!(app.sessions.get(&session).await.unwrap(), None);
}

#[tokio::test]
async fn redirect_to_login_ends_the_session() {
    let app = TestApp::new().await;
    Mock::given(method("GET"))
        .and(path("/api/users/current"))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("location", "/User/Login?ReturnUrl=%2Fapi%2Fusers%2Fcurrent"),
        )
        .mount(&app.vocadb)
        .await;
    let session = app.session().await;

    let response = app.get("/api/me", Some(&session)).await;

    assert_eq!(response.status, StatusCode::UNAUTHORIZED);
    assert_eq!(app.sessions.get(&session).await.unwrap(), None);
}

#[tokio::test]
async fn refreshed_upstream_cookies_are_saved() {
    let app = TestApp::new().await;
    Mock::given(method("GET"))
        .and(path("/api/users/current"))
        .and(header_eq("cookie", VOCADB_COOKIE))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(user_json())
                .insert_header(
                    "set-cookie",
                    ".AspNetCore.Cookies=refreshed; expires=Fri, 01 Jan 2100 00:00:00 GMT; path=/",
                ),
        )
        .expect(1)
        .mount(&app.vocadb)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/users/current"))
        .and(header_eq("cookie", ".AspNetCore.Cookies=refreshed"))
        .respond_with(ResponseTemplate::new(200).set_body_json(user_json()))
        .expect(1)
        .mount(&app.vocadb)
        .await;
    let session = app.session().await;

    for _ in 0..2 {
        assert_eq!(
            app.get("/api/me", Some(&session)).await.status,
            StatusCode::OK
        );
    }

    let stored = app.sessions.get(&session).await.unwrap().unwrap();
    assert_eq!(stored.cookies, [".AspNetCore.Cookies=refreshed"]);
    assert_eq!(stored.expires_at, "2100-01-01T00:00:00Z".parse().ok());
}

#[tokio::test]
async fn upstream_failure_is_bad_gateway() {
    let app = TestApp::new().await;
    Mock::given(method("GET"))
        .and(path("/api/users/current"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&app.vocadb)
        .await;
    let session = app.session().await;

    let response = app.get("/api/me", Some(&session)).await;

    assert_eq!(response.status, StatusCode::BAD_GATEWAY);
    assert_eq!(response.body["code"], 502);
    // A failing VocaDB doesn't end the session.
    assert!(app.sessions.get(&session).await.unwrap().is_some());
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
        .and(header_eq("cookie", VOCADB_COOKIE))
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
            .expect(1)
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
        .expect(1)
        .mount(server)
        .await;

    let session = app.session().await;
    let uri = "/api/notifications?offset=10&limit=3&language=Romaji";
    let response = app.get(uri, Some(&session)).await;
    assert_eq!(response.status, StatusCode::OK, "{}", response.body);

    // Messages and songs are served from the cache the second time.
    let cached = app.get(uri, Some(&session)).await;
    assert_eq!(cached.status, StatusCode::OK);
    assert_eq!(cached.body, response.body);

    let body = response.body;
    assert_eq!(body["totalCount"], 13);

    let notifications = body["notifications"].as_array().unwrap();
    assert_eq!(notifications.len(), 3);

    let song = &notifications[0];
    assert_eq!(song["notificationType"], "SongNotification");
    assert_eq!(song["id"], 1);
    assert_eq!(song["originalSubject"], "New song tagged with Miku");
    assert_eq!(song["originalBody"], "Song: https://vocadb.net/S/100");
    assert_eq!(song["createdDate"], "2022-02-25T14:29:00Z");
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
    let session = app.session().await;

    for (offset, limit) in [(-1, 10), (0, -1), (0, 101)] {
        let response = app
            .get(
                &format!("/api/notifications?offset={offset}&limit={limit}&language=Default"),
                Some(&session),
            )
            .await;
        assert_eq!(response.status, StatusCode::BAD_REQUEST, "{offset} {limit}");
        assert_eq!(response.body["code"], 400);
    }
}

#[tokio::test]
async fn delete_notifications() {
    let app = TestApp::new().await;
    Mock::given(method("DELETE"))
        .and(path(format!("/api/users/{USER_ID}/messages")))
        .and(query_param("messageId", "1"))
        .and(query_param("messageId", "2"))
        .and(header_eq("cookie", VOCADB_COOKIE))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&app.vocadb)
        .await;

    // Deleted messages are evicted from the cache.
    let deleted = Cache::message_key(Database::VocaDb, USER_ID, 1);
    let kept = Cache::message_key(Database::VocaDb, USER_ID, 3);
    app.kv.set(&deleted, "{}", MESSAGE_TTL).await.unwrap();
    app.kv.set(&kept, "{}", MESSAGE_TTL).await.unwrap();

    let response = app
        .json(
            Method::DELETE,
            "/api/notifications",
            Some(&app.session().await),
            json!({ "ids": [1, 2] }),
        )
        .await;

    assert_eq!(
        (response.status, response.body),
        (StatusCode::OK, Value::Null)
    );
    assert_eq!(app.kv.get(&deleted).await.unwrap(), None);
    assert!(app.kv.get(&kept).await.unwrap().is_some());
}

#[tokio::test]
async fn cors_preflight_allows_configured_origins() {
    use vocadb_notification_reader::config::{Config, default_database_urls};
    use vocadb_notification_reader::web::app;

    let app = app(Config {
        listen_addr: "127.0.0.1:0".parse().unwrap(),
        redis_url: None,
        cors_allowed_origins: vec![FRONTEND.to_string()],
        database_urls: default_database_urls(),
    })
    .await
    .unwrap();

    let preflight = |origin: &str| {
        Request::options("/api/notifications")
            .header(header::ORIGIN, origin)
            .header(header::ACCESS_CONTROL_REQUEST_METHOD, "DELETE")
            .header(header::ACCESS_CONTROL_REQUEST_HEADERS, "content-type")
            .body(Body::empty())
            .unwrap()
    };

    let response = app.clone().oneshot(preflight(FRONTEND)).await.unwrap();
    let headers = response.headers();
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_ORIGIN], FRONTEND);
    assert_eq!(headers[header::ACCESS_CONTROL_ALLOW_CREDENTIALS], "true");
    assert!(
        headers[header::ACCESS_CONTROL_ALLOW_METHODS]
            .to_str()
            .unwrap()
            .contains("DELETE")
    );

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
