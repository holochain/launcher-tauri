# How to bump Holochain and/or lair-keystore versions

1. bump the launcher version itself

   - check current verison in tauri.conf.json
   - search for this version and replace it where needed

2. search for NEW_VERSION in the repo
   All Cargo.tomls: Check the holochain release's CHANGELOG to see the sub-crate versions https://github.com/holochain/holochain/blob/main-0.2/CHANGELOG.md

3. change externalBin values in `src-tauri/tauri.conf.json` if necessary

4. rename holochain an lair-keystore binaries in `src-tauri/tauri.conf.json` if necessary

5. if required, change devhub and appstore links in package.json and update shasum values

6. If required: update holochain_client version in all Cargo.toml files. It's possible that it conflicts with other holochain dependencies. In that case holochain_client would need to be updated as well (create fork).

7. If required: update devhub_types and mere_memory_types crate version if necessary in Cargo.toml files. It's possible that it conflicts with other holochain dependencies. In that case the devhub_types would need to be updated by Matthew or you can create a fork and do it yourself.

https://github.com/holochain/launcher/tree/update-dry-run
https://github.com/holochain/launcher/commit/2f2109821feece8e1cc4033055e877ecadd84d1a

## For release:

- Make sure the launcher version does not conflict with an existing release
- merge develop into main
