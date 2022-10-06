# Maven Wrapper

Activated when the user calls
- `mvn`
- `mvnw`
- `./mvnw` and the script is our wrapper, ie. pointing to us

## Important files

### `.mvn/`

Directory with various maven-related configs. Useful as indicator of the project's top.

### `.mvn/wrapper/maven-wrapper.jar`

Java code to download Maven distro, install it, and pass arguments to it.
Its source code is at https://github.com/apache/maven-wrapper/

Migrator will remove this file.

### `.mvn/wrapper/maven-wrapper.properties`

Sample content:
```
distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.8.6/apache-maven-3.8.6-bin.zip
wrapperUrl=https://repo.maven.apache.org/maven2/org/apache/maven/wrapper/maven-wrapper/3.1.1/maven-wrapper-3.1.1.jar
```

We can use this to determine exact Maven version, and the fact that this is an Apache wrapper (and its version).

Migrator will remove property `wrapperUrl`.

### `.mvn/jvm.config`

JVM arguments to pass to java when invoking Maven.

Not very useful for us.

Sample content:
```
-Dfile.encoding=UTF-8
```

### `.mvn/maven.config`

CLI arguments added to every mvn commandline.

Not very useful for us.

Sample content:
```
--show-version --errors
```

### `pom.xml` - the project descriptor

Can be used to:
- estimate Java major version
  - `/project/properties/{source,target}`
  - `/project/build/plugins[artifactId='maven-compiler-plugin']/configuration/{source,target,compilerVersion}`
- determine modules and their structure, including project top
  - `/project/parent/relativePath`
  - `/project/modules/module`
- module identification
  - `/project/{groupId,artifactId,version}`
  - `/project/parent/{groupId,artifactId,version}`
- type of primary output artifact
  - `/project/packaging`
- name of primary output artifact
  - `/project/build/finalName`
- dependencies and other build pre-requisities
  - `/project/dependencies`
  - `/project/plugins`
  - `/reporting/plugins`
  - `/project/dependenctManagement/dependencies` (?)
