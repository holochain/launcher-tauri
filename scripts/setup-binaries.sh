#!/bin/bash

rm -rf src-tauri/bins/
mkdir src-tauri/bins

TARGET_TRIPLE=$(rustc -vV | sed -n 's/^.*host: \(.*\)*$/\1/p')

HOLOCHAIN_PATH=$(which holochain)
cp $HOLOCHAIN_PATH src-tauri/bins/holochain-$TARGET_TRIPLE

LAIR_PATH=$(which lair-keystore)
cp $LAIR_PATH src-tauri/bins/lair-keystore-$TARGET_TRIPLE
