const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: './src/index.tsx',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js',
  },
  plugins: [
    new webpack.ProvidePlugin({'process': 'process'})
  ],
  module: {
    rules: [ {
        test: /\.(jpe?g|png|gif|svg|wasm)$/i,
        use: `file-loader`
      }, {
        test: /\.(scss|sass)$/i,
        use: [
          {
            loader: `style-loader`
          }, {
            loader: `css-loader`
          }, {
            loader: `sass-loader`
          }
        ]
      }, {
        test: /\.(css)$/i,
        use: [
          {
            loader: `style-loader`
          }, {
            loader: `css-loader`
          }
        ]
      }, {
        test: /\.tsx?$/,
        loader: "awesome-typescript-loader"
      }
    ]
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js", ".json"],
    alias: {
      "@types/react": path.resolve(__dirname, `./node_modules/@types/react`),
      "@types/react-dom": path.resolve(__dirname, `./node_modules/@types/react-dom`),
      "react": path.resolve(__dirname, `./node_modules/react`),
      "react-dom": path.resolve(__dirname, `./node_modules/react-dom`),
    },
    fallback: {
      "path": require.resolve("path-browserify"),
      util: require.resolve("util/"),
      process: require.resolve("process")
    }
  }
};