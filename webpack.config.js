const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry: "./src/js/index.tsx",
  output: {
    path: dist,
    filename: "bundle.js"
  },
  devServer: {
    contentBase: dist
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "index.html"
    }),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "crate")
      // WasmPackPlugin defaults to compiling in "dev" profile. To change that, use forceMode: 'release':
      // forceMode: 'release'
    })
  ],
  resolve: {
    extensions: [".ts", ".tsx", ".js", ".jsx", ".json", ".wasm"]
  },
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
                      browsers: [
                        "edge >= 17",
                        "ff >= 61",
                        "chrome >= 63",
                        "safari >= 11.1"
                      ]
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
                    isTSX: true,
                    jsxPragma: "h"
                  }
                ],
                "@babel/plugin-syntax-dynamic-import",
                [
                  "@babel/plugin-transform-react-jsx",
                  {
                    pragma: "h",
                    pragmaFrag: "Fragment"
                  }
                ]
              ]
            }
          }
        ]
      }
    ]
  }
};
