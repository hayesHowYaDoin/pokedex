#!/usr/bin/env bash

if [ -z "$1" ]; then
    CURRENT_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    ASSETS_PATH="$CURRENT_PATH/assets"
else
    ASSETS_PATH="$1"
fi

if [ -z "$2" ]; then
    CORES=$(nproc)
else
    CORES="$2"
fi

retrieve_assets() {
    if [ -z "$ASSETS_PATH" ]; then
        echo "No assets path provided"
        return
    fi
    
    if [ -z "$1" ]; then
        echo "No id provided"
        return
    fi

    local id=$1

    mkdir -p ${ASSETS_PATH}/${id}

    # Download sprites
    wget -nc -O ${ASSETS_PATH}/${id}/bw_front.png https://veekun.com/dex/media/pokemon/main-sprites/black-white/${id}.png

    # Download cries
    if [ ! -f "$ASSETS_PATH/$id/cry.wav" ]; then
        wget -nc -O ${ASSETS_PATH}/${id}/cry.ogg https://veekun.com/dex/media/pokemon/cries/${id}.ogg
        ffmpeg -y -i "$ASSETS_PATH/$id/cry.ogg" -acodec pcm_s16le "$ASSETS_PATH/$id/cry.wav"
    fi

    if [ -f "$ASSETS_PATH/$id/cry.ogg" ]; then
        rm "$ASSETS_PATH/$id/cry.ogg"
    fi
}

export ASSETS_PATH
export -f retrieve_assets

echo "Downloading assets to: $ASSETS_PATH "
echo "Using $CORES cores..."

seq 1 649 | xargs -n 1 -P $CORES -I {} bash -c 'retrieve_assets "$@"' _ {}
