test:
	cargo test --all-targets --all-features

test_print:
	cargo test --all-targets --all-features -- --nocapture

docs:
	RUSTFLAGS="--cfg docsrs" cargo +nightly doc --all-features --document-private-items

docs_open:
	RUSTFLAGS="--cfg docsrs" cargo +nightly doc --all-features --document-private-items \
		--open

docs_public:
	RUSTFLAGS="--cfg docsrs" cargo +nightly doc --all-features
