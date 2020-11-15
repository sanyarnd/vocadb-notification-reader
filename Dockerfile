# syntax=docker/dockerfile:experimental
FROM maven:3-jdk-14 as backend-builder
WORKDIR /build

COPY pom.xml pom.xml
RUN --mount=type=cache,target=/root/.m2 mvn de.qaware.maven:go-offline-maven-plugin:resolve-dependencies

COPY spotbugs-exclude.xml spotbugs-exclude.xml
COPY lombok.config lombok.config
COPY checkstyle.xml checkstyle.xml

COPY src/main/resources src/main/resources
COPY src/main/java src/main/java

RUN --mount=type=cache,target=/root/.m2 mvn package \
    --batch-mode --errors --fail-at-end --show-version -Dorg.slf4j.simpleLogger.showDateTime=true
RUN mkdir exploded-jar && cd exploded-jar && java -Djarmode=layertools -jar ../target/notification-reader.jar extract


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


FROM debian:buster
WORKDIR /application
ENV JAVA_OPTS=""

RUN apt update && apt install software-properties-common gnupg wget gettext-base -y && \
    wget -qO - https://adoptopenjdk.jfrog.io/adoptopenjdk/api/gpg/key/public | apt-key add - && \
    add-apt-repository --yes https://adoptopenjdk.jfrog.io/adoptopenjdk/deb/ && \
    apt update && apt-get install adoptopenjdk-15-hotspot nginx nginx-extras -y && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder build/exploded-jar/dependencies/ ./
COPY --from=backend-builder build/exploded-jar/snapshot-dependencies/ ./
COPY --from=backend-builder build/exploded-jar/spring-boot-loader/ ./
COPY --from=backend-builder build/exploded-jar/application/ ./

COPY src/main/nginx/nginx-heroku.conf nginx.template
COPY src/main/nginx/mime.types /etc/nginx/conf/mime.types
COPY --from=frontend-builder /build/target/spa /var/www/application

ENV PORT=8081
EXPOSE $PORT

CMD (nohup java -Duser.timezone=UTC -Dfile.encoding=UTF-8 $JAVA_OPTS org.springframework.boot.loader.JarLauncher &) && \
    (envsubst '${PORT}' < nginx.template > /etc/nginx/nginx.conf) && \
    nginx -g 'daemon off;'
