# Holochain Launcher

A cross-platform executable that launches a local Holochain conductor, and lets you install and run Holochain apps.

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


# For Developers

### Packaging a Web-hApp

Requirements:

- On the zome side, target one of the HDK versions supported by the launcher.
  - Find out which versions are supported in https://github.com/holochain/launcher/releases.

The easiest way to create a Holochain web-app is to use holochain's [scaffolding tool](https://docs.rs/holochain_scaffolding_cli/latest/holochain_scaffolding_cli/). If you develop in [nix-shell](https://developer.holochain.org/install/), `hc scaffold` will be readily available to you to call in the command line.

When your UI is served from the launcher and `AppWebsocket.connect()` or `AdminWebsocket.connect()` are called, the call will be redirected to the actual port in which Holochain is running its interfaces. Also, when the `appWebsocket.getAppInfo({ app_id: <APP_ID> })` is called, the `APP_ID` parameter is going to be replaced with the actual application ID that is installed in the Launcher.

For now, the UI still has access to the admin interface, but its usage is discouraged as newer versions of the launcher will block any admin requests not coming from the Holochain Admin UI itself. There will be a call available in `AppWebsocket` that will prompt the user to do an action in the Holochain Admin UI, or similar (To Be Defined).

If you used the scaffolding tool to create your hApp, packaging it for the launcher is as easy as running `npm run package` in the root directory of your project. Otherwise you need to:
1. Package your happ into a `.happ` file, with `hc app pack`.
2. Package your UI into a `.zip` file. IMPORTANT: this `.zip` is supposed to contain an `index.html` file in its root level.
3. Create a Web-hApp manifest with `hc web-app init`.
4. Set the corrrect locations for the UI `.zip` file and the `.happ` file.
5. Package your Web-hApp with `hc web-app pack`, and publish it so that other users can download and install it.


### Publishing a .webhapp to the DevHub


1. Open the `DevHub-0.1.0-beta-rc.2` app in the Holochain Launcher.
2. Click on "Add hApp", then choose a title, subtitle and description for your app and **add the tag "app-store-ready"**. Without this tag, the app won't appear in the app library.
3. Once you have created a new hApp, click "Upload new bundle".
4. Enter the HDK version your hApp is written with, then upload the .webhapp file of your app.
5. You will be prompted to choose names, descriptions and version numbers for your zomes and DNA's. Fill out all necessary fields and create the zomes and DNA's of your hApp, then click on "Continue".
6. Set the appropriate "Name" for your release (e.g. v0.0.1). This is the version name that participants will see when downloading your application.
7. Review and save your hApp.
8. Now you need to upload the GUI (provided your hApp has one) and associate it with the hApp release you just made. Click on "GUIs" in the left panel and then "Add GUI".
9. Give your GUI a name and a description and save it. No tag is required but make sure to choose a name that let's you disambiguate it from other developers GUI's.
10. Go to "My GUIs" in the left panel, click on "More info" on the GUI you just added, then "Create new release".
11. Choose a version number for the GUI and upload your GUI as a zip file in the file drop area and click "save". "Select hApp Releases" is not required.
12. Go back to "My hApps" in the left panel, then click on "More info" of the hApp you added earlier.
13. Under "hApp Releases", click on "More info" for the Release you want to add the GUI to, then click on "Edit release".
14. Click on "Select GUI Release" and select the GUI you just uploaded. THen click on "Save changes".
15. Now everything is ready but make sure to keep your Launcher open until another Launcher (on another peer's computer) can install the app (to be safe, wait for about 30 minutes at least).
16. If you get stuck at any point, don't hesitate to ask for help on Holochain's Discord server. You should find an invite link to it on https://developer.holochain.org/.


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

See [Developer Setup](/docs/dev-setup.md).


## License

[![License: CAL 1.0](https://img.shields.io/badge/License-CAL%201.0-blue.svg)](https://github.com/holochain/cryptographic-autonomy-license)

Copyright (C) 2019 - 2021, Holochain Foundation

This program is free software: you can redistribute it and/or modify it under the terms of the license
provided in the LICENSE file (CAL-1.0). This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
PURPOSE.
