bindings:
	bash ./scripts/generate-bindings.sh

clean:
	cd rust_lib && rm -rf out && cargo clean

framework:
	bash ./scripts/build-framework.sh

lib:
	 cd rust_lib && \
	 cargo build --target aarch64-apple-ios --release && \
	 cargo build --target aarch64-apple-ios-sim --release && \
	 cargo build --target aarch64-apple-darwin --release
	
test:
	cd shared_lib && cargo test