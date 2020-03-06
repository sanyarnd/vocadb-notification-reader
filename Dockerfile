# syntax=docker/dockerfile:experimental
FROM maven:3.6.3-jdk-14 as builder
WORKDIR build

COPY . .

RUN --mount=type=cache,target=/root/.m2 \
    mvn package -P prod

RUN mkdir extracted-app
RUN cd extracted-app && java -Djarmode=layertools -jar ../target/notification.reader.jar extract

# layered final image
FROM adoptopenjdk:14-jre-hotspot
WORKDIR application

COPY --from=builder build/extracted-app/dependencies/ .
COPY --from=builder build/extracted-app/snapshot-dependencies/ .
COPY --from=builder build/extracted-app/resources/ .
COPY --from=builder build/extracted-app/application/ .
ENTRYPOINT ["java", "org.springframework.boot.loader.JarLauncher"]
