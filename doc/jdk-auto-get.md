# Automatic JDK download

## Oracle JDK

Archives:
- Archive page: https://www.oracle.com/java/technologies/downloads/archive/
- [Java SE 19](https://www.oracle.com/java/technologies/javase/jdk19-archive-downloads.html)
- [Java SE 18](https://www.oracle.com/java/technologies/javase/jdk18-archive-downloads.html)
- [Java SE  9](https://www.oracle.com/java/technologies/javase/javase9-archive-downloads.html)
- [Java SE 8 >= 211](https://www.oracle.com/java/technologies/javase/javase8u211-later-archive-downloads.html), 
  [Java SE 8 < 211](https://www.oracle.com/java/technologies/javase/javase8-archive-downloads.html)
- [Java SE 5](https://www.oracle.com/java/technologies/java-archive-javase5-downloads.html)

### JDK Script-friendly URLs

Documented here: https://www.oracle.com/java/technologies/jdk-script-friendly-urls/

Checksum: add `.sha256` to the URL

Latest versions:
- `linux-aarch64`: https://download.oracle.com/java/19/latest/jdk-19_linux-aarch64_bin.tar.gz
- `linux-x64`: https://download.oracle.com/java/19/latest/jdk-19_linux-x64_bin.tar.gz
- `windows-x64`: https://download.oracle.com/java/19/latest/jdk-19_windows-x64_bin.zip
- `macos-aarch64`: https://download.oracle.com/java/19/latest/jdk-19_macos-aarch64_bin.tar.gz
- `macos-x64`: https://download.oracle.com/java/19/latest/jdk-19_macos-x64_bin.tar.gz

Explicit versions:
- `https://download.oracle.com/java/19/archive/jdk-19.0.1_linux-x64_bin.tar.gz`
- `https://download.oracle.com/java/17/archive/jdk-17.0.1_linux-x64_bin.tar.gz`
                  

## OpenJDK

How does Jenkins do it?
* has plugin named [Tool Auto Installation](https://wiki.jenkins.io/display/JENKINS/Tool+Auto-Installation)
* Jenkins data for tools: [list of JSON files](https://mirrors.jenkins-ci.org/updates/updates/)
* [JSON file for JDK](https://mirror.gruenehoelle.nl/jenkins/updates/updates/hudson.tools.JDKInstaller.json)
