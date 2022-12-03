

clean:
	cd ui && trunk clean
	cargo clean

build:
	cd ui && trunk build
	cargo build

run: build
	cargo run

clean-run:
	cd ui && trunk build
	cargo run

release:
	cd io && trunk build --release
	cargo build --release
