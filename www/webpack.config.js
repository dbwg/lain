module.exports = {
  entry: ['./js/dash/index.js'],
  output: {
    path: __dirname + '/static/build/dash/',
    publicPath: 'build/dash/',
    filename: 'bundle.js',
  },

  module: {
    rules: [
      {
        test: /\.vue$/,
        loader: 'vue-loader',
      },
      {
        test: /\.js$/, exclude: /node_modules/,
        use: [
          {
            loader: 'babel-loader',
            options: {
              presets: ['env']
            }
          }
        ],
      },
    ],
  },
}
