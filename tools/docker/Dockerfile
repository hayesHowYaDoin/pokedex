# Use Ubuntu as the base image
FROM ubuntu:22.04
LABEL maintainer="Jordan Hayes jordanhayes98@gmail.com"

# Declare container arguments
ARG USERNAME=default
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Configure timezone
ENV DEBIAN_FRONTEND=noninteractive 
ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Install rust dependencies
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install and configure SQLite
RUN apt-get update
RUN apt-get install -y \ 
    cargo \
    git \
    libsqlite3-dev \
    sqlite3

# [Optional] Install alternate shells
RUN apt-get install -y zsh fish

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME
