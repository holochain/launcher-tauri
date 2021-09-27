const path = require("path");

module.exports = {
  lintOnSave: false,
  filenameHashing: false,
  chainWebpack: (config) => {
    config.optimization.delete("splitChunks");
    config.module
      .rule("vue")
      .use("vue-loader")
      .tap((options) => ({
        ...options,
        compilerOptions: {
          // treat any tag that starts with ion- as custom elements
          isCustomElement: (tag) =>
            tag.startsWith("copyable-") ||
            tag.startsWith("mwc-") ||
            tag.startsWith("ui5-") ||
            tag.startsWith("sl-"),
        },
      }));
  },
  configureWebpack: {
    resolve: {
      alias: {
        vue$: path.resolve(
          "./node_modules/vue/dist/vue.runtime.esm-bundler.js"
        ),
        vuex: path.resolve("./node_modules/vuex/dist/vuex.esm-bundler.js"),
        "@material/mwc-ripple": path.resolve(
          "./node_modules/@material/mwc-ripple"
        ),
        "@material/mwc-button": path.resolve(
          "./node_modules/@material/mwc-button"
        ),
        "@material/mwc-icon": path.resolve("./node_modules/@material/mwc-icon"),
        "@material/mwc-dialog": path.resolve(
          "./node_modules/@material/mwc-dialog"
        ),
        "@material/mwc-snackbar": path.resolve(
          "./node_modules/@material/mwc-snackbar"
        ),
        "@material/mwc-icon-button": path.resolve(
          "./node_modules/@material/mwc-icon-button"
        ),
        "@material/mwc-textarea": path.resolve(
          "./node_modules/@material/mwc-textarea"
        ),
        "@material/mwc-textfield": path.resolve(
          "./node_modules/@material/mwc-textfield"
        ),
        "@material/mwc-fab": path.resolve("./node_modules/@material/mwc-fab"),
        "@material/mwc-circular-progress": path.resolve(
          "./node_modules/@material/mwc-circular-progress"
        ),
      },
    },
  },
};
