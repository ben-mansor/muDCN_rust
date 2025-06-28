FROM rust:1.75 as build
WORKDIR /udcn
COPY . .
RUN cargo build --release --workspace

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /udcn/target/release/udcn-cli /usr/local/bin/udcn
COPY --from=build /udcn/target/release/udcn-daemon /usr/local/bin/udcn-daemon
CMD ["bash"]
