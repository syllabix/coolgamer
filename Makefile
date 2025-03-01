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
				printf "  %-20s %s\n", $$1, doc_first_line; \
			} \
			' <"$(abspath $(lastword $(MAKEFILE_LIST)))" \
		| sort >&2
	@echo "" >&2

## install all development dependencies.
devenv:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-server-runner
	cargo install cargo-watch
	cargo install matchbox_server

## run Blockshot natively.
blockshot.run:
	matchbox_server &
	cd blockshot && cargo run

## run Blockshot in browser.
blockshot.run.web:
	matchbox_server &
	cd blockshot && cargo run --target wasm32-unknown-unknown

## run Venture Time natively.
venture.run:
	cd venture_time && cargo run

## run Paddle natively.
paddle.run:
	cd paddle && cargo run

## run Paddle in browser.
paddle.run.web:
	cd paddle && cargo run --target wasm32-unknown-unknown

## clean all build artifacts.
clean:
	cargo clean