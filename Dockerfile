# syntax=docker/dockerfile:1.7
#
# dcrm — Dirmacs CRM.
#
# ⚠ STACK NOTE: dcrm is a Dioxus 0.7 *desktop* application
# (`dioxus = { features = ["desktop", "router"] }`). It renders a native
# window via WebView/GTK and has NO server/headless mode — no network service
# to expose. A desktop GUI app cannot run meaningfully in a container without
# display/GPU passthrough.
#
# This Dockerfile satisfies the phase-2 dockerization directive (every repo
# gets a Dockerfile) and acts as a *build vehicle* that proves the app
# compiles. It is NOT a runnable service image.
#
# dcrm builds via the Dioxus CLI (`dx`), NOT plain `cargo build`: the DX
# pipeline auto-generates `assets/tailwind.css` (gitignored) from the
# Tailwind v4 entry `tailwind.css`, and emits the desktop binary to
# target/x86_64-unknown-linux-gnu/desktop-release/. The builder stage copies a
# prebuilt `dx` binary (build context) to avoid a ~1h `cargo install`.
#
# Build (from the repo root, after copying dx into it):
#   cp ~/.cargo/bin/dx ./dx && docker build -t dcrm .
#
# Builder uses Debian 13 (trixie) — glibc >= 2.39 — because the prebuilt `dx`
# binary is built on a glibc-2.39 host and needs GLIBC_2.38/2.39 at runtime.
# dcrm's rust-version is 1.85, satisfied by trixie's 1.98 toolchain.

# ---- builder ---------------------------------------------------------------
FROM rust:1.98-trixie AS builder

# Dioxus desktop build needs webkit/gtk dev libraries plus libxdo-dev (the
# `xdo` crate links -lxdo) to satisfy the linker.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
        libxdo-dev pkg-config build-essential curl && \
    rm -rf /var/lib/apt/lists/*

# Use the prebuilt Dioxus CLI shipped in the build context (avoids compiling
# the ~100 MB dx CLI from source).
COPY dx /usr/local/bin/dx
RUN chmod +x /usr/local/bin/dx

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY Dioxus.toml tailwind.css ./
COPY assets/ ./assets/

# dx build runs the full Dioxus pipeline (tailwind asset gen + cargo build).
# Desktop binary lands in target/x86_64-unknown-linux-gnu/desktop-release/.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    dx build --release --platform desktop && \
    cp /app/target/x86_64-unknown-linux-gnu/desktop-release/dcrm /tmp/dcrm

# ---- runtime ---------------------------------------------------------------
# Desktop GUI: no headless entrypoint. Keep the built binary in a slim image
# so the artifact is captured and the build is proven; not expected to
# `docker run` as a service.
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    useradd --create-home --uid 1000 --shell /usr/sbin/nologin dcrm

COPY --from=builder /tmp/dcrm /usr/local/bin/dcrm

USER dcrm

# Intentionally no EXPOSE / HEALTHCHECK: nothing listens.
# Default command documents the desktop-GUI limitation rather than launching.
CMD ["sh", "-c", "echo 'dcrm is a Dioxus desktop GUI app; no headless/server mode. Run the binary on a host with a display.'"]
