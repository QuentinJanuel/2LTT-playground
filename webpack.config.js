const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const internalIP = require("internal-ip");

module.exports = {
	entry: "./src/index.tsx",
	output: {
		path: path.join(__dirname, "dist"),
		filename: "bundle.js",
	},
	devServer: {
		static: path.join(__dirname, "public"),
		port: 3000,
		open: true,
		host: internalIP.v4.sync(),
	},
	mode: "development",
	devtool: "source-map",
	module: {
		rules: [
			{
				test: /\.tsx?$/,
				use: "ts-loader",
				exclude: /node_modules/,
			},
			{
				test: /\.scss$/,
				use: ["style-loader", "css-loader", "sass-loader"],
			},
			{
				test: /\.(png|jpg)$/,
				use: "url-loader",
			},
			{
				test: /\.svg$/,
				use: "@svgr/webpack",
			},
		],
	},
	resolve: {
		extensions: [".js", ".ts", ".tsx", ".scss"],
	},
	plugins: [
		new CopyWebpackPlugin({
			patterns: [
				{ from: "public", to: "" },
			],
		}),
	],
	experiments: {
		asyncWebAssembly: true,
	},
};
