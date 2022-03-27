import 'webpack-dev-server';
import { Configuration } from 'webpack';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import MiniCssExtractPlugin from 'mini-css-extract-plugin';

const isDev = process.argv[4] === 'development';

const config: Configuration = {
  mode: isDev ? 'development' : 'production',
  resolve: {
    extensions: ['.js', '.ts', '.jsx', '.tsx', '.json'],
  },
  entry: './web/index.tsx',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        loader: 'ts-loader',
      },
      {
        test: /\.s?css$/,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: {
              sourceMap: isDev,
              importLoaders: 1,
            },
          },
          {
            loader: 'sass-loader',
            options: {
              sourceMap: isDev,
            },
          },
        ],
      },
      {
        test: /\.png$/,
        type: 'asset/inline',
      },
    ],
  },
  plugins: [
    new MiniCssExtractPlugin(),
    new HtmlWebpackPlugin({
      inject: 'body',
      template: isDev ? './web/index.dev.html' : './web/index.html',
    }),
  ],
  devServer: {
    static: ['dist'],
  },
};

export default config;
