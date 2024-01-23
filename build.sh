cd rust
cargo build --release
cd ..

mkdir bin
cp ./rust/target/release/nvim-gmail bin/nvim-gmail
