const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = [
  {
    entry: "./src/client/index.tsx",
    output: {
      path: dist,
      filename: "bundle.js"
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: "./src/index.html"
      }),

      new WasmPackPlugin({
        crateDirectory: "."
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
  },
  {
    entry: "./src/server/index.tsx",
    target: "node",
    node: {
      __dirname: false,
      __filename: false
    },
    output: {
      path: dist,
      filename: "index.js"
    },
    plugins: [
      new WasmPackPlugin({
        crateDirectory: "."
      })
    ],
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
  }
];
