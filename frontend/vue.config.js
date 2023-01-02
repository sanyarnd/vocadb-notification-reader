// eslint-disable-next-line @typescript-eslint/no-var-requires
const path = require("path");

module.exports = {
  pages: {
    home: {
      title: "VocaDB Notification Reader",
      entry: "src/main/typescript/pages/home/main.ts"
    },
    login: {
      title: "Login",
      entry: "src/main/typescript/pages/login/main.ts"
    },
    index: {
      title: "VocaDB Notification Reader",
      entry: "src/main/typescript/pages/home/main.ts"
    }
  },

  configureWebpack: {
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "src/main/typescript")
      }
    }
  },

  lintOnSave: false,

  css: {
    sourceMap: true
  },

  devServer: {
    proxy: "http://localhost:8080"
  }
};
