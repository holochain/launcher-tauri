# Holochain Launcher

A cross-platform executable that launches a local Holochain conductor, and installs and opens apps.

> Feedback is immensely welcome in the [issues section](https://github.com/holochain/launcher/issues).

## Installing the Launcher

1. Go to the [Releases page](https://github.com/holochain/launcher/releases).
2. Download the appropriate executable for your platform and execute it.

## Updating Launcher

If you already have Launcher installed and would like to update to a new version of the Holochain Launcher, go to the [Releases page](https://github.com/holochain/launcher/releases) and download the appropriate executable for your platform.

### Windows

1. Double click the downloaded `.msi` file
2. On the Windows SmartScreen pop up, click on Run Anyway
3. Run through the setup process without changing the Destination Folder
4. The setup process will request to close applications that are using files that need to be updated. This will include the Old Holochain Launcher as well, if it is currently open
5. Click Ok at this step, to proceed with the setup
6. Open Launcher once installation is complete

### MacOS

1. Quit the running Holochain Launcher
2. Open the downloaded `.dmg` file and drag it to your Applications Folder
3. A pop-up with the message "An item named Holochain Launcher already exists in this location. Do you want to replace it with the one you're moving?" will appear
4. Select "Replace"
5. Go to the Applications folder
6. Click on Holochain Launcher and select Open

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

- On the zome side, target one of the HDK versions supported by the launcher. 
  - Find out which versions are supported in https://github.com/holochain/launcher/releases.
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

1. Open the `DevHub-0.0.x` app in the Holochain Launcher.
  - In case there are multiple DevHub apps, open the one for the latest version.
2. Create a new App, **with the tag "app-store-ready"**. Without this tag, the app won't appear in the app library.
   - Also input the appropriate "Name" and "Description". These are the details that the participants will see when downloading your application. Make the description short, maybe one or two lines.
3. Once you have created a new App, click "Upload new bundle".
4. Set the appropriate "Name" for your release (e.g. v0.0.1). This is the version name that the participants will see when downloading your application.
5. Select the `.webhapp` for your app and upload it.
6. Select the HDK appropriate version which your app was compiled for.
  - Make sure that this is the correct HDK, in the format "0.0.127".
6. Click "Quick Create" first for each zome, and then for each DNA.
7. Click "Save".
8.  Keep your Launcher open until another Launcher (in another computer) can install the app.
  - This may take a while (in the order of many minutes). In the future it won't be necessary.
  - This step is necessary for the happ files to sync with other peers in the DHT.

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
