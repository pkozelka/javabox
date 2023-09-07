# Note: JDK download from Adoptium

* Web: https://adoptium.net/temurin/releases/
* API: https://api.adoptium.net/
  * swagger: https://api.adoptium.net/q/swagger-ui/


## API endpoints

### Available releases:

https://api.adoptium.net/q/swagger-ui/#/Release%20Info/getAvailableReleases

```
$ curl -v https://api.adoptium.net/v3/info/available_releases
< HTTP/1.1 200 OK

{
    "available_lts_releases": [
        8,
        11,
        17
    ],
    "available_releases": [
        8,
        11,
        16,
        17,
        18,
        19,
        20
    ],
    "most_recent_feature_release": 20,
    "most_recent_feature_version": 21,
    "most_recent_lts": 17,
    "tip_version": 22
}
```

### Release versions:

https://api.adoptium.net/q/swagger-ui/#/Release%20Info/getReleaseVersions

```
$ curl -v 'https://api.adoptium.net/v3/info/release_versions?architecture=x64&OS=linux&version=\[11,18\]'

{
    "versions": [
        {
            "build": 5,
            "major": 17,
            "minor": 0,
            "openjdk_version": "17.0.9-beta+5-202309031424",
            "optional": "202309031424",
            "pre": "beta",
            "security": 9,
            "semver": "17.0.9-beta+5.0.202309031424"
        },
        {
            "build": 2,
            "major": 17,
            "minor": 0,
            "openjdk_version": "17.0.9-beta+2-202308110040",
            "optional": "202308110040",
            "pre": "beta",
            "security": 9,
            "semver": "17.0.9-beta+2.0.202308110040"
        }
    ]
}
```

### Redirect to binary

https://api.adoptium.net/q/swagger-ui/#/Binary/getBinaryByVersion

```
$ curl -v 'https://api.adoptium.net/v3/binary/latest/17/ga/linux/x64/jdk/hotspot/normal/eclipse'
< HTTP/1.1 307 Temporary Redirect
< Location: https://github.com/adoptium/temurin17-binaries/releases/download/jdk-17.0.8.1%2B1/OpenJDK17U-jdk_x64_linux_hotspot_17.0.8.1_1.tar.gz
```


### Checksum, other metadata

curl 'https://api.adoptium.net/v3/assets/feature_releases/8/ga?os=linux&architecture=x64' > adopt.json
