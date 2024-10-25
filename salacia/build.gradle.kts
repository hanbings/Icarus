plugins {
    id("java")
    @Suppress("SpellCheckingInspection")
    id("com.github.johnrengelman.shadow") version "8.1.1"
}

@Suppress("SpellCheckingInspection")
group = "io.hanbings.server"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    // https://mvnrepository.com/artifact/org.kohsuke/github-api
    implementation("org.kohsuke:github-api:2.0.0-alpha-2")
    // https://mvnrepository.com/artifact/com.squareup.okhttp3/okhttp
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("org.mongodb:mongodb-driver-sync:5.2.0")
    // https://mvnrepository.com/artifact/com.google.code.gson/gson
    implementation("com.google.code.gson:gson:2.11.0")

    implementation("org.tinylog:tinylog-api:2.7.0")
    implementation("org.tinylog:tinylog-impl:2.7.0")
    // https://mvnrepository.com/artifact/org.tinylog/slf4j-tinylog
    implementation("org.tinylog:slf4j-tinylog:2.7.0")
    // https://mvnrepository.com/artifact/org.slf4j/slf4j-api
    implementation("org.slf4j:slf4j-api:2.0.16")

    compileOnly("org.projectlombok:lombok:1.18.34")
    annotationProcessor("org.projectlombok:lombok:1.18.34")

    testImplementation(platform("org.junit:junit-bom:5.10.0"))
    testImplementation("org.junit.jupiter:junit-jupiter")
}

tasks.test {
    useJUnitPlatform()
}