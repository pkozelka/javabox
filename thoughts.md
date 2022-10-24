# Notes

CLAP: see multicall example: https://github.com/clap-rs/clap/blob/master/examples/multicall-busybox.rs

## Download directories

* download base is directed to user's download dir
* similar structure will be there as in legacy caches:
  ```
  <TOOLNAME>/
    <LATEST_FILE>
    latest-url.txt
    <ARCHIVE_BASENAME>-<URL_HASH>/
      <ARCHIVE> (.zip/.tgz)
      <CRC files> (.md5, .sha1, .sha256, .asc ...)
      url.txt
      <symlink to installation dir>
  ```

## Installation dir

?? Candidates:
- `~/.java/<TOOLNAME>` _(also default for others)_
- `~/.m2` | `~/.gradle` | ...
- `~/opt` | `/opt`

## Linking tools to javabox

Linking options:
- native symlink, using FS capabilities
  - ux: `ln -s ...`
  - win: `mklink` (requires special setup)
- launcher script
  - ux: `sh` script ending with `exec` handover
  - win: `.bat` | `.cmd`
- ?? user can choose or configure
- ?? default: symlink on ux, `.bat` on win
Links will be placed in a directory on PATH list, perhaps `~/bin`.

## Configuration file

- file: `config.properties` | `<tool>.properties`
- ?? maybe TOML format ??
- location: something standard, like ux: `~/.config/javabox/`, win: ...

Configurable options:

...global
- ??linking strategy??
- default clone directory
  - per well-known repos
- configure custom well-known repos
- enable initial build
- new version check min. interval

... per tool
- java: default vendor
- java: default major version per vendor
- java: default version per each major
- default tool version if latest is not to be checked
- enforce version (overrides any defaults, meant for temporary use)
- skip/enforce hash checking
- specific version aliases
- new version check min. interval

## Wrapper extra options

Normally, javabox passes the commandline args to the target tool as-is.
Some explicit modifications are however possible, by including options that are specially prefixed/postfixed.

- ?? The prefix is `@@` and postfix is `::'.

Use-cases:
- override JDK major version: `@@jdk=11::`
- override JDK exact version: `@@jdk=11.0.b2::`
- override JDK vendor:        `@@jdk:graal::`
- override tool version:      `@@mvn=3.2.1::`

User places these into the command-line.
Javabox removes them prior passing to the tool.

Example:

```shell
mvn @@jdk=19:: @@mvn=3.2.1:: clean build
```

## Usage scenarios

These scenarios will be presented in file `BUILD.md` so that the user can see it on
web repo.
Part of that file may also serve as a wrapper configuration.

User wants to build a project

(WX) ... **from web repo, and has completely empty box**
- ?? user must install git/svn/hg/* himself ??
- runs command suggested from readme; kind of `curl something.url?github=x/y | sh"`
  - if not present, javabox is downloaded and installed
    - javabox setup is executed: symlinks, default configs etc
  - javabox executes initial sequence
    `javabox init --url=https://x.y.z/a/b/c --dir=/code/dir`
    - ??maybe install scm??
    - ??maybe use embedded git/scm??
    - project is cloned to code directory
    - wrapper properties are examined
    - JDK is located or downloaded+installed
    - Maven is located or downloaded+installed
    - initial build or preload is performed
    - little timing and action report is presented on console

(DX) ... **that resides on disk, but he has no javabox yet**
- runs `./mvnw clean build`
  - if not present, javabox is downloaded and installed
      - javabox setup is executed: symlinks, default configs etc
  - javabox passes command to maven
    `javabox mvn $*`
      - wrapper properties are examined
      - ... etc.

(DJ) ... **that resides on disk, and already has javabox**
- runs `mvn clean install`

(WJ) ... **from web repo and has javabox installed**
- runs `javabox init --scm=git:https://x.y.z/a/b/c --under=bindings/java .`
  - `--scm` is the scm url, starts with the scm type prefix (`git`, `svn`, ... see how Maven defines this)
  - `--under` is optional, indicates that the project resides somewhere else than on the root
  - the last param is target dir (here dot), indicates the desire to install it here (must be empty) or indicated dir (must be empty or missing) rather than in auto-determined directory (the default)

### ??phased approach

- install and setup javabox first if missing
  - one day this will be part of the distro
- adjust javabox settings
- init project from web or run via wrapper

## Misc

### ??running from ./mvnw or ./gradlew

javabox checks if setup was performed, and suggests it if not

### Support for adding to repo

- creates `BUILD.md`
- adds wrapper launchers and configuration to repo
- adds it to git
