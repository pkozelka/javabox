# Development toolbox

This tool tries to improve developer's comfort by automating some everyday stuff, like installing various
versions of common tools, invoking them etc.

Some inspiration for this tool comes from the design of the excellent [BusyBox project](https://www.busybox.net/).

Similarly, this tries to:
- be as small and fast as possible
- independent of OS configuration
- have `box` in its name


Currently, the primary target is java development. 

## Goals

- Automatically download and install JDK, Maven, Gradle suitable for given project
- Work as typical wrapper, without need to explicitly specify wrapper scripts (`./gradlew`,`./mvnw`) and use original name instead (`gradle`, `mvn`) without path
- Always select the right version 
- Reuse cached tool installations already used by legacy wrappers, and _honor their per-project configuration_
- Get out of the way once the tool runs
- Be small, and easily configurable as a wrapper

## Non-Goals

- Auto-install everything, like IDE and other tools
  - when coming to a new computer, only support fast first setup and build; choice of GUI etc is up tp the developer
- Send any telemetry anywhere

## Features

**Auto-init**

- provide a script for initial automatic setup everything:
  - install itself (if not present),
  - install git (if missing, and using system tools),
  - clone the project repository (to a well-known location), 
  - then download and install the build tools (to their typical locations), 
  - then tries to build/test the project
- then the developer continues using `javabox`, directly or via its aliases

**Aliases**

`javabox` aliases multiple names to itself (just like BusyBox does) in order to keep the binary single and altogether small.

Depending on purpose and OS capabilities, different aliasing approaches can be taken:
- symlink
- delegating script
- hardlink

**JDK specification**

Current wrappers for Maven and Gradle do not specify a way to determine java version, because they operate on the 
one already installed. For that, a new format is invented and proposed here.

Something like this:

- file name is `jdk.version`
- allows to specify either just major version (like `1.8`) or concrete version (like `1.8.0_341`)
- in case of major, the latest available one should be used
- allows to specify JDK distro: `OracleJDK`, `OpenJDK` or others
- all fields are optional, reasonable defaults should work (like, OpenJDK 11?)

Example 1:
```
VENDOR=OpenJDK
MAJOR=11
```

Example 2:
```
VENDOR=OracleJDK
VERSION=1.8.0_341
```

## Usage

### Fresh new installed box

Imagine your computer burns, and you need to get another one and start working on your project as quickly as possible.
Or you want to engage a new developer but he doesn't 
On a fresh new computer, you will 
