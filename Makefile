.PHONY: gen
gen:
	cargo run --bin backstage --release
	
.PHONY: run
run:
	cargo run --release