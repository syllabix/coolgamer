## print the help message.
# Parses this Makefile and prints targets that are preceded by "##" comments.
help:
	@echo "" >&2
	@echo "Available targets: " >&2
	@echo "" >&2
	@awk -F : '\
			BEGIN { in_doc = 0; } \
			/^##/ && in_doc == 0 { \
				in_doc = 1; \
				doc_first_line = $$0; \
				sub(/^## */, "", doc_first_line); \
			} \
			$$0 !~ /^#/ && in_doc == 1 { \
				in_doc = 0; \
				if (NF <= 1) { \
					next; \
				} \
				printf "  %-15s %s\n", $$1, doc_first_line; \
			} \
			' <"$(abspath $(lastword $(MAKEFILE_LIST)))" \
		| sort >&2
	@echo "" >&2

## install the dependencies for the project.
devenv:
	rustup target install wasm32-unknown-unknown
	rustup target add wasm32-unknown-unknown
	cargo install wasm-server-runner
	cargo install cargo-watch

## run the game
run:
	cargo run
	
## run the game natively
run.native:
	cargo run

## run the project in the browser.
run.web:
	cargo run --target wasm32-unknown-unknown

## run the project in the browser and watch for changes.
watch.web:
	cargo watch -cx "run --target wasm32-unknown-unknown"

## run the project in the native environment and watch for changes.
watch.native:
	cargo watch -cx "run"