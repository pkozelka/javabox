PROJECT_DIR=$(shell cd ../..; pwd)
MVN386=.m2/wrapper/dists/apache-maven-3.8.6-bin/67568434/apache-maven-3.8.6-bin.zip

DOCKER_RUN=docker run --init -it --rm --network host \
	-v ~/Downloads/cache:/root/Downloads/cache \
	-v $(PROJECT_DIR)/launchers/mvnw:/root/code/mvnw \
	-v /tmp/mvn-repository:/root/.m2/repository:rw \
	test-ccmp

#	-v $(PROJECT_DIR)/target/debug/javabox:/usr/local/bin/javabox
all:

build:
	docker build --network host . -t test-ccmp

.PHONY: sh
sh:
	$(DOCKER_RUN) bash

run:
	$(DOCKER_RUN) ./mvnw package
