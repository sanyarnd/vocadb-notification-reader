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
    }
  },

  configureWebpack: {
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "src/main/typescript")
      }
    }
  },

  outputDir: "target/classes/static",
  lintOnSave: false,

  css: {
    sourceMap: true
  },

  devServer: {
    proxy: "http://localhost:8080"
  }
};
