FROM rust:slim-trixie

RUN sed -i 's/http/https/g' /etc/apt/sources.list.d/debian.sources && \
    apt update && export DEBIAN_FRONTEND=noninteractive && \
    apt install -y --no-install-recommends systemd-container && \
    apt clean && rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1337 dev 
