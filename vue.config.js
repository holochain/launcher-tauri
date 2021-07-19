const path = require("path");

module.exports = {
  lintOnSave: false,
  configureWebpack: {
    resolve: {
      alias: {
        vue$: path.resolve("./node_modules/vue/dist/vue.runtime.esm.js"),
      },
    },
  },
};
