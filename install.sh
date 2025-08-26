#!/usr/bin/env bash

detect_os() {
    case "$(uname -s)" in
        Linux*)     os=linux;;
        *)          os="unknown"
    esac
    echo $os
}

detect_arch() {
    arch=$(uname -m)
    case "$arch" in
        x86_64)    arch="amd64";;
        aarch64)   arch="arm64";;
        *)         arch="unknown"
    esac
    echo $arch
}

install_release() {
    version=$1
    os=$2
    arch=$3

    if [ "$os" == "unknown" ]; then
        echo "Unsupported operating system."
        exit 1
    fi

    # Create and navigate to temporary installation directory
    mkdir -p tmp
    cd tmp

    url="https://github.com/hayesHowYaDoin/pokedex/releases/$version/download/$arch-$os";
    echo "Downloading from $url"
    {
        curl -L $url -o rich_pokedex.zip;
    } || { echo "Download failed"; cd .. && rm -fr ./tmp; exit 1; }

    echo "Extracting files..."
    unzip rich_pokedex.zip || { echo "Extraction failed"; cd .. && rm -fr ./tmp; exit 1; }

    echo "Installing..."
    {
        mkdir -p /usr/share/rich_pokedex;
        chmod +x ./rich_pokedex;
        mv ./rich_pokedex /usr/local/bin && \
        mv ./pokedex.db ./assets /usr/share/rich_pokedex;
    } || { echo "Installation failed"; cd .. && rm -fr ./tmp; exit 1; }

    echo "Cleaning up..."
    cd .. && rm -fr ./tmp

    echo "Installation complete."
}

# Main script
if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

version=$1
os=$(detect_os)
arch=$(detect_arch)

install_release $version $os $arch
