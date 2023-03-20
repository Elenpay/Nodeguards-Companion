const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin")

module.exports = {
  mode: process.env.NODE_ENV || "development",
  entry: {
    popup: "./src/popup.ts"
  },
  output: {
    path: path.resolve(__dirname, "dist/"),
  },
  module: {
    rules: [
      { 
        test: /\.tsx?$/,
        loader: "ts-loader"
      },
    ]
  },
  plugins: [
    // new WasmPackPlugin({
    //   crateDirectory: path.resolve(__dirname, "node_modules", "signer"),
    //   outDir: path.resolve(__dirname, "dist", "node_modules", "signer"),
    //   outName: "signer"
    // }),
    new CopyWebpackPlugin({
      patterns: [
        { from: "static", to: "." },
      ]
    })
  ],
  devtool: "inline-source-map",
  resolve: {
    extensions: [".tsx", ".ts", ".js", ".wasm"]
  },
  devServer: {
    static: {
      directory: path.join(__dirname, 'static'),
    },
    compress: true,
    port: 9000,
    historyApiFallback: true
  },
};
