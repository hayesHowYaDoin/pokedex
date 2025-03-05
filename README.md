# Pokedex

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![GitHub Release](https://img.shields.io/github/v/release/hayesHowYaDoin/pokedex)]()
[![CI](https://github.com/hayesHowYaDoin/pokedex/actions/workflows/ci.yml/badge.svg?event=push)]()

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

At this time, there are only releases for x86 (AMD64) and ARM64 Linux. The 
binaries for each, along with a Debian installer, can be found in the releases
section.

## Development

The following sections outline the process for setting up the and using the 
development environment.

### Prerequisites

The following instructions assume that [nix][3], [direnv][4], and [Git][5] are installed on 
the host computer.

### Setting Up The Development Environment

1) Clone the repository onto the host computer with the following command:
   ```
   git clone https://github.com/hayesHowYaDoin/pokedex.git
   ```
2) Navigate into the repository and un-block .envrc:
   ```
   direnv allow
   ```

And... that's it!

### Future Plans

Please refer to any open issues for any future planned work on this project.

[1]: https://github.com/rusqlite/rusqlite
[2]: https://github.com/ratatui-org/ratatui
[3]: https://nixos.org/
[4]: https://direnv.net/
[5]: https://git-scm.com/
