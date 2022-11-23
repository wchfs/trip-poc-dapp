# syntax=docker.io/docker/dockerfile:1.4
FROM cartesi/toolchain:0.11.0 as dapp-build

WORKDIR /opt/cartesi/dapp
COPY . .

ENV CC=riscv64-cartesi-linux-gnu-gcc
ENV CFLAGS="-march=rv64ima -mabi=lp64"

RUN cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target riscv64ima-cartesi-linux-gnu.json --release
