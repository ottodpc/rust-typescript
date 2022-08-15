const path = require("path");
const copyWebpackPlugin = require("copy-webpack-plugin");

// http://localhost:8080/webpack-dev-server
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "index.js",
  },
  mode: "development",
  plugins: [
    new copyWebpackPlugin({
      patterns: [{ from: "./index.html", to: "./" }],
    }),
  ],
};
