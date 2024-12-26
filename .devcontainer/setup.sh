## update and install some things we should probably have
apt-get update
apt-get install -y \
  pkg-config \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  vim \
  build-essential \
  openssl \
  libssl-dev

## Install rustup and common components
# curl https://sh.rustup.rs -sSf | sh -s -- -y 