FROM nixos/nix:latest AS builder

WORKDIR /app

COPY flake.nix flake.lock .

COPY . .

RUN nix --extra-experimental-features "nix-command flakes" \
	develop .\#build --command ./scripts/generate-api-signatures.sh

RUN nix --extra-experimental-features "nix-command flakes" \
	develop .\#build --command cargo build --release

RUN mv target/release/auxiliary-api ./auxiliary-api

CMD ["./auxiliary-api"]
