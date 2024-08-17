cargo build --target x86_64-apple-darwin --release
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-darwin --release
cargo build --target aarch64-apple-darwin
lipo -create ./target/x86_64-apple-darwin/release/libcorrosion.dylib ./target/aarch64-apple-darwin/release/libcorrosion.dylib -output ./target/release/libcorrosion.dylib
./osx_vst3_bundler.sh Corrosion ./target/release/libcorrosion.dylib
