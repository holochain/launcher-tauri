# Holochain Launcher

A cross-platform executable that launches a local Holochain conductor, and installs and opens apps.

> Feedback is immensely welcome in the [issues section](https://github.com/holochain/launcher/issues).

## Installing the Launcher

1. Go to the [Releases page](https://github.com/holochain/launcher/releases).
2. Download the appropriate executable for your platform and execute it.

## Usage

### Setting Up the Password

The first time you start the Holochain Launcher, it will ask you to set up a password. This password will be used later to unlock the encrypted data that will be stored in your computer.

WARNING! If you lose this password, you will also lose access any data from your Holochain applications, as there is no password recovery mechanism.

### Installing a hApp

When you are on the `Installed Apps` section, click `Install New App`. This will bring up the `App Store`, from which you can:

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;1.a) Install an application from the list of published applications.

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;or

&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;1.b) Select Application from the FileSystem`.

2. Set a unique `AppId` for the app, and click install, it may take a while.
   > This AppId only needs to be unique inside your Holochain Launcher, it doesn't affect other users.
3. When it's finished installation, the happ will appear in the list of active happs.
4. Click `Open` in one of the installed apps to open the app.

### Packaging a Web-hApp

Requirements:

- On the zome side, target `hdk v0.0.127`.
- On the UI side, target `@holochain/client v0.3.2` (v0.2.x is not going to work).

When your UI is served from the launcher and `AppWebsocket.connect()` or `AdminWebsocket.connect()` are called, the call will be redirected to the actual port in which Holochain is running its interfaces. Also, when the `appWebsocket.appInfo({ installed_app_info: <APP_ID> })` is called, the `APP_ID` parameter is going to be replaced with the actual application ID that is installed in the Launcher.

For now, the UI still has access to the admin interface, but its usage is discouraged as newer versions of the launcher will block any admin requests not coming from the Holochain Admin UI itself. There will be a call available in `AppWebsocket` that will prompt the user to do an action in the Holochain Admin UI, or similar (To Be Defined).

1. Package your happ into a `.happ` file, with `hc app pack`.
2. Package your UI into a `.zip` file. IMPORTANT: this `.zip` is supposed to contain an `index.html` file in its root level.
3. Create a Web-hApp manifest with `hc web-app init`.
4. Set the corrrect locations for the UI `.zip` file and the `.happ` file.
5. Package your Web-hApp with `hc web-app pack`, and publish it so that other users can download and install it.

See [where](https://github.com/lightningrodlabs/where) for a complete working setup.

### Publishing a .webhapp to the DevHub

1. Open the `DevHub` app in the Holochain Launcher.
2. Create a new App, with the appropriate "Name" and "Description". These are the details that the participants will see when downloading your application.
3. Once you have created a new App, click "Upload new bundle".
4. Select the `.happ` (not the `.webhapp`) and upload it.
5. Select the HDK appropriate version which your app was compiled for.
6. Click "Quick Create" first for each zome, and then for each DNA.
7. Click "Save".
8. When you are done, go back to the application detail page and click "Create New Release".
9. Set the appropriate "Name" (e.g. v0.0.1). This is the version name that the participants will see when downloading your application.
10. In the DNA box, search for the DNAs you uploaded earlier.
11. In the GUI box, upload your `ui.zip`.
12. Click "Save".

## Known issues

- Encryption at rest is not supported yet. It will be enabled when the upstream `rusqlite` crate creates the next release, introducing the option of statically bundling `sqlcipher` in MacOs and in Windows.
- Only MacOs v10.13 and after are supported at this moment.

## Data storage

The Holochain Launcher uses the same config and data locations as the main `holochain` and `lair-keystore` processes. These locations are:

- Configuration: `$CONFIG_DIR/holochain`.
  - In Ubuntu this is `$HOME/.config/holochain`.
- Data: `$DATA_DIR/holochain` and `$DATA_DIR/lair`.
  - In Ubuntu this is `$HOME/.local/share/holochain` and `$HOME/.local/share/lair`.

To reset the launcher, you can execute a factory reset in the `Holochain Admin` window > `Settings` menu > `Factory Reset`.

### Logs

Logs can be found at `$DATA_DIR/holochain-launcher/launcher.log`. When reporting issues, please attach the contents of this file.

## Developer Setup

See [Developer Setup](/dev-setup.md).

## Adding a new Holochain version

See [Adding Holochain Version](/adding-holochain-version.md).

## License

[![License: CAL 1.0](https://img.shields.io/badge/License-CAL%201.0-blue.svg)](https://github.com/holochain/cryptographic-autonomy-license)

Copyright (C) 2019 - 2021, Holochain Foundation

This program is free software: you can redistribute it and/or modify it under the terms of the license
provided in the LICENSE file (CAL-1.0). This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
PURPOSE.
