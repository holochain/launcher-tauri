FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.1
RUN apt-get update && \
    apt-get install --assume-yes \
    libgdk-pixbuf2.0-dev \
    libssl-dev \
    pkg-config \
    libglib2.0-dev \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    squashfs-tools

ARG NODE_VERSION=14.16.0
ARG NODE_PACKAGE=node-v$NODE_VERSION-linux-x64
ARG NODE_HOME=/opt/$NODE_PACKAGE

ENV NODE_PATH $NODE_HOME/lib/node_modules
ENV PATH $NODE_HOME/bin:$PATH

RUN curl https://nodejs.org/dist/v$NODE_VERSION/$NODE_PACKAGE.tar.gz | tar -xzC /opt/

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
