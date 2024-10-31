FROM openjdk:21-jdk
COPY starplex/build/libs/starplex-1.0.0-SNAPSHOT.jar application.jar
COPY starplex/src/main/resources/application.properties application.properties
EXPOSE 61000
ENTRYPOINT ["java","-jar","application.jar"]