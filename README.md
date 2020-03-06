# VocaDB Notification Reader

App URL: https://unofficial-vocadb-ntf-reader.herokuapp.com/

# Requirements
Required:
* Maven 3.6.1+
* JDK 11+

Optional:
* Docker
* Heroku CLI

# Localhost build
```shell script
mvn -P prod clean package
```
You'll find the final `notification.reader.jar` file in `target/` directory.

Now you can run application on localhost:
```shell script
<path-to-java-executable> -Dfile.encoding=UTF-8 -Xms64m -Xmx256m -jar notification.reader.jar
```

Alternatively, you can build and use docker image: 
```shell script
set DOCKER_BUILDKIT=1 # or export DOCKER_BUILDKIT=1
docker build -t registry.heroku.com/unofficial-vocadb-ntf-reader/web .
```
and run application as docker container:
```shell script
docker run --rm -p 8080:8080 registry.heroku.com/unofficial-vocadb-ntf-reader/web
```

# Publish to Heroku
You need to login in [`Heroku CLI`](https://devcenter.heroku.com/articles/container-registry-and-runtime#logging-in-to-the-registry).

Build and push an image:
```shell script
set DOCKER_BUILDKIT=1 # or export DOCKER_BUILDKIT=1
docker build -t registry.heroku.com/unofficial-vocadb-ntf-reader/web .
docker push registry.heroku.com/unofficial-vocadb-ntf-reader/web
```

Deploy application:
```shell script
heroku container:release web --app=<your app name>
```
