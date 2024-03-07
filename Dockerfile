# Rust 소스 코드를 실행할 수 있는 환경을 제공하는 Docker 이미지를 선택합니다.
FROM rust:alpine as builder

# 작업 디렉토리를 설정합니다.
WORKDIR /usr/src/myapp

# 소스 코드를 Docker 컨테이너로 복사합니다.
COPY . .

# Cargo.toml 및 Cargo.lock 파일을 복사하고 의존성을 미리 빌드합니다.
RUN apk add --no-cache musl-dev && \
    cargo build --release

# 실행을 위한 최종 이미지를 준비합니다.
FROM alpine

# 빌드된 바이너리 파일을 복사합니다.
COPY --from=builder /usr/src/myapp/target/release/myapp /usr/local/bin/
# 컨테이너가 실행될 때 실행될 명령을 설정합니다.
CMD ["myapp"]
