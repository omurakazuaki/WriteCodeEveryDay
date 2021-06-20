const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: 'development',
  entry: {
    components: './src/components/index.js',
    index: './src/index.tsx'
  },
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, 'dist'),
  },
  module: {
    rules: [{
      test: /.html$/i,
      use: 'html-loader',
    },
    {
      test: /\.tsx?$/,
      use: 'ts-loader',
    },
    {
      test: /.scss$/,
      use: [
        'raw-loader',
        {
          loader: 'sass-loader',
          options: {
            sassOptions: {
               includePaths: [path.resolve(__dirname, 'node_modules')],
            }
          }
        }
      ]
    }],
  },
  plugins: [
    new HtmlWebpackPlugin({
      filename: 'index.html',
      template: './index.html',
    }),
  ],
};
