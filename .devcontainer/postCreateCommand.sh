# Install trunk
cargo install trunk

# Install WASM target
rustup target add wasm32-unknown-unknown

# Install tailwindcss
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v3.4.12/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mv tailwindcss-linux-x64 /bin/tailwindcss
