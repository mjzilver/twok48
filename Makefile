.PHONY: run
run:
	cargo fmt
	trunk serve --open

.PHONY: build
build:
	trunk clean
	trunk build --release