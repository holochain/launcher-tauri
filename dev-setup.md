# Developer Setup

Go through the [tauri developer setup](https://tauri.studio/en/docs/getting-started/intro#setting-up-your-environment).

> Note: for now in Linux you will need to install `patchelf` and `librsvg2-dev`:
> `sudo apt-get install patchelf librsvg2-dev`

## Project setup

This assumes that you have the `holochain`, `lair-keystore` and `caddy` binaries installed and available in your PATH.

```bash
yarn
sh scripts/setup-binaries.sh
```

### Compiles and hot-reloads for development

```bash
npm run serve
```

### Compiles and minifies for production

```bash
npm run build
```

### Customize configuration

See [Configuration Reference](https://cli.vuejs.org/config/).
