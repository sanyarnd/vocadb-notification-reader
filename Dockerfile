# syntax=docker/dockerfile:experimental
#FROM node:12-lts AS frontend-builder
#WORKDIR build
#COPY yarn.lock yarn.lock
#COPY package.json package.json
#RUN yarn --frozen-lockfile
#
#COPY public public
#COPY .browserslistrc .browserslistrc
#COPY .editorconfig .editorconfig
#COPY .eslintrc.js .eslintrc.js
#COPY tsconfig.json tsconfig.json
#COPY vue.config.js vue.config.js
#COPY src/main/typescript src/main/typescript
#
#RUN yarn run build:prod

FROM maven:3-jdk-11 as backend-builder
WORKDIR build
ARG VERSION="0.0.0-local"

COPY pom.xml pom.xml
RUN --mount=type=cache,target=/root/.m2 mvn de.qaware.maven:go-offline-maven-plugin:resolve-dependencies

COPY checkstyle.xml checkstyle.xml
COPY spotbugs-exclude.xml spotbugs-exclude.xml
COPY lombok.config lombok.config
RUN --mount=type=cache,target=/root/.m2 mvn versions:set -DnewVersion="$VERSION"

COPY src/main/java src/main/java
COPY src/main/resources src/main/resources
RUN --mount=type=cache,target=/root/.m2 mvn package

WORKDIR exploded-jar
RUN java -Djarmode=layertools -jar target/notification-reader.jar extract
WORKDIR build

# run backend with reverse proxy
#FROM debian:buster
#WORKDIR application
#
#RUN apt update && apt install software-properties-common gnupg wget gettext-base -y && \
#    wget -qO - https://adoptopenjdk.jfrog.io/adoptopenjdk/api/gpg/key/public | apt-key add - && \
#    add-apt-repository --yes https://adoptopenjdk.jfrog.io/adoptopenjdk/deb/ && \
#    apt update && apt-get install adoptopenjdk-11-hotspot nginx nginx-extras -y && \
#    apt-get clean && rm -rf /var/lib/apt/lists/*
#
#COPY --from=backend-builder /build/dependencies/ ./
#COPY --from=backend-builder /build/snapshot-dependencies/ ./
#COPY --from=backend-builder /build/spring-boot-loader/ ./
#COPY --from=backend-builder /build/application/ ./
#
#COPY --from=frontend-builder /build/src/main/nginx/nginx-heroku.conf nginx.template
#COPY --from=frontend-builder /build/src/main/nginx/mime.types /etc/nginx/conf/mime.types
#COPY --from=frontend-builder /build/target/spa /var/www/application
#
#ENV PORT=8081
#
#EXPOSE $PORT
#CMD (nohup java org.springframework.boot.loader.JarLauncher &)&& \
#    (envsubst '${PORT}' < nginx.template > /etc/nginx/nginx.conf) && \
#    nginx -g 'daemon off;'
