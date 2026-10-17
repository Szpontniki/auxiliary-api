FROM nixos/nix:latest AS builder

WORKDIR /app

RUN mkdir -p /etc/nix && \
	echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

COPY flake.nix flake.lock .

COPY . .

RUN nix develop .\#build --command ./scripts/generate-api-signatures.sh

RUN nix develop .\#build --command cargo build --release

# Yes, this is ugly, but it also makes the image 3GiB smaller.
# It's a shame that nix doesn't have a built-in utility CLI like this.
# Copies runtime dependencies required by the binary from /nix/store to be used in Debian.
RUN nix develop .\#build --command bash -euc '\
	{ \
		readelf -l /app/target/release/auxiliary-api; \
		readelf -d /app/target/release/auxiliary-api; \
	} | grep -oE "/nix/store/[a-z0-9]{32}-[^/:]+" | sort -u \
		> /tmp/runtime-roots; \
	nix-store -qR $(cat /tmp/runtime-roots) | sort -u \
		> /tmp/runtime-closure; \
	mkdir -p /tmp/rootfs; \
	tar -C / -cf /tmp/runtime-store.tar -T /tmp/runtime-closure; \
	tar -C /tmp/rootfs -xf /tmp/runtime-store.tar; \
	rm /tmp/runtime-store.tar'

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /tmp/rootfs/nix/store /nix/store
COPY --from=builder /app/target/release/auxiliary-api /app/auxiliary-api

CMD ["/app/auxiliary-api"]
