const path = require("path")
const HtmlWebpackPlugin = require("html-webpack-plugin")
const webpack = require("webpack")
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin")

module.exports = {
	entry: "./js/index.ts",
	module: {
		rules: [
			{
				test: /\.tsx?$/,
				use: "ts-loader",
				exclude: /node_modules/,
			},
		],
	},
	output: {
		path: path.resolve(__dirname, "dist"),
		filename: "index.js",
	},
	plugins: [
		new HtmlWebpackPlugin(),
		new WasmPackPlugin({
			crateDirectory: path.resolve(__dirname, "rust"),
		}),
	],
	resolve: {
		extensions: [".tsx", ".ts", ".js"],
	},
	experiments: {
		asyncWebAssembly: true,
	},
	mode: "development",
}
