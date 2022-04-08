# Adding a new Holochain version

In the develop branch:

1. Search this repo for the text `NEW_VERSION`.
2. Follow the instructions in each of the comments there.
3. Change the Holochain Launcher version:
   1. You can search for all the instances of the previous version and replace them.
4. Commit the changes and make sure that the CI is green.
5. Merge to main. This should trigger the creation of a new Github release.
