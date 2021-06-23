#!/bin/bash

rm -rf src-tauri/bins/
mkdir src-tauri/bins

HOLOCHAIN_PATH=$(which holochain)
cp $HOLOCHAIN_PATH src-tauri/bins/holochain-x86_64-unknown-linux-gnu

LAIR_PATH=$(which lair-keystore)
cp $LAIR_PATH src-tauri/bins/lair-keystore-x86_64-unknown-linux-gnu

CADDY_PATH=$(which caddy)
cp $CADDY_PATH src-tauri/bins/caddy-x86_64-unknown-linux-gnu