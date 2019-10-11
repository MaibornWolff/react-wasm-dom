const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = [
  {
    entry: "./src/server/index.tsx",
    target: "node",
    node: {
      __dirname: true,
      __filename: false
    },
    output: {
      path: dist,
      filename: "index.js"
    },
    resolve: {
      extensions: [".ts", ".tsx", ".js", ".jsx", ".json", ".wasm"]
    },
    externals: [require("webpack-node-externals")()],
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          exclude: /node_modules/,
          use: [
            {
              loader: "babel-loader",
              query: {
                babelrc: false,
                presets: [
                  [
                    "@babel/env",
                    {
                      targets: {
                        node: "current"
                      },
                      useBuiltIns: "usage",
                      modules: false
                    }
                  ]
                ],
                plugins: [
                  [
                    "@babel/plugin-transform-typescript",
                    {
                      isTSX: true
                    }
                  ],
                  "@babel/plugin-syntax-dynamic-import",
                  ["@babel/plugin-transform-react-jsx"]
                ]
              }
            }
          ]
        }
      ]
    }
  }
];
