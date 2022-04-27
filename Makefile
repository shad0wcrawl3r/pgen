.PHONY: Cargo.toml

build:
	cargo build

install: build
	rm -f /home/${USER}/.cargo/bin/pgen
	install target/debug/pgen /home/${USER}/.cargo/bin/pgen
	

clean: 
	rm rm -rf target