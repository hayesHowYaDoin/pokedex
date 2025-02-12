#!/usr/bin/env bash

# Function to determine the OS
detect_os() {
    case "$(uname -s)" in
        Linux*)     os=linux;;
        Darwin*)    os=mac;;
        *)          os="unknown"
    esac
    echo $os
}

# Function to determine the architecture
detect_arch() {
    arch=$(uname -m)
    case "$arch" in
        x86_64)    arch="x86_64";;
        aarch64)   arch="aarch64";;
        *)         arch="unknown"
    esac
    echo $arch
}

# Function to download and install the release
install_release() {
    version=$1
    os=$2
    arch=$3

    if [ "$os" == "unknown" ]; then
        echo "Unsupported operating system."
        exit 1
    fi

    url="https://github.com/hayesHowYaDoin/pokedex/releases/latest/download/x86_64-linux"
    echo "Downloading from $url"
    curl -L $url -o rich_pokedex.zip

    echo "Extracting files..."
    unzip rich_pokedex.zip

    echo "Installing..."
    chmod +x rich_pokedex
    mv rich_pokedex /usr/local/bin/
    mv data/pokedex.sqlite /usr/share/bin/
    mv -r data/assets /usr/share/bin/

    echo "Cleaning up..."
    rm rich_pokedex.zip

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
install_release $version $os
