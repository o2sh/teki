const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin(['index.html', 'index.css', 'assets/**/*']),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
