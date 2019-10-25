const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const dist = path.resolve(__dirname, "dist");
const distServer = path.resolve(dist, "server");

module.exports = [
  {
    entry: "./src/client/index.tsx",
    output: {
      path: dist,
      filename: "index.js"
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: "src/index.html"
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
                      isTSX: true
                    }
                  ],
                  "@babel/plugin-syntax-dynamic-import",
                  "@babel/plugin-transform-react-jsx"
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
      path: distServer,
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
                  "@babel/plugin-transform-react-jsx"
                ]
              }
            }
          ]
        }
      ]
    }
  }
];
