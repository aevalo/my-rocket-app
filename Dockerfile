# Stage 1: Build

FROM rust:1.99-alpine AS build

RUN apk add --no-cache musl-dev

WORKDIR /app

# Cache dependencies with a dummy build
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build the real application
COPY src ./src
COPY Rocket.toml ./
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Minimal production image
FROM alpine:3.19

RUN apk --no-cache add ca-certificates

# Create non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy the binary and configuration
COPY --from=build /app/target/release/my-rocket-app /server
COPY --from=build /app/Rocket.toml /Rocket.toml

USER appuser
EXPOSE 8000

# Set Rocket to release profile
ENV ROCKET_PROFILE=release

ENTRYPOINT ["/server"]
