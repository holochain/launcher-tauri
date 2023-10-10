# Holochain Launcher

A cross-platform executable that launches a local Holochain conductor, and lets you install and run [Holochain](https://www.holochain.org) apps.

> Feedback is welcome in the [issues section](https://github.com/holochain/launcher/issues).

## Download Holochain Launcher

| Operating System | [Latest Stable Release](https://github.com/holochain/launcher/releases/tag/v0.9.4)<br>(Holochain v0.1.3) | [Latest Unstable Release](https://github.com/holochain/launcher/releases/tag/v0.11.1)<br>(Holochain v0.2.X) | How to Install                          |
| ---------------- | --------------------- | --------------------------- | --------------------------------------- |
| Windows          | [Download](https://github.com/holochain/launcher/releases/download/v0.9.4/Holochain.Launcher_0.9.4_x64_en-US.msi)<br>          | [Download](https://github.com/holochain/launcher/releases/download/v0.11.1/Holochain.Launcher.Beta.2_0.11.1_x64_en-US.msi)  | [Instructions](docs/install-windows.md) |
| macOS | [Download](https://github.com/holochain/launcher/releases/download/v0.9.4/Holochain.Launcher_0.9.4_x64.dmg) | [Download](https://github.com/holochain/launcher/releases/download/v0.11.1/Holochain.Launcher.Beta.2_0.11.1_x64.dmg) |  [Instructions](docs/install-macos.md) |
| Linux | [.AppImage](https://github.com/holochain/launcher/releases/download/v0.9.4/holochain-launcher_0.9.4_amd64.AppImage)<br>[.deb](https://github.com/holochain/launcher/releases/download/v0.9.4/holochain-launcher_0.9.4_amd64.deb) | [.AppImage](https://github.com/holochain/launcher/releases/download/v0.11.1/holochain-launcher-beta-2_0.11.1_amd64.AppImage)<br>[.deb](https://github.com/holochain/launcher/releases/download/v0.11.1/holochain-launcher-beta-2_0.11.1_amd64.deb) | [Instructions](docs/install-linux.md) |



Notes:

- **macOS**: If you are running the Holochain Launcher from a `.app` file extracted from an `app.tar.gz` bundle instead of having it installed from a `.dmg` file, automatic updates will only succeed if the `.app` file is located in the `Applications` folder due to file permissions.
- **Linux**: Automatic updates are currently not supported when the Launcher is installed from a `.deb` file instead of runniing it from the `.AppImage`.


# For Developers
👉 [Package a Web hApp for the Launcher](#packaging-a-web-happ)<br>
👉 [Publish your Web hApp to DevHub and AppStore](#publishing-and-updating-an-app-in-the-app-store) for end-users to download from within the Launcher<br>
👉 [Use the launcher API](https://www.npmjs.com/package/@holochain/launcher-api) to send notifications to end-users (currently only available on the latest unstable release)
### Packaging a Web hApp

Requirements:

Depending in which version of the Launcher you are targeting, use an HDK and HDI version that is compatible with the Holochain version it's using.

The easiest way to create a Holochain web-app is to use holochain's [scaffolding tool](https://docs.rs/holochain_scaffolding_cli/latest/holochain_scaffolding_cli/). If you develop in [nix-shell](https://developer.holochain.org/install/), `hc scaffold` will be readily available to you to call in the command line.

When your UI is served from the launcher and `AppWebsocket.connect()` or `AdminWebsocket.connect()` are called, the call will be redirected to the actual port in which Holochain is running its interfaces. Also, when the `appWebsocket.getAppInfo({ app_id: <APP_ID> })` is called, the `APP_ID` parameter is going to be replaced with the actual application ID that is installed in the Launcher.

For now, the UI still has access to the admin interface, but _its usage is discouraged_ as newer versions of the Launcher will block any admin requests not coming from the Holochain Admin UI itself. There will be a call available in `AppWebsocket` that will prompt the user to do an action in the Holochain Admin UI, or similar (to be defined).

If you used the scaffolding tool to create your hApp, packaging it for the launcher is as easy as running `npm run package` in the root directory of your project. Otherwise you need to:

1. Package your happ into a `.happ` file, with `hc app pack`.
2. Package your UI into a `.zip` file. IMPORTANT: this `.zip` is supposed to contain an `index.html` file at its root level.
3. Create a Web-hApp manifest with `hc web-app init`.
4. Set the corrrect locations for the UI `.zip` file and the `.happ` file.
5. Package your Web-hApp with `hc web-app pack`, and publish it so that other users can download and install it or simply share it with your friends.

### Publishing and Updating an App in the App Store

In order for your app to appear in the App Store within the Holochain Launcher, you need to follow a two-step process:

1. Upload your app to the Dev Hub, which is a Holochain app that you can opt-in to install to the Launcher
2. Register your app in the App Store, a Holochain app that's installed in the Launcher by default and that contains metadata about apps available in the DevHub.

#### 1. Publishing your app in the Dev Hub

1. Go to the Settings section in the Holochain Launcher (gear icon in the top right corner).
2. Expand the "Advanced Settings" section.
3. Enable the Developer Mode, which will install the Dev Hub.
4. Click on "Publish an app" and then "Open Dev Hub".
5. Within the Dev Hub, click on "Add hApp", then choose a title, subtitle and description for your app and click on "Save".
6. Once you have created the new hApp, click on "Upload Bundle".
7. If your app has an accompanying web UI, upload the `.webhapp` file of your app, if it's a headless app, upload the `.happ` file of your app.
8. You will be prompted to choose names, descriptions and version numbers for your zomes and DNA's. Fill out all necessary fields and create the zomes and DNA's of your hApp.
9. On the same page still, you can also give a name and version number to the UI that comes with the `.webhapp` file if it's a web app. The UI will be stored separately from the `.happ` file such that you may later associate a new "official UI" to the same happ release. You will be able to associate any UI of _anyone_ to your happ release so _make sure to choose a unique enough name for the UI such that you will still recognize it amongst all other UI's_.
10. Click on "Continue" and then set the appropriate "Name" for your release (e.g. v0.0.1, _this is the version name that participants will see when downloading your application_) as well as a changelog.
11. Review and save your hApp.

If you get stuck at any point, don't hesitate to ask for help on Holochain's Discord server. You should find an invite link to it on https://developer.holochain.org/.

#### 2. Registering your app in the App Store

Now that you have uploaded your app to the DevHub, you need to register it in the App Store.

1. Still in "Advanced Settings", click again on "Publish an app", then on "Open App Store". This will open the admin UI of the App Store.
2. Before being able to add an app to the App Store, you need to add yourself as a publisher if you haven't done so earlier. To do so, click on "Add Publisher" in the top right corner, then fill out all the required fields, add an icon and click on "Save".
3. Now you can move on to add an App under the name of this Publisher by clicking on "Add App".
4. Add an Icon.
5. Paste the hApp HRL of your app that you uploaded to the Dev Hub earlier. It is the address required by end-users to request your app before installing it. You can copy the HRL from your hApp's page in the _Dev Hub_. Within the Dev Hub, you find it under "My hApps" by clicking on "More Info" next to the app that you uploaded. You should see the HRL in red font with a copy icon next to it.
6. After pasting the HRL, hit the Enter key. Most of the Details should now be auto-populated based on the information from the Dev Hub. Edit them if desired and select a Publisher (the one that you created in step 3).
7. Click on "Save".
8. Your app should now appear in the App Store in the Holochain Launcher. _To make sure that all the data you just uploaded to Dev Hub and App Store is being synchronized with other peers, keep the Launcher running for while_ (to be safe, around 30 Minutes).

If you get stuck at any point, don't hesitate to ask for help on Holochain's Discord server. You should find an invite link to it on https://developer.holochain.org/.

#### Updating the UI of your Web App

Provided that you already created a happ release in the Dev Hub as described in step 1 above, you can always update the UI associated with it. End-users that already have your app installed will then automatically be offered to switch to your new UI.

To update the UI associated to your app, follow these steps:

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
11. The UI of your app should now be updated and after (re)starting the Launcher, people will see the option to update the UI in case they already have your app installed. No actions on your side are required in the App Store admin UI.
12. And again, to make sure your uploads get gossiped in the network, keep your Launcher running until another Launcher (on another peer's computer) can install the app. To be safe, wait for about 30 minutes at least.

If you get stuck at any point, don't hesitate to ask for help on Holochain's Discord server. You should find an invite link to it on https://developer.holochain.org/.

## Data storage

The Holochain Launcher stores data in different places on the file system, depending on the platform:

- Configuration:
  - Windows: `{FOLDERID_RoamingAppData}/holochain-launcher-0.1`
  - macOS: `$HOME/Library/Application Support/holochain-launcher-0.1`
  - Linux: `$HOME/.config/holochain-launcher-0.1`.
- Data:
  - Windows: `{FOLDERID_RoamingAppData}/holochain-launcher-0.1`
  - macOS: `$HOME/Library/Application Support/holochain-launcher-0.1`
  - Linux: `$XDG_DATA_HOME/holochain-launcher` or `$HOME/.local/share/holochain-launcher-0.1`
- Logs:
  - Windows: `{FOLDERID_RoamingAppData}/holochain-launcher-0.1/profiles/${profile}/logs`
  - macOS: `$HOME/Library/Logs/holochain-launcher-0.1`
  - Linux: `$XDG_DATA_HOME/holochain-launcher-0.1/profiles/${profile}/logs` or `$HOME/.local/share/holochain-launcher-0.1/profiles/${profile}/logs`

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
