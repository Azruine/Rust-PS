FROM rust:bullseye

RUN rustup component add rustfmt && \
    rustup component add clippy

# RUN apt-get update && apt-get install -y git curl unzip

# RUN curl -fsSL https://github.com/git-ecosystem/git-credential-manager/releases/latest/download/gcm-linux_amd64.2.6.0.deb -o gcm.deb && \
#     dpkg -i gcm.deb && \
#     rm gcm.deb

# RUN git-credential-manager configure