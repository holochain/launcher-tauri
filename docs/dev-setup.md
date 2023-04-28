# Developer Setup

## Tauri and Holochain Prerequisites

Go through the [tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites).

[Install Go](https://go.dev/doc/install)

> Note: for now on Linux you will need to install `patchelf` and `librsvg2-dev`:
> `sudo apt-get install patchelf librsvg2-dev`

## Install Holochain and lair-keystore globally

Check out `.github/workflows/release.yaml` to see what holochain and lair-keystore versions are needed for this version of the Launcher.

The commands to install them should look like below, replacing the version numbers with the right ones and `[_ARCHITECTURE_]` with the architecture of your computer. Run them in the root directory of this repository:

### Linux/macOS
```
mkdir src-tauri/bins

cargo install --version 0.2.4 lair_keystore
LAIR_PATH=$(which lair-keystore)
cp $LAIR_PATH src-tauri/bins/lair-keystore-v0.2.4-x86_64-apple-darwin
cp $LAIR_PATH src-tauri/bins/lair-keystore-v0.2.4-[_ARCHITECTURE_]


cargo install holochain --version 0.2.0-beta-rc.7 --locked --features db-encryption
HOLOCHAIN_PATH=$(which holochain)
cp $HOLOCHAIN_PATH src-tauri/bins/holochain-v0.2.0-beta-rc.7-[_ARCHITECTURE_]


[... install further holochain versions if required]

```
`[_ARCHITECTURE_]` is `x86_64-apple-darwin` on an x86 macOS, `aarch64-apple-darwin` on an Arm/M1 macOS and `unknown-linux-gnu` on Linux.

### Windows
```
cargo install --version 0.2.4 lair_keystore
$LkPath = Get-Command lair-keystore | Select-Object -ExpandProperty Definition
Copy-Item $LkPath -Destination src-tauri/bins/lair-keystore-v0.2.4-x86_64-pc-windows-msvc.exe

cargo install holochain --version 0.2.0-beta-rc.7 --locked --features db-encryption
$HcPath = Get-Command holochain | Select-Object -ExpandProperty Definition
Copy-Item $HcPath -Destination src-tauri/bins/holochain-v0.2.0-beta-rc.7-x86_64-pc-windows-msvc.exe

[... install further holochain versions if required]

```


### Compiles and hot-reloads for development

```bash
npm run build:ui
npm run tauri dev
```
**Note:** This mode will show different behavior in some cases than running production builds.
To get the production build behavior (but thereby sacrificing hot-reloading), run instead:

```bash
npm run build:ui
cd src-tauri && cargo run
```


### Compiles and minifies for production

```bash
npm run build
```