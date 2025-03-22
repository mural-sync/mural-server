# Mural Server

This is the server software for mural.

Mural is a program that allows you to synchronize a wallpaper slideshow across your devices.
It supports having multiple different slideshows (called pools). For example, you might have
a pool called "Games" for wallpapers related to games and a pool called "Landscapes" for
wallpapers of beautiful landscapes.

## Setup

### Using Docker (recommended)

1. Download the provided `docker-compose.yml` file:

```bash
wget https://raw.githubusercontent.com/mural-sync/mural-server/refs/heads/main/docker-compose.yml
```

2. Create the `config` and `config/wallpapers` directories:

```bash
mkdir config
mkdir config/wallpapers
```

3. Put your wallpapers into `config/wallpapers`. For this guide, we will assume that there are
   two wallpapers in the directory: `wallpapers/picture1.png` and `wallpapers/picture2.jpg`.

4. Setup your pools in `config/config.toml`:

```toml
[pools]
default = [
  "picture1",
  "picture2",
]
```

5. Start the server:

```bash
docker compose up -d
```

### From Source

1. Download the source code for `mural-server`:

```bash
git clone https://github.com/mural-sync/mural-server
cd mural-server
```

2. Create the configuration and wallpaper directories in your configurations directory:

```bash
mkdir ~/.config/mural-server
mkdir ~/.config/mural-server/wallpapers
```

3. Put your wallpapers into `~/.config/mural-server/wallpapers`. For this guide, we will assume that there are
   two wallpapers in the directory: `wallpapers/picture1.png` and `wallpapers/picture2.jpg`.

4. Setup your pools in `~/.config/mural-server/config.toml`:

```toml
[pools]
default = [
  "picture1",
  "picture2",
]
```

5. Start the server:

```bash
RUST_LOG=info cargo run
```

## Configuration

This is a full configuration file using all the default options (except the pools configuration; `mural-server` does not setup any pools by default):

```toml
port = 46666 # the port to bind the server to
interval = 600 # how long each wallpaper should be shown (in seconds)

[pool]
foo = [
  "picture1",
  "picture2",
]
bar = [
  "picture2",
  "picture3",
]
```
