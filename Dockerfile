FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /proxy

FROM chef AS planner
# prepare dependencies for caching
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# build project dependencies
COPY --from=planner /proxy/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# build project
COPY . .
RUN cargo build --release --bin tum-cal-proxy

FROM gcr.io/distroless/cc-debian12 as runtime
COPY --from=builder /proxy/target/release/tum-cal-proxy /proxy
ENTRYPOINT ["./proxy"]
EXPOSE 8080