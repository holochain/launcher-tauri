# Developer Setup

Go through the [tauri developer setup](https://tauri.studio/en/docs/getting-started/intro#setting-up-your-environment).

> Note: for now in Linux you will need to install `patchelf` and `librsvg2-dev`:
> `sudo apt-get install patchelf librsvg2-dev`

## Install Caddy

Go to [the Caddy website](https://caddyserver.com/) and install it for your distribution.

## Install Holochain and lair-keystore globally

You can use these commands to install them:

```bash
cargo install --locked holochain --git https://github.com/holochain/holochain.git --tag holochain-0.0.119
cargo install lair_keystore --git https://github.com/guillemcordoba/lair --rev 8c0a3683d93540301af53688ea431c7cc0beb438
```

## Project setup

This assumes that you have the `holochain`, `lair-keystore` and `caddy` binaries installed and available in your PATH.

```bash
yarn
sh scripts/setup-binaries.sh
```

### Compiles and hot-reloads for development

```bash
npm run dev
```

### Compiles and minifies for production

```bash
npm run build
```

### Customize configuration

See [Configuration Reference](https://cli.vuejs.org/config/).
