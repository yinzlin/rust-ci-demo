# 构建阶段
FROM rust:1.95-slim-bullseye AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 运行阶段（很小）
FROM debian:bullseye-slim
COPY --from=builder /app/target/release/rust-ci-demo /usr/local/bin/app
CMD ["app"]
