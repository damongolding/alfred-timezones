# Build for intel and apple silicon then bundle them into a single file
cargo build --release --target=aarch64-apple-darwin && cargo build --release --target=x86_64-apple-darwin && lipo -create -output build/timezones target/{aarch64,x86_64}-apple-darwin/release/timezones
