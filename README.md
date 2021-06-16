# Holochain Launcher

> WARNING! This package is in its early development stages. Expect breaking changes, and a much better and polished UX coming soon.

> Feedback is immensely welcome in the [issues section](https://github.com/holochain/launcher/issues).

## Installing the Launcher

0. Install `libgtksourceview`:

```bash
sudo apt-get install libgtksourceview-3.0-dev
```

> This step should not be needed once [Tauri supports it](https://github.com/tauri-apps/tauri/issues/1986).

1. Go to the [Releases page](https://github.com/holochain/launcher/releases).
2. Download the appropriate executable for your platform.
3. Execute the AppImage.

## Usage

### Preparing your UI
 
When your UI is served from the launcher, you will have the Holochain app interface available at `ws://localhost:8888`. 

You will also have the admin interface available at `ws://localhost:8889`, but its usage is discouraged as newer versions of the launcher will block any admin requests not coming from the Holochain Admin UI itself. There will be a call available in `AppWebsocket` that will prompt the user to do an action in the Holochain Admin UI, or similar (To Be Defined).

### Installing a hApp

1. Package your happ into a `.happ` file.
2. Package your UI into a `.zip` file.
3. In the "Install hApp" section of the Holochain Admin, select your `.happ` file and your `.zip` file.
4. Click install, it will take a while.
5. When it's finished installation, the happ will appear in the list of active happs.

## Data storage

The Holochain Launcher uses the same config and data locations as the main `holochain` and `lair-keystore` processes. These locations are:

- Configuration: `$CONFIG_DIR/holochain`.
  - In Ubuntu this is `$HOME/.config/holochain`.
- Data: `$DATA_DIR/holochain` and `$DATA_DIR/lair`.
  - In Ubuntu this is `$HOME/.local/share/holochain` and `$HOME/.local/share/lair`.

To reset the launcher, it is enough to remove all the folders mentioned above and start the launcher again.

## Support

Right now only Linux amd64 is supported. MacOS will be available soon. Windows support will be blocked until Holochain core supports it.

There is a known issue that prevents the UIs from opening in chromium based browsers. For now the launcher will only open UIs in Firefox.

## Developer Setup

See [Developer Setup](/dev-setup.md).

## License

[![License: CAL 1.0](https://img.shields.io/badge/License-CAL%201.0-blue.svg)](https://github.com/holochain/cryptographic-autonomy-license)

Copyright (C) 2019 - 2021, Holochain Foundation

This program is free software: you can redistribute it and/or modify it under the terms of the license
provided in the LICENSE file (CAL-1.0). This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
PURPOSE.
