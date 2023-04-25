# Holochain Launcher

A cross-platform executable that launches a local Holochain conductor, and lets you install and run [Holochain](https://www.holochain.org) apps.

> Feedback is more than welcome in the [issues section](https://github.com/holochain/launcher/issues).

## Installing the Launcher

1. Go to the [Releases page](https://github.com/holochain/launcher/releases).
2. Download the appropriate executable for your platform and execute it.

Notes:
* **macOS**: If you are running the Holochain Launcher from a `.app` file extracted from an `app.tar.gz` bundle instead of having it installed from a `.dmg` file, automatic updates will only succeed if the `.app` file is located in the `Applications` folder due to file permissions.
* **Linux**: Automatic updates are currently not supported when the Launcher is installed from a `.deb` file instead of runniing it from the `.AppImage`.


## Updating Launcher

If your current version is 0.9.0 or lower (click on "Menu" > "Version Info" on Windows/Linux or "Launcher > "Version Info" on macOS to find out the version you are using), go to the [Releases page](https://github.com/holochain/launcher/releases) and install the latest version of the Holochain Launcher.

If your current version is 0.9.1 or higher, you should automatically receive Updates in the Launcher and don't need to care about manually updating the Launcher.

## Usage

### Setting Up the Password

The first time you start the Holochain Launcher, it will ask you to set up a password. This password will be used later to unlock the encrypted data that will be stored in your computer.

WARNING! If you lose this password, you will also lose access any data from your Holochain applications, as there is no password recovery mechanism.

### Installing a Holochain App (hApp)

When you are on the `Installed Apps` section, click `Install New App`. This will bring up the `App Library`, from where you can:

1. Install an application from the list of published applications or install one from your file system.
2. Choose a unique `AppId` for the app, optionally enter a network seed and click install. It may take a while.
   > This AppId only needs to be unique inside your Holochain Launcher, it doesn't affect other users.
3. When the installation is finished, the hApp will appear in the list of active hApps.
4. Click on the icon that says "Open App" when hovering over it to open the app in a new window.


# For Developers

### Packaging a Web hApp

Requirements:

The Holochain Launcher is currently using a `0.1.X` version of Holochain. Use an HDK and HDI version that is compatible with it.

The easiest way to create a Holochain web-app is to use holochain's [scaffolding tool](https://docs.rs/holochain_scaffolding_cli/latest/holochain_scaffolding_cli/). If you develop in [nix-shell](https://developer.holochain.org/install/), `hc scaffold` will be readily available to you to call in the command line.

When your UI is served from the launcher and `AppWebsocket.connect()` or `AdminWebsocket.connect()` are called, the call will be redirected to the actual port in which Holochain is running its interfaces. Also, when the `appWebsocket.getAppInfo({ app_id: <APP_ID> })` is called, the `APP_ID` parameter is going to be replaced with the actual application ID that is installed in the Launcher.

For now, the UI still has access to the admin interface, but *its usage is discouraged* as newer versions of the Launcher will block any admin requests not coming from the Holochain Admin UI itself. There will be a call available in `AppWebsocket` that will prompt the user to do an action in the Holochain Admin UI, or similar (to be defined).

If you used the scaffolding tool to create your hApp, packaging it for the launcher is as easy as running `npm run package` in the root directory of your project. Otherwise you need to:
1. Package your happ into a `.happ` file, with `hc app pack`.
2. Package your UI into a `.zip` file. IMPORTANT: this `.zip` is supposed to contain an `index.html` file at its root level.
3. Create a Web-hApp manifest with `hc web-app init`.
4. Set the corrrect locations for the UI `.zip` file and the `.happ` file.
5. Package your Web-hApp with `hc web-app pack`, and publish it so that other users can download and install it or simply share it with your friends.


### Publishing and Updating an App in the DevHub

In order for your app to appear in the App Library within the Holochain Launcher, you need to publish it to the DevHub, which is itself a Holochain app. Currently, the DevHub is installed by default in the Holochain Launcher.

#### Publishing your app

1. Open the `DevHub-0.1.3` app in the Holochain Launcher.
2. Click on "Add hApp", then choose a title, subtitle and description for your app and **add the tag "app-store-ready"**. Without this tag, the app won't appear in the app library.
3. Once you have created the new hApp, click on "Upload Bundle".
4. If your app has an accompanying web UI, upload the `.webhapp` file of your app, if it's a headless app, upload the `.happ` file of your app.
5. You will be prompted to choose names, descriptions and version numbers for your zomes and DNA's. Fill out all necessary fields and create the zomes and DNA's of your hApp.
6. On the same page still, you can also give a name and version number to the UI that comes with the `.webhapp` file if it's a web app. The UI will be stored separately from the `.happ` file such that you may later associate a new "official UI" to the same happ release. You will be able to associate any UI of *anyone* to your happ release so *make sure to choose a unique enough name for the UI such that you will still recognize it amongst all other UI's*.
7. Click on "Continue" and then set the appropriate "Name" for your release (e.g. v0.0.1, *this is the version name that participants will see when downloading your application*) as well as a changelog.
7. Review and save your hApp.
8. Your app should now appear in the App Library.
9. Now everything is ready locally but make sure to keep your Launcher open until another Launcher (on another peer's computer) can install the app. To be safe, wait for about 30 minutes at least.

If you get stuck at any point, don't hesitate to ask for help on Holochain's Discord server. You should find an invite link to it on https://developer.holochain.org/.

#### Updating the UI of your Web App
Provided that you already created a happ release as described just above, you can always update the UI associated with it. To do so, follow these steps:
1. Click on "My GUIs" in the left side panel of the DevHub
2. In the list, select the UI that you want to update by clicking on "More Info"
3. Click on "Create new release"
4. Add a version number for that UI, as well as a changelog that describes the changes this new UI comes with. People updating to the latest UI of your app will see this text displayed before confirming to update.
5. Upload your updated web assets as a `.zip` file. Optionally you can explicitly specify hApp releases that this UI is compatible with. Then click on "Save".
6. Now go to "My hApps" in the left side panel of the DevHub and select the hApp you want to update the UI for by clicking on "More Info".
7. Under "hApp Releases", select the happ release of that happ that you want to update the UI for by clicking again on "More Info".
8. Click on "Edit release".
9. Click on "Select GUI Release" and select the GUI release you just created earlier and click on "Finish".
10. Click on "Save Changes".
11. The UI of your app should now be updated and after (re)starting the Launcher, people will see the option to update the UI in case they already have your app installed.
12. Again, to make sure your uploads get gossiped in the network, keep your Launcher open until another Launcher (on another peer's computer) can install the app. To be safe, wait for about 30 minutes at least.


If you get stuck at any point, don't hesitate to ask for help on Holochain's Discord server. You should find an invite link to it on https://developer.holochain.org/.


## Data storage

The Holochain Launcher stores data in different places on the file system, depending on the platform:

- Configuration:
  - Windows: `{FOLDERID_RoamingAppData}/holochain-launcher`
  - macOS: `$HOME/Library/Application Support/holochain-launcher`
  - Linux: `$HOME/.config/holochain-launcher`.
- Data:
  - Windows: `{FOLDERID_RoamingAppData}/holochain-launcher`
  - macOS: `$HOME/Library/Application Support/holochain-launcher`
  - Linux: `$XDG_DATA_HOME/holochain-launcher` or `$HOME/.local/share/holochain-launcher`
- Logs:
  - Windows: `{FOLDERID_RoamingAppData}/holochain-launcher/profiles/${profile}/logs`
  - macOS: `$HOME/Library/Logs/holochain-launcher`
  - Linux: `$XDG_DATA_HOME/holochain-launcher/profiles/${profile}/logs` or `$HOME/.local/share/holochain-launcher/profiles/${profile}/logs`

To reset the launcher, you can execute a factory reset in the `Holochain Admin` window via `Settings` > `Factory Reset`.

## Developer Setup

See [Developer Setup](/docs/dev-setup.md).

## Final Note
Have fun, it's peer-to-peer!


## License

[![License: CAL 1.0](https://img.shields.io/badge/License-CAL%201.0-blue.svg)](https://github.com/holochain/cryptographic-autonomy-license)

Copyright (C) 2019 - 2021, Holochain Foundation

This program is free software: you can redistribute it and/or modify it under the terms of the license
provided in the LICENSE file (CAL-1.0). This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
PURPOSE.
