cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-unknown-linux-gnu
mkdir -p dist
cp target/release/drcom dist/drcom-cli
cp target/aarch64-unknown-linux-gnu/release/drcom-cli dist/drcom-aarch64
cp target/x86_64-pc-windows-gnu/release/drcom-cli.exe dist/drcom.exe
