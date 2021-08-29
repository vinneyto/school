import path from 'path';
import CompressionPlugin from 'compression-webpack-plugin';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import { CleanWebpackPlugin } from 'clean-webpack-plugin';
import FriendlyErrorsWebpackPlugin from 'friendly-errors-webpack-plugin';
import { Plugin } from 'webpack';

const debug = process.env.NODE_ENV !== 'production';

const plugins: Plugin[] = [
  new HtmlWebpackPlugin({
    title: 'xr',
    favicon: './assets/favicon.ico',
    meta: {
      viewport: 'width=device-width, initial-scale=1, shrink-to-fit=no',
    },
  }),
  new CleanWebpackPlugin(),
  new FriendlyErrorsWebpackPlugin(),
];

if (!debug) {
  plugins.push(
    new CompressionPlugin({
      test: /\.js/,
    })
  );
}

export default {
  mode: debug ? 'development' : 'production',
  context: path.resolve('./src'),
  entry: {
    main: './index.ts',
  },
  output: {
    filename: debug ? '[name].js' : '[name].[chunkhash].js',
    path: path.resolve('./dist'),
  },
  resolve: {
    extensions: ['.ts', '.js'],
  },
  module: {
    rules: [
      { test: /\.ts/, loader: 'ts-loader' },
      { test: /\.glsl/, loader: 'raw-loader' },
      { test: /\.gltf/, loader: 'gltf-webpack-loader' },
      { test: /\.bin|png|svg|jpg|gif|glb/, loader: 'file-loader' },
    ],
  },
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    port: 9001,
    open: true,
    noInfo: true,
  },
  plugins,
  optimization: {
    splitChunks: {
      chunks: 'all',
    },
    runtimeChunk: true,
  },
};
