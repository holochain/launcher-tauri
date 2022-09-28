# Architecture

## Runtime components

- [Lair](https://github.com/holochain/lair): stores the private keys, has to be unlocked using a "master" password.
- [Holochain](https://github.com/holochain/holochain): conductor for a particular holochain version.
- Happ windows: tauri windows

```mermaid
graph TD

lair_keystore_0_0_2
holochain_0_0_152
holochain_0_0_154
launcher

holochain_0_0_152-->|lair_keystore_api|lair_keystore_0_0_2
holochain_0_0_154-->|lair_keystore_api|lair_keystore_0_0_2

launcher-->|lair_keystore_api|lair_keystore_0_0_2

launcher-.->|websocket|holochain_0_0_152
launcher-.->|websocket|holochain_0_0_154

happ_window1-->launcher
happ_window2-->launcher

happ_window1-.->|websocket|holochain_0_0_152
happ_window2-.->|websocket|holochain_0_0_154
```

### Happ tauri window

```mermaid
graph TD

subgraph tauri_window
  subgraph launcher_secure_iframe
    subgraph happ_webapp_iframe
      holochain_client
    end
  end
end
```

## Signing a Zome Call

```mermaid
sequenceDiagram

happ->>@holochain/client: callZome(cell, zome, fn, payload, cap_secret)
@holochain/client ->> @holochain/client: add nonce & expires_at if not provided?
alt no signature field and this is a launcher environment
  @holochain/client->>@holochain/client: encode payload and convert Uint8Arrays to Arrays for tauri
  @holochain/client->>launcher: invoke("sign_zome_call", ZomeCallUnsigned)
  launcher->>lair_keystore: sign_by_pub_key(...)
  lair_keystore-->>launcher: signature
  launcher-->>@holochain/client: ZomeCall (signed)
  @holochain/client->>@holochain/client: decode payload or prevent double-encoding
end

@holochain/client-->>conductor: callZome({ cell_id, zome_name, fn_name, payload, cap_secret, signature, ... })
conductor->>@holochain/client: result
@holochain/client-->>happ: result
```

## Code components

A manager is something that given a path, instantiates and manages an external process (usually a tauri sidecar binary).

High-level functions:

- lair_keystore_manager: given a path to a folder, run a lair_keystore instance in that folder and maintain an active connection to it to:
  - Sign zome calls

- holochain_manager: given a path to a config folder and a path to a data folder, run and manage a holochain instance that installs apps in that path.

- holochain_web_app_manager: given a path to a config folder and a path to a data folder, run and manage a holochain_manager for `${DATA_FOLDER}/conductor` and also the UIs for the apps installed in that conductor in `${DATA_FOLDER}/ui`.

- launcher_manager: given a path to a config folder and a path to a data folder, run and manage multiple holochain_web_app_manager for all the different supported versions and also a lair_keystore_manager.

```mermaid
classDiagram
    launcher_manager "1" --> "*" holochain_web_app_manager
    holochain_web_app_manager "1" --> "1" holochain_manager
    launcher_manager "1" --> "1" lair_keystore_manager
```
