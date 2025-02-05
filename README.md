# Pokedex

![Builds](https://github.com/hayesHowYaDoin/pokedex/actions/workflows/build.yml/badge.svg?event=push)
![Lints](https://github.com/hayesHowYaDoin/pokedex/actions/workflows/lint.yml/badge.svg?event=push)
![Tests](https://github.com/hayesHowYaDoin/pokedex/actions/workflows/test.yml/badge.svg?event=push)

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

This application is intended to serve as a method of browsing the various 
statistics of the various Pokemon™ from the hit game series Pokemon™.

## Project Goals

The primary goal of this project is to gain more experience with the Rust 
programming language. It is also a place for me to experiment with the several 
fundamental crates, most notably [rusqlite][1] for database management and 
[ratatui][2] for a simple yet extensible TUI.

This project will also serve as a way to experiment with the "functional core, 
imperative shell" paradigm, whose inspiration can be seen through the file 
project's structure.

## Usage

TODO

## Development

The following sections outline the process for setting up the and using the 
development environment.

### Prerequisites

The following instructions assume that Docker, VSCode and Git are installed on 
the host computer. The VSCode extension Remote Development 
(ms-vscode-remote.vscode-remote-extensionpack) is required to open the project 
in a Dev Container. 

### Setting Up The Development Environment

1) Clone the repository onto the host computer with the following command:
   ```
    git clone https://github.com/hayesHowYaDoin/pokedex.git
   ```
2) Open the folder in VSCode. In the Command Palette (Ctrl+Shift+P), execute 
the command "Dev Containers: Open Folder In Container..."

And... that's it!

### Future Plans

TODO

[1]: https://github.com/rusqlite/rusqlite
[2]: https://github.com/ratatui-org/ratatui
