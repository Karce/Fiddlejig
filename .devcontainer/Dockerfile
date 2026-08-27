# Debian 13 (trixie) — glibc 2.41.
#
# This image is for Rust development and testing only: `cargo build`, `cargo test`,
# rust-analyzer. It deliberately does NOT try to produce a binary that runs on the
# bare host — the shippable artifact is a Flatpak, built on the host with
# org.flatpak.Builder. See tools/autofisher-3000/packaging/.
FROM docker.io/library/debian:trixie-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    # toolchain
    build-essential pkg-config \
    # opencv-rs runs a libclang-based binding generator at build time
    clang libclang-dev \
    # OpenCV 4.10 from the distro: shared libs plus opencv4.pc, so opencv-rs's
    # pkg-config probe just works and no OPENCV_* env overrides are needed.
    libopencv-dev \
    # videoio's GStreamer backend, plus the pipewiresrc element capture.rs needs
    libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev \
    gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-pipewire \
    # ort's default (download-binaries + tls-native) pulls in openssl-sys
    libssl-dev \
    # deps of flatpak-cargo-generator.py, so packaging/cargo-sources.json can be
    # regenerated in here whenever Cargo.lock moves
    python3 python3-aiohttp python3-yaml python3-tomlkit \
    # general tooling. openssh-client is NOT optional: the git remote is
    # git@github.com, and Debian's git does not pull it in under
    # --no-install-recommends, so push/fetch fail with "cannot run ssh".
    ca-certificates curl git openssh-client gh sudo fish \
    && rm -rf /var/lib/apt/lists/*

# uid 1000 matches the host user, so bind-mounted files keep their ownership
# (devcontainer.json pins updateRemoteUserUID=false). fish is the login shell.
#
# /home/vscode/.claude must exist with the right owner in the image: the named
# volume mounted there (see "mounts" in devcontainer.json) inherits the image
# directory's ownership when first created, and would otherwise come up
# root-owned and unwritable.
RUN useradd -m -u 1000 -s /usr/bin/fish vscode \
    && echo "vscode ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/vscode \
    && chmod 0440 /etc/sudoers.d/vscode \
    && install -d -o vscode -g vscode /home/vscode/.claude

USER vscode
