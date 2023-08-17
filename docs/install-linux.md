# Install Holochain Launcher on Linux

## AppImage

1. Download the `.AppImage` file.
2. Double-click on it to run it (you may have to give executable permissions to the `AppImage` file).

**Note:** On Ubuntu 22.04 you may need to install `libfuse2` in order to be able to run AppImage files:
(https://askubuntu.com/questions/1403811/appimage-on-ubuntu-22-04)

```
sudo apt install libfuse2
```

## Deb

**Note:** Automatic updates are currently not supported for the Holochain Launcher installed with a `.deb` file.

1. Upgrade your dependencies by running `sudo apt-get update && sudo apt-get upgrade` in a terminal.
2. Download the `.deb` file.
3. Install it by running `sudo apt install ./path/to/downloaded/file` in the terminal (using the actual path to the downloaded file).
