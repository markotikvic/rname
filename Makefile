all:
	cargo build --release
install:
	cp target/release/rname /usr/local/bin
