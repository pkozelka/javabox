R_VERSION=shapshot

all:

mklinks:
	mkdir -p target
	ln -s debug/examples/multicall-derive target/true
	ln -s debug/examples/multicall-derive target/false

install:
	cargo install --path .

dist:
	cargo build --release
	echo $(R_VERSION)
	rm -rf "target/dist"
	mkdir -p "target/dist"
	cp -t "target/dist" "target/release/javabox"
	cp -t "target/dist" "launchers/mvnw"

upload: dist
	rsync -e "ssh -o StrictHostKeyChecking=no" -rlpcgoD -zi --delete target/dist/ origis_info@www.origis.info:dist.origis.info/javabox/$(R_VERSION)
