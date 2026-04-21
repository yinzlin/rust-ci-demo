# ============================================
# 多阶段构建：前端构建 + 后端构建 + 运行
# ============================================

# -------------------------------------------
# 阶段1：构建前端（Dioxus Web）
# -------------------------------------------
FROM rust:1.95-slim-bullseye AS frontend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    binaryen \
    && rm -rf /var/lib/apt/lists/*
COPY crates/frontend/ .
RUN cargo install trunk && trunk build --release

# -------------------------------------------
# 阶段2：构建后端（Axum）
# -------------------------------------------
FROM rust:1.95-slim-bullseye AS backend-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
RUN cargo build --release -p backend

# -------------------------------------------
# 阶段3：运行阶段
# -------------------------------------------
FROM debian:bullseye-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend-builder /app/target/release/app /usr/local/bin/
COPY --from=frontend-builder /app/dist /app/dist

EXPOSE 8080

CMD ["app"]