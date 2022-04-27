import "webpack-dev-server";
import { Configuration } from "webpack";
import HtmlWebpackPlugin from "html-webpack-plugin";
import MiniCssExtractPlugin from "mini-css-extract-plugin";

const isDev = process.argv[process.argv.length - 1] === "development";

const config: Configuration = {
  resolve: {
    extensions: [".js", ".ts", ".jsx", ".tsx", ".json"],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        loader: "ts-loader",
      },
      {
        test: /\.s?css$/,
        use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
      },
      {
        test: /\.png$/,
        type: "asset/inline",
      },
    ],
  },
  plugins: [
    new MiniCssExtractPlugin(),
    new HtmlWebpackPlugin({
      templateParameters: {
        devtools: isDev
          ? '<script src="http://localhost:8097"></script>'
          : undefined,
      },
    }),
  ],
  devServer: {
    static: { directory: "./dist" },
  },
};

export default config;
