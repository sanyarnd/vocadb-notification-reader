# VocaDB Notification Reader
[![Backend](https://github.com/sanyarnd/vocadb-notification-reader/actions/workflows/backend.yml/badge.svg)](https://github.com/sanyarnd/vocadb-notification-reader/actions/workflows/backend.yml)
[![Frontend](https://github.com/sanyarnd/vocadb-notification-reader/actions/workflows/frontend.yml/badge.svg)](https://github.com/sanyarnd/vocadb-notification-reader/actions/workflows/frontend.yml)

Unofficial notification reader for:
* https://vocadb.net
* https://touhoudb.com
* https://utaitedb.net

## Structure

* `backend` — Rust API ([axum](https://github.com/tokio-rs/axum) + [reqwest](https://github.com/seanmonstar/reqwest)),
  shipped as a container image `ghcr.io/sanyarnd/vocadb-notification-reader`.
* `frontend` — static SPA (Vue 3, Vuetify, Pinia, Vite), can be served from any static hosting / S3 / CDN.

## Backend

```shell
cd backend
cargo run          # http://localhost:8080
cargo test         # set TEST_REDIS_URL=redis://localhost:6379 to also test against Valkey/Redis
cargo clippy --all-targets
```

Configuration is done through environment variables:

| Variable               | Default        | Description                                                                                       |
|------------------------|----------------|---------------------------------------------------------------------------------------------------|
| `LISTEN_ADDR`          | `0.0.0.0:8080` | Address to listen on                                                                              |
| `REDIS_URL`            | —              | Valkey/Redis for sessions and cache, e.g. `redis://valkey:6379/0`. Required for production       |
| `CORS_ALLOWED_ORIGINS` | —              | Comma separated list of origins allowed to call the API (e.g. the frontend CDN origin)           |
| `TRUSTED_PROXIES`      | loopback       | Comma separated networks of reverse proxies whose `X-Forwarded-For` is trusted                   |
| `RUST_LOG`             | `info`         | Log filter                                                                                        |

Sessions live on the server: after login the client receives an opaque random token,
while the VocaDB session cookies are kept in Valkey/Redis (stored under a SHA-256 of the token).
A session lasts as long as VocaDB keeps the user signed in: it follows the expiration of the
VocaDB auth cookie, picks up cookies VocaDB refreshes, and ends as soon as VocaDB rejects them
(or on `POST /api/logout`). Sessions whose cookie has no expiration are dropped after a year
without use.

VocaDB responses are cached in the same storage: messages for 30 days, songs for an hour
(per language), which keeps repeated page loads from hitting VocaDB.

Without `REDIS_URL` sessions and cache are kept in process memory, which is only suitable
for development. Sessions are long-lived, so enable persistence (AOF) for Valkey.

```yaml
services:
  backend:
    image: ghcr.io/sanyarnd/vocadb-notification-reader:latest
    environment:
      REDIS_URL: redis://valkey:6379/0
      CORS_ALLOWED_ORIGINS: https://vocadb-notification-reader.example.com
    ports:
      - "8080:8080"
    depends_on: [valkey]

  valkey:
    image: valkey/valkey:8-alpine
    command: ["valkey-server", "--appendonly", "yes"]
    volumes:
      - valkey:/data

volumes:
  valkey:
```

### Rate limits and client addresses

Limits are counted in Valkey/Redis, so they hold across restarts and replicas, and answered
with `429` and `Retry-After`:

| What                               | Limit             |
|------------------------------------|-------------------|
| Any API request from one address   | 600 per minute    |
| Login attempts from one address    | 20 per 15 minutes |
| Login attempts to one account      | 10 per 15 minutes |
| Requests within one session        | 120 per minute    |

The client address is taken from `X-Forwarded-For` only when the request comes from
`TRUSTED_PROXIES` (loopback by default), so it can't be spoofed. When the reverse proxy runs on
the host network (e.g. Caddy with `--network host`), run the backend on the host network too
(`LISTEN_ADDR=127.0.0.1:8080`); with a published port Docker forwards connections from its bridge
gateway, which then has to be added to `TRUSTED_PROXIES`.

Repeated failed logins and rate limit abuse can be banned with CrowdSec, see
[deploy/crowdsec](deploy/crowdsec/README.md).

### API

| Method   | Path                                                  | Description                                    |
|----------|-------------------------------------------------------|------------------------------------------------|
| `POST`   | `/api/session`                                        | Log in, sets the `__Host-session` cookie       |
| `DELETE` | `/api/session`                                        | Log out                                        |
| `GET`    | `/api/me`                                             | Current account, extends the cookie            |
| `GET`    | `/api/notifications?offset=0&limit=25&language=Default` | Page of notifications                        |
| `DELETE` | `/api/notifications`                                  | Delete notifications, body: `{ "ids": [...] }` |
| `GET`    | `/health`                                             | Liveness probe                                 |

The session cookie is `HttpOnly`, `Secure` and `SameSite=Strict`, so the frontend and the API must be
served from the same site (e.g. `foobar.com` and `api.foobar.com`). State changing requests are only
accepted from the API's own origin or `CORS_ALLOWED_ORIGINS`.

TypeScript types of the API are generated from the Rust types into `frontend/src/api/generated`
by `cargo test`; CI fails when they are out of date.

## Frontend

```shell
cd frontend
npm ci
npm run dev        # http://localhost:5173, proxies /api to localhost:8080
npm test
npm run lint
npm run build      # produces dist/
```

The backend URL is baked in at build time through `VITE_API_URL`
(requests go to the same origin when it's empty):

```shell
VITE_API_URL=https://api.vocadb-notification-reader.example.com npm run build
```

The bundle uses relative asset paths and hash based routing, so `dist/` can be uploaded
as is to a bucket or CDN, no rewrite rules required.

## CI

* Pull requests and pushes to `master` run linters and tests for both parts.
* Pushes to `master` publish the backend image to GHCR and upload the frontend `dist` as a workflow artifact.
  The frontend build reads the backend URL from the `API_URL` repository variable.
