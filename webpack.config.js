const path = require("path");
const fs = require("fs");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");

const outdir = path.resolve(__dirname, "./dist");

if (fs.existsSync(outdir)) {
  fs.rmSync(outdir, { recursive: true });
}

module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
  entry: {
    app: "./index.js",
  },
  output: {
    path: outdir,
    filename: "[name].js",
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({ template: "index.html" }),
    // WasmPackPlugin doesn't work in CI environment
    // thanks to https://github.com/wasm-tool/wasm-pack-plugin/issues/90
    ...(!process.env.CI
      ? [
          new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "./ffi/wasm/"),
            outDir: path.resolve(__dirname, "./ffi/wasm/pkg/"),
            forceMode: "production",
          }),
        ]
      : []),
  ],
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
      },
      {
        test: /\.ttf$/,
        use: ["file-loader"],
      },
    ],
  },
  optimization: {
    minimize: true,
    minimizer: [new TerserPlugin()],
  },
  mode: "production",
};
