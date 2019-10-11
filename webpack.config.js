const path = require("path");

const pkg = path.resolve(__dirname, "pkg");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = [
  // generateConfig({
  //   target: "web",
  //   filename: "index.js",
  //   wasmPackPlugin: new WasmPackPlugin({
  //     crateDirectory: ".",
  //     outName: "react-wasm"
  //   }),
  //   copyWebpackPlugin: new CopyWebpackPlugin([
  //     {
  //       from: path.join(pkg, "package.json"),
  //       to: path.join(pkg, "package.json"),
  //       transform(content, path) {
  //         let package = JSON.parse(content);
  //         package.main = "index.js";
  //         package.module = "index.js";
  //         package.types = "index.d.ts";
  //         package.files.push("index.js");
  //         package.files.push("index.d.ts");
  //         return JSON.stringify(package, undefined, 2);
  //       }
  //     }
  //   ]),
  //   declarations: new CopyWebpackPlugin([
  //     {
  //       from: path.join(__dirname, "src", "declarations", "index.d.ts"),
  //       to: path.join(pkg, "index.d.ts")
  //     }
  //   ]),
  //   targets: {
  //     browsers: ["edge >= 17", "ff >= 61", "chrome >= 63", "safari >= 11.1"]
  //   }
  // }),
  generateConfig({
    target: "node",
    node: {
      __dirname: false,
      __filename: false
    },
    filename: "index.js", //filename: "server.js",
    wasmPackPlugin: new WasmPackPlugin({
      crateDirectory: ".",
      outName: "react-wasm", //outName: "react-wasm-server",
      extraArgs: "--target nodejs"
    }),
    copyWebpackPlugin: new CopyWebpackPlugin([
      {
        from: path.join(pkg, "package.json"),
        to: path.join(pkg, "package.json"),
        transform(content, path) {
          let package = JSON.parse(content);
          package.main = "index.js";
          package.module = "index.js";
          package.types = "index.d.ts";
          package.files.push("index.js"); //package.files.push("server.js");
          package.files.push("index.d.ts"); //package.files.push("server.d.ts");
          return JSON.stringify(package, undefined, 2);
        }
      }
    ]),
    declarations: new CopyWebpackPlugin([
      {
        from: path.join(__dirname, "src", "declarations", "index.d.ts"),
        to: path.join(pkg, "index.d.ts") // to: path.join(pkg, "server.d.ts")
      }
    ]),
    targets: {
      node: "10"
    }
  })
];

function generateConfig({
  target,
  node,
  filename,
  wasmPackPlugin,
  copyWebpackPlugin,
  declarations,
  targets
}) {
  return {
    entry: "./src/js/index.ts",
    target,
    node,
    output: {
      path: pkg,
      filename,
      library: "React",
      libraryTarget: "umd"
    },
    mode: "development",
    plugins: [wasmPackPlugin, copyWebpackPlugin, declarations],
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
                      targets,
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
  };
}
