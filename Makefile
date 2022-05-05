.PHONY: Cargo.toml

build:
	cargo build

install: build
	rm -f /home/${USER}/.cargo/bin/pgen
	install target/debug/pgen /home/${USER}/.cargo/bin/pgen
	
release: build
	cp target/debug/pgen ./pgen-release
	zip pgen-release.zip pgen-release
	rm pgen-release

clean: 
	rm rm -rf target