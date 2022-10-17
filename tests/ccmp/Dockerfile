FROM ubuntu:latest
MAINTAINER pkozelka@gmail.com

RUN apt update && apt install -y git

#TODO let's get rid of the need for this, ASAP:
RUN apt install -y openjdk-8-jdk-headless

#RUN mkdir -p /home/builder
#RUN useradd -r -g users -d /home/builder builder && chown builder:users /home/builder
RUN adduser builder

RUN ln -s javabox /usr/local/bin/mvnw


ENV HOME=/root
#USER builder:users
RUN git clone --depth=2 https://github.com/pkozelka/contentcheck-maven-plugin.git /root/code
RUN mkdir -p /root/.m2/repository
#VOLUME /home/builder/.m2/repository


RUN mkdir -p /root/.m2/wrapper/dists .m2/repository
RUN ln -s /root/Downloads/cache/apache-maven-3.8.6-bin /root/.m2/wrapper/dists/apache-maven-3.8.6-bin
RUN alias l='ls -la'

WORKDIR /root/code
