FROM rust:slim AS builder
ARG ALIYUN=false

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    gpg \
    gnupg-agent \
    git \
    sudo

RUN ARCH=$(uname -m) && if [ "${ARCH}" = "x86_64" ]; then curl -fsSL https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key | \
    gpg --dearmor --output /usr/share/keyrings/intel-sgx.gpg && \
    echo 'deb [arch=amd64 signed-by=/usr/share/keyrings/intel-sgx.gpg] https://download.01.org/intel-sgx/sgx_repo/ubuntu jammy main' | \
    tee /etc/apt/sources.list.d/intel-sgx.list; fi && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    libclang-dev \
    libprotobuf-dev \
    libssl-dev \
    make \
    perl \
    pkg-config \
    protobuf-compiler \
    wget \
    clang \
    cmake \
    libtss2-dev && \
    if [ "${ARCH}" = "x86_64" ]; then apt-get install -y --no-install-recommends \
    libsgx-dcap-quote-verify-dev \
    libsgx-dcap-default-qpl \
    libsgx-dcap-quote-verify \
    libtss2-tctildr0 \
    libtss2-esys-3.0.2-0 \
    libtdx-attest-dev ; fi

# Build and Install KBS
WORKDIR /home
COPY . .

RUN git clone -b internal_main https://github.com/chl337/trustee.git \
        && cd trustee/kbs && \
        make AS_FEATURE=coco-as-builtin ALIYUN=${ALIYUN} background-check-kbs && \
        make install-kbs

CMD /usr/local/bin/kbs --config-file /home/trustee/kbs/config/kbs-config.toml
