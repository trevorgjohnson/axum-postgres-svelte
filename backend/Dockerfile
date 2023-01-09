# This tells docker to use the Rust official image
FROM rust:1.66 as build

# Create a new empty shell project
RUN USER=root cargo new --bin axum-postgres-svelte
WORKDIR /axum-postgres-svelte

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependency is built, copy your source code
COPY ./src ./src

# Build for release.
RUN cargo clean
RUN cargo build --release

# our final base
FROM rust:1.66-slim-buster

# copy the build artifact from the build stage
COPY --from=build /axum-postgres-svelte/target/release/axum-postgres-svelte .

# Set ENV variables
ENV DATABASE_URL=postgres://postgres:postgres@85.239.241.0:5432/docker

# set the startup command to run your binary
CMD ["./axum-postgres-svelte"]
