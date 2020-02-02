const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  plugins: [
    new HtmlWebpackPlugin(),
    new WasmPackPlugin({ crateDirectory: path.resolve(__dirname, "crate") })
  ]
};

// vim: set ts=2 sw=2 et:
