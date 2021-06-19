const path = require('path');

module.exports = {
    mode: 'development',
    entry: {
        common: './src/components/index.js',
    },
    output: {
        filename: 'components.js',
        path: path.resolve(__dirname, 'dist'),
    },
    module: {
        rules: [{
            test: /.html$/i,
            use: 'html-loader',
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
};
