.PHONY: gen
gen:
	cargo build --release --bin backstage
	cargo run --bin backstage --release
	
.PHONY: run
run:
	cargo run --release

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo test -- --show-output