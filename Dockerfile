# syntax=docker/dockerfile:1.7

# dcrm is a Dioxus 0.7 NATIVE DESKTOP GUI app (WebView/WebKitGTK renderer,
# default_platform = "desktop", dioxus::launch). A desktop GUI cannot run as a
# meaningful headless service, so this image is an ARTIFACT BUILDER, not a
# deployable runtime:
#   - It carries the full Dioxus toolchain + the WebKitGTK/GTK build deps, so it
#     doubles as the per-task development container for dcrm
#     (dev workflow: docker run -it --rm -v "$PWD":/src -w /src dcrm-dev bash).
#   - The default build compiles the release desktop binary into /app/target/release.
#
# Build:   docker build -t dcrm-dev .
# Extract: docker run --rm -v "$PWD/out:/out" dcrm-dev \
#            sh -c 'cp target/release/dcrm /out/ 2>/dev/null || echo no-binary'

FROM rust:1.99-bookworm AS builder

# Dioxus desktop (WebView) build dependencies.
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      libwebkit2gtk-4.1-dev \
      libgtk-3-dev \
      libappindicator3-dev \
      librsvg2-dev \
      pkg-config \
      ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Dioxus CLI (dx) drives the desktop build and Tailwind CSS compilation.
RUN cargo install dioxus-cli --locked

WORKDIR /app

COPY Cargo.toml Cargo.lock Dioxus.toml ./
COPY src/ ./src/
COPY assets/ ./assets/
COPY tailwind.css ./

# Compile the release desktop binary. This proves dcrm builds reproducibly in a
# clean container. FAIL THE BUILD if the binary is not produced — no false green.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    dx build --release && \
    test -f /app/target/release/dcrm

# Keep the toolchain for dev-container use (container-dev-workflow). Source is
# bind-mounted at /src when used as a dev container; the CMD below is the
# artifact-extraction convenience for the baked build above.
CMD ["sh", "-c", "ls -la /app/target/release/dcrm 2>/dev/null || echo 'dcrm desktop binary builder image — bind-mount the repo at /src for dev'"]
