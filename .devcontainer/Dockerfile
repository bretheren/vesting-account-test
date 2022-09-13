### wasmd ###
FROM cosmwasm/wasmd:v0.28.0 as wasmd

### rust-optimizer ###
FROM cosmwasm/rust-optimizer:0.28.0 as rust-optimizer

# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.245.0/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
# ARG VARIANT="bullseye"
FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

COPY --from=wasmd /usr/bin/wasmd /usr/local/bin/wasmd
COPY --from=wasmd /opt/* /opt/

# Install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends curl jq \
    && sudo rm -rf /var/lib/apt/lists/*

RUN rustup update stable \
   && rustup target add wasm32-unknown-unknown