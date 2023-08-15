# @lightningrodlabs/we-applet

This package contains the interfaces and contracts that a UI module needs to implement in order to become a We Applet.

## Implementing the UI for a we applet

We applets don't have an `index.html` file as their entrypoint, but an `index.js`. This `index.js` **must** have a default export that implements the interface defined by the `WeApplet` type.

To implement the UI for your applet, import the `WeApplet` type from `@lightningrodlabs/we-applet`, create an object that implements it, and have that be the default export in your file:

> index.ts.

```ts
import { DnaHash, EntryHash, AppAgentClient } from "@holochain/client";
import { WeApplet, AppletViews, WeServices, Hrl } from "@lightningrodlabs/we-applet";
import { ProfilesClient } from '@holochain-open-dev/profiles';

async function appletViews(
  client: AppAgentClient,        // The client for this applet, already set up
  appletHash: EntryHash,   // The applet instance id, usually used when opening views
  profilesClient: ProfilesClient,  // The services that this group offers, like the group's profile or the ProfilesClient for the agents
  weServices: WeServices         // The services that "we" offers to this applet, to enable attachments, open views, search...
): AppletViews {
  return {
    main: (element: HTMLElement) => element.innerHTML = "<span>This is the main view for this applet, which is going to be opened when the user clicks on the applet's icon</span>",
    blocks: {
      my_block: {
        label: "My Block",
        icon_src: "<svg>...</svg>",
        view: (element: HTMLElement) => element.innerHTML = '<span>This is a block view for this applet, which can be opened from the main view</span>'
      }
    },
    entries: {
      my_role_name: {
        my_integrity_zome_name: {
          my_entry_type_name: {
            async view(element: HTMLElement, hrl: Hrl, context) {
              const myEntry = await client.callZome({
                cell_id: [hrl[0], client.myPubKey],
                payload: hrl[1],
                /** TODO: call the appropriate zome function in your app */
              });

              element.innerHTML = `<span>The title of this entry is ${myEntry.title}</span>`
            },
            async info(hrl: Hrl) { // The HRL is a [DnaHash, ActionHash | EntryHash] pair, identifying the entry to retrieve
              const myEntry = await client.callZome({
                cell_id: [hrl[0], client.myPubKey],
                payload: hrl[1],
                /** TODO: call the appropriate zome function in your app */
              });

              if (!myEntry) return undefined;

              return {
                name: myEntry.title,
                icon_src: /** Here you can use a SVG, maybe from the @mdi/js package */
              }
            },
          }
        }
      }
    }
  }
}

const applet: WeApplet = {
  appletViews,
  async crossAppletViews: (
    applets: ReadonlyMap<EntryHash, AppletClients>, // Segmented by applet ID
    weServices: WeServices
  ) {
    return {
      main: element => {},
      blocks: {}
    }
  },
  attachmentTypes: async client => ({}),
  search: async () => [],
};

export default applet;
```

## Building

Use [rollup](https://rollupjs.org/guide/en/) to build a fully bundled javascript file that doesn't have any external imports.

This is an example configuration for it:

> rollup.config.js

```js
import nodeResolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import typescript from "@rollup/plugin-typescript";

import babel from "@rollup/plugin-babel";
import { terser } from "rollup-plugin-terser";

export default {
  input: "src/index.ts", // This needs to be pointing to the file that has the `WeApplet` default export
  output: {
    format: "es",
    dir: 'dist',
  },
  watch: {
    clearScreen: false,
  },

  plugins: [
    /** Resolve bare module imports */
    nodeResolve({
      browser: true,
      preferBuiltins: false,
    }),
    commonjs({}),
    typescript(),
    /** Minify JS */
    terser(),
    /** Compile JS to a lower language target */
    babel({
      exclude: /node_modules/,

      babelHelpers: "bundled",
      presets: [
        [
          require.resolve("@babel/preset-env"),
          {
            targets: [
              "last 3 Chrome major versions",
              "last 3 Firefox major versions",
              "last 3 Edge major versions",
              "last 3 Safari major versions",
            ],
            modules: false,
            bugfixes: true,
          },
        ],
      ],
    }),
  ],
};
```

Now you have it! You can use the generated `.js` file as a We Applet UI file.
