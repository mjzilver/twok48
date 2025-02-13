.PHONY: run
run:
	trunk serve --open

.PHONY: build
build:
	trunk clean
	trunk build --release