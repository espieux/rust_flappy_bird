# Build stage for compiling the Rust code to WebAssembly
FROM rust:latest as builder

# Install wasm-pack
RUN cargo install wasm-pack

# Copy the Rust source code and manifest into the container
WORKDIR /usr/src/rust_flappy_bird
COPY ./src ./src
COPY Cargo.toml Cargo.lock ./

# Build the Rust project to WebAssembly using wasm-pack
RUN wasm-pack build --target web --out-name wasm --out-dir ./pkg

# Serve stage: Use nginx to serve the static files
FROM nginx:alpine

# Copy the static HTML and JS files
COPY index.html /usr/share/nginx/html/index.html
COPY index.js /usr/share/nginx/html/index.js

# Copy the compiled WebAssembly code from the builder stage
COPY --from=builder /usr/src/rust_flappy_bird/pkg /usr/share/nginx/html/pkg

# Expose port 80 to access nginx
EXPOSE 80

# Start nginx and keep the process running
CMD ["nginx", "-g", "daemon off;"]
