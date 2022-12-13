# syntax=docker.io/docker/dockerfile:1.4
FROM cartesi/toolchain:0.12.0 as dapp-build

WORKDIR /opt/cartesi/dapp
COPY . .

ENV CC=riscv64-cartesi-linux-gnu-gcc
ENV CFLAGS="-march=rv64g -mabi=lp64d"

RUN cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target riscv64g-cartesi-linux-gnu.json --release

