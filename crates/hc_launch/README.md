# Holochain hc launch CLI

CLI to launch holochain apps in a sandboxed Holochain Launcher environment for testing and development purposes.

## Install

This CLI is part of the holochain development environment (holonix) and therein callable via `hc launch`.<br><br>

If you prefer to install it globally:

```
cargo install holochain_cli_launch
```

## Example Usage

- Launch a .webhapp with 2 agents communicating over mdns network and initializing lair-keystore "on-the-fly"
  by piping a password through on the command line:

```bash
echo pass | hc-launch --piped -n 2 path/to/my/app.webhapp network mdns
```

<br>

- Launch a .happ with the UI assets specified with the `--ui-path` option for 2 agents communicating over mdns network, initializing lair-keystore "on-the-fly"
  by piping a password through on the command line and watching for file changes in the specified UI path:

```bash
echo pass | hc-launch --piped -n 2 path/to/my/app.happ --ui-path path/to/my/ui/assets --watch network mdns
```

<br>

- Launch a .happ while pointing to a port on localhost from where you serve the UI assets:<br>
  **Note:** This mode is only meant for development purposes! Serving assets from localhost will result in different
  behavior than running the app in the real Holochain Launcher.

```bash
echo pass | hc-launch --piped -n 2 path/to/my/app.happ --ui-port 5500 network mdns
```

<br>

## Documentation

Documentation is on https://docs.rs/holochain_cli_launch
