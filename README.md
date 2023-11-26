# LXC

[![Build Status](https://gitlab.com/sanpi/lxc-rs/badges/main/pipeline.svg)](https://gitlab.com/sanpi/lxc-rs/commits/main)

Crate to play with [LXC container](https://linuxcontainers.org/lxc/) in rust.

## Usage

Add it to your dependencies:

```
$ cargo add lxc
```

You also need the lxc C development files:

```
# Archlinux
$ sudo pacman -S lxc

# Debian/Ubuntu
$ sudo apt install lxc-dev

# Fedora
$ sudo dnf install lxc-devel
```

See [demo.rs](examples/demo.rs) for a complete example.
