# Build Binaries

## 32 bit win
rustup target add i686-pc-windows-msvc 
cargo build --target=i686-pc-windows-msvc 

## 64 bit win
rustup target add x86_64-pc-windows-msvc 
cargo build --target=x86_64-pc-windows-msvc 

## 32 bit linux standalone
rustup target add i686-unknown-linux-musl 
cargo build --target=i686-unknown-linux-musl 

## 64 bit linux standalone
rustup target add x86_64-unknown-linux-musl 
cargo build --target=x86_64-unknown-linux-musl 