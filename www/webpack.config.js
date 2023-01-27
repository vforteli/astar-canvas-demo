const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: ["./src/index.ts"],
  output: {
    filename: "index.js",
    path: path.resolve(__dirname, "dist")
  },
  module: {
    rules: [
      {
        test: /\.(ts|tsx)$/,
        use: ["ts-loader"]
      },
      {
        test: /\.wasm$/,
        type: "asset/inline"
      }
    ]
  },
  resolve: {
    extensions: [".js", ".jsx", ".tsx", ".ts"]
  },
  experiments: {
    asyncWebAssembly: true,
    topLevelAwait: true
  },
  plugins: [
    new CopyWebpackPlugin({ patterns: ['public'] })
  ],
};