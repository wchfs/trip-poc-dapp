# syntax=docker.io/docker/dockerfile:1.4
FROM cartesi/toolchain:0.10.0 as dapp-build

RUN apt update \
    && apt install -y \
    libsqlite3-dev

WORKDIR /opt/cartesi/dapp
COPY . .

RUN cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target riscv64ima-cartesi-linux-gnu.json --release

RUN cargo install diesel_cli --no-default-features --features sqlite
#
RUN echo DATABASE_URL=database.db > .env
#
RUN diesel setup
#
RUN diesel migration run
