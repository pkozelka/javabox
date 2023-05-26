# Goals

Various tracks of project ambitions are listed here.

## Goal: full support of linux

Steps

- [ ] move most functionality from mvnw script to javabox code in Rust (chmod, PATH + .bashrc)
- [ ] re-think symlink vs. hardlink vs. minilaunchers
- [ ] download a default JDK (perhaps OpenJDK 1.8? or latest?)
- [ ] JDK selection by properties
- [ ] ?JDK version selection on commandline
- [ ] ?maven/gradle/ant version selection on commandline
- [ ] add support for JAVABOX_LOG configuration

## Goal: support for all platforms

Steps

- [ ] setup cross-compilation for windows and mac
- [ ] prepare traits to enforce complete support for each platform
- [ ] adjust windows launcher script
- [ ] add install/uninstall variant for windows
- [ ] upload distro for windows
- [ ] make sure that linux launcher works for mac as well
- [ ] add install/uninstall variant for mac
- [ ] upload distro for mac

## Goal: prepare packages for major package management platforms

- [ ] prepare system-install for linux (installed by root, executed by user)
- [ ] Ubuntu ppt
- [ ] RHEL
- [ ] Gentoo
- [ ] Windows Chocolatey
- [ ] Mac: BREW
