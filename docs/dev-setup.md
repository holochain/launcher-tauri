# Developer Setup

Go through the [tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites).

> Note: for now in Linux you will need to install `patchelf` and `librsvg2-dev`:
> `sudo apt-get install patchelf librsvg2-dev`


## Install Holochain and lair-keystore globally

Check out `.github/workflows/release.yaml` to see what holochain and lair-keystore versions are needed for this version of the Launcher.

The commands to install them should look like below, replacing the version numbers with the right ones. Run them in the root directory of this repository:

### Linux/macOs
```bash
cargo install --version 0.2.2 lair_keystore
LAIR_PATH=$(which lair-keystore)
cp $LAIR_PATH src-tauri/bins/lair-keystore-x86_64-apple-darwin

cargo install holochain --version 0.0.175 --locked --features db-encryption
HOLOCHAIN_PATH=$(which holochain)
cp $HOLOCHAIN_PATH src-tauri/bins/holochain-v0.0.175-x86_64-apple-darwin


[... install further holochain versions if required]

```

### Windows
```bash
cargo install --version 0.2.2 lair_keystore
$LkPath = Get-Command lair-keystore | Select-Object -ExpandProperty Definition
Copy-Item $LkPath -Destination src-tauri/bins/lair-keystore-x86_64-pc-windows-msvc.exe

cargo install holochain --version 0.0.175 --locked --features db-encryption
$HcPath = Get-Command holochain | Select-Object -ExpandProperty Definition
Copy-Item $HcPath -Destination src-tauri/bins/holochain-v0.0.175-x86_64-pc-windows-msvc.exe

[... install further holochain versions if required]

```


### Compiles and hot-reloads for development

```bash
npm run build:ui
npm run tauri dev
```
**Note:** This mode will show different behavior in some cases than running production builds.
To get the production build behavior (but without hot-reloading), run instead:

```bash
npm run build:ui
cd src-tauri && cargo run
```


### Compiles and minifies for production

```bash
npm run build
```