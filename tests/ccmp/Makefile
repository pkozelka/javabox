PROJECT_DIR=$(shell cd ../..; pwd)
MVN386=.m2/wrapper/dists/apache-maven-3.8.6-bin/67568434/apache-maven-3.8.6-bin.zip

D_USER=$(shell id -u)
D_GROUP=$(shell id -g)

D_HOME=/home/builder
DOCKER_RUN=docker run --init -it --rm --network host \
	-u $(D_USER):$(D_GROUP) \
	-v ~/Downloads/cache:$(D_HOME)/Downloads/cache \
	-v $(PROJECT_DIR)/launchers/mvnw:$(D_HOME)/code/mvnw \
	--mount type=bind,source=/tmp/mvn-repository,target=$(D_HOME)/.m2/repository \
	test-ccmp

#	-v /tmp/mvn-repository:$(D_HOME)/.m2/repository:rw
#	-v $(PROJECT_DIR)/target/debug/javabox:/usr/local/bin/javabox
#	-u $(D_USER):$(D_GROUP)

all:

build:
	docker build --network host . -t test-ccmp

.PHONY:

init-docker:
	mkdir -p /tmp/mvn-repository

sh: init-docker
	$(DOCKER_RUN) bash

sh-ls: init-docker
	$(DOCKER_RUN) ls -la /home/builder/.m2 /home/builder/code/mvnw /home/builder/Downloads

run: init-docker
	$(DOCKER_RUN) ./mvnw package
