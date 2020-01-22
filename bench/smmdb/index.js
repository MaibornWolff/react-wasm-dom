const React = require('react')
const ReactIs = require('react-is')
const ReactDOMServer = require('react-dom/server')
const Benchmark = require('benchmark')

const app = require('./app').getApp()

const suite = new Benchmark.Suite()

import('react-wasm-dom/server').then(async (module) => {
  const renderToString = module.default.renderToString;
  console.warn = () => {}
  console.info = () => {}
  console.error = () => {}

  app.getLocations().forEach(location => {
    const jsx = app.jsx(location)
    suite.add(`ReactWasmDOM#renderToString#loc"${location}"`, () => renderToString(React, ReactIs, jsx))
    suite.add(`ReactDOM#renderToString#loc"${location}"`, () => ReactDOMServer.renderToString(jsx))
  })

  suite
    .on('cycle', event => {
      console.log(String(event.target));
    })
    .run({ async: true })
})

