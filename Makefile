R_VERSION=shapshot

all:

mklinks:
	mkdir -p target
	ln -s debug/examples/multicall-derive target/true
	ln -s debug/examples/multicall-derive target/false

install:
	cargo install --path .

install-debug:
	ln -sf $(PWD)/target/debug/javabox $(shell which javabox)

init-dist:
	echo $(R_VERSION)
	rm -rf "target/dist"
	mkdir -p "target/dist"
	cp -t "target/dist" "launchers/mvnw"

dist-release: init-dist
	cargo build --release && cp -t "target/dist" "target/release/javabox"

dist-debug:
	cargo build && cp -t "target/dist" "target/debug/javabox"

upload:
	rsync -e "ssh -o StrictHostKeyChecking=no" -rlpcgoD -zi --delete target/dist/ origis_info@www.origis.info:dist.origis.info/javabox/$(R_VERSION)

up-sh:
	ssh origis_info@www.origis.info
