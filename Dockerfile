# syntax=docker/dockerfile:experimental
FROM rust:buster AS backend-planner
WORKDIR /build

RUN cargo install cargo-chef
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src/main/rust src/main/rust
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:buster AS backend-cache
WORKDIR /build

RUN cargo install cargo-chef
COPY --from=backend-planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:buster AS backend-builder
WORKDIR /build

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src/main/rust src/main/rust

COPY --from=backend-cache /build/target target
COPY --from=backend-cache /usr/local/cargo /usr/local/cargo

RUN cargo build --release

FROM node:lts-buster AS frontend-builder
WORKDIR /build
COPY yarn.lock yarn.lock
COPY package.json package.json
RUN yarn --frozen-lockfile

COPY public public
COPY .browserslistrc .browserslistrc
COPY .eslintrc.js .eslintrc.js
COPY tsconfig.json tsconfig.json
COPY vue.config.js vue.config.js
COPY src/main/typescript src/main/typescript

RUN yarn run build:prod

FROM nginx:stable AS runtime
WORKDIR /application

COPY src/main/nginx/nginx-heroku.conf nginx.template
COPY src/main/nginx/mime.types /etc/nginx/conf/mime.types
COPY --from=frontend-builder /build/target/spa /var/www/application
COPY --from=backend-builder /build/target/release/vocadb-notification-reader vocadb-notification-reader

ENV PORT=8081
EXPOSE $PORT

CMD (nohup ./vocadb-notification-reader &) && \
    (envsubst '${PORT}' < nginx.template > /etc/nginx/nginx.conf) && \
    nginx -g 'daemon off;'
