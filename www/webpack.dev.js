const common = require("./webpack.config")
const { merge } = require("webpack-merge")

module.exports = merge(common, {
    mode: "development",
    target: ["web", "es5"],
    devServer: {
        port: 8080,
        open: true,
        hot: true
    }
});