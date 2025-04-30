cargo build --release --target x86_64-pc-windows-msvc
cp ./target/x86_64-pc-windows-msvc/release/space-rs.exe .

cargo build --release --target wasm32-unknown-unknown
cp ./target/wasm32-unknown-unknown/release/space-rs.wasm .

wsl -e cargo build --release --target x86_64-unknown-linux-gnu
cp ./target/x86_64-unknown-linux-gnu/release/space-rs .