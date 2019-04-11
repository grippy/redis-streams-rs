.PHONY: test-all doc fmt

fmt:
	cargo +nightly fmt

doc:
	cargo doc --no-deps --jobs=10

test-all:
	RUST_BACKTRACE=true REDISRS_SERVER_TYPE=tcp cargo test -- --nocapture