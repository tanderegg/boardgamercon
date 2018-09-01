exports.config = {
  // See http://brunch.io/#documentation for docs.
  files: {
    javascripts: {
      joinTo: "assets/js/app.js"
    },
    stylesheets: {
      joinTo: "assets/css/app.css"
    }
  },
  conventions: {
  // This option sets where we should place non-css and non-js assets in.
  // By default, we set this to "/assets/static". Files in this directory
  // will be copied to `paths.public`, which is set below to "../public".
    assets: /^(assets)/
  },
  // paths configuration
  paths: {
    // Dependencies and current project directories to watch
    watched: ["elm", "css", "js", "assets"],
    // Where to compile files to
    public: "../api/static"
  },
  // Configure your plugins
  plugins: {
    babel: {
      // Do not use ES6 compiler in vendor code
      ignore: [/vendor/]
    },
    elmBrunch: {
      elmFolder: ".",
      mainModules: ["elm/Main.elm"],
      outputFolder: "js",
      outputFile: "main.js"
    },
  },
  modules: {
    autoRequire: {
      "assets/js/app.js": ["js/app"]
    }
  }
};
