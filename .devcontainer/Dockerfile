# Use Ubuntu as the base image
# FROM ghcr.io/xtruder/nix-devcontainer:v1
FROM ubuntu:latest
LABEL maintainer="Jordan Hayes jordanhayes98@gmail.com"

RUN apt-get update && apt-get install curl xz-utils -y
RUN bash -c "sh <(curl -L https://nixos.org/nix/install) --daemon"
