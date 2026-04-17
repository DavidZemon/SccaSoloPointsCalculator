const webpack = require('webpack');

module.exports = function override(config) {
  config.plugins.push(
    new webpack.ProvidePlugin({
      Buffer: ['buffer', 'Buffer'],
    }),
  );
  return config;
};

