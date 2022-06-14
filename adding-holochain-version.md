# Adding a new Holochain version

## If the DevHub has been upgraded

1. Change the URL for the DevHub in the `prepare` script of the `package.json`.
 
## Adding support for a new Holochain version


In the develop branch:

1. Search this repo for the text `NEW_VERSION`.
   1. Follow the instructions in each of the comments there.
2. Install the new holochain binary locally.
   1. If you are in linux, you can run:
   ```bash
   cargo install --locked --git https://github.com/guillemcordoba/holochain.git --rev f4873057ac7318fb6897690b78a4d82968253a4d holochain
   HOLOCHAIN_PATH=$(which holochain)
   cp $HOLOCHAIN_PATH src-tauri/bins/holochain-v0.0.139-x86_64-unknown-linux-gnu
   ```
3. Add the new version to `src-tauri/tauri.conf.json`
4. Change the Holochain Launcher version:
   1. You can search for all the instances of the previous version and replace them.
5. Commit the changes and make sure that the CI is green.
6. Merge to main. This should trigger the creation of a new Github release.

