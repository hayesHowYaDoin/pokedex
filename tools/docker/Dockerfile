# Use Ubuntu as the base image
FROM ubuntu:22.04
LABEL maintainer="Jordan Hayes jordanhayes98@gmail.com"

# Configure timezone
ENV DEBIAN_FRONTEND=noninteractive 
ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Install dependencies
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
