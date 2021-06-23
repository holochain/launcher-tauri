# Developer Setup

Go through the [tauri developer setup](https://tauri.studio/en/docs/getting-started/intro#setting-up-your-environment).

## Project setup

This assumes that you have the `holochain`, `lair-keystore` and `caddy` binaries installed and available in your PATH

```bash
npm install
sh scripts/setup-binaries.sh
```

### Compiles and hot-reloads for development

```bash
npm run tauri:serve
```

### Compiles and minifies for production

```bash
npm run tauri:build
```

### Customize configuration

See [Configuration Reference](https://cli.vuejs.org/config/).
