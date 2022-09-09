VERSION 0.6
FROM rust
RUN apt update -y && apt install mono-devel -y
WORKDIR /rust


all:
	BUILD +check
	BUILD +test

build:
	COPY . .
	RUN cargo build

check:
	COPY . .
	RUN cargo check

test:
	COPY . .
	RUN cargo test


