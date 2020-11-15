# syntax=docker/dockerfile:experimental
FROM oracle/graalvm-ce:20.2.0-java11 as backend-builder
WORKDIR /build

RUN gu install native-image

COPY .mvn .mvn
COPY mvnw mvnw
RUN chmod u+x mvnw
COPY pom.xml pom.xml
RUN --mount=type=cache,target=/root/.m2 ./mvnw de.qaware.maven:go-offline-maven-plugin:resolve-dependencies

COPY spotbugs-exclude.xml spotbugs-exclude.xml
COPY lombok.config lombok.config
COPY checkstyle.xml checkstyle.xml

COPY src/main/resources src/main/resources
COPY src/main/java src/main/java

RUN --mount=type=cache,target=/root/.m2 ./mvnw package \
    --batch-mode --errors --fail-at-end --show-version -Dorg.slf4j.simpleLogger.showDateTime=true

RUN mkdir -p target/native-image && \
    cd target/native-image && \
    jar -xvf ../notification-reader.jar >/dev/null 2>&1 && \
    cp -R META-INF BOOT-INF/classes && \
    LIBPATH=`find BOOT-INF/lib | tr '\n' ':'` && \
    CP=BOOT-INF/classes:$LIBPATH && \
    native-image \
      -J-Xmx4G \
      -H:+TraceClassInitialization \
      -H:Name=notification-reader \
      -H:+ReportExceptionStackTraces \
      -Dspring.spel.ignore=true \
      -Dspring.native.remove-unused-autoconfig=true \
      -Dspring.native.remove-yaml-support=true \
#      -Dspring.native.verbose=true \
#      --initialize-at-build-time \
      --enable-all-security-services \
      -cp $CP vocadb.notification.reader.Application


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

FROM debian:buster-slim
WORKDIR /application

RUN apt update && apt-get install gettext-base nginx nginx-extras -y && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /build/target/native-image/notification-reader notification-reader

COPY src/main/nginx/nginx-heroku.conf nginx.template
COPY src/main/nginx/mime.types /etc/nginx/conf/mime.types
COPY --from=frontend-builder /build/target/spa /var/www/application

ENV PORT=8081
EXPOSE $PORT

CMD ["/bin/bash", "-c", "envsubst '${PORT}' < nginx.template > /etc/nginx/nginx.conf && (nginx -g 'daemon off;' &) && ./notification-reader"]
