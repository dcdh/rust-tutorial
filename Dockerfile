FROM rust:1.85.0 AS build

# Set working directory
WORKDIR /app

# 1️⃣ Copy only Cargo.toml and Cargo.lock first (to use Docker cache)
COPY Cargo.toml Cargo.lock ./

# 2️⃣ Create a dummy main.rs to compile only dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 3️⃣ Precompile dependencies
RUN cargo build --release --target-dir=/app/target

# 4️⃣ Now copy the actual source code
COPY src ./src

# 5️⃣ Compile the final binary
RUN cargo build --release --target-dir=/app/target

# Step 2: Final lightweight image
FROM debian:bookworm-slim

RUN apt-get update; \
    apt-get install -y \
        ca-certificates \
        ;

# Copy only the compiled binary
COPY --from=build /app/target/release/rust-tutorial /app/rust-tutorial

# Copy static assets (if needed)
COPY assets ./app/assets

EXPOSE 3000

# set the startup command to run your binary
CMD ["/app/rust-tutorial"]
