/**
 * @jest-environment node
 */
const React = require('react')
const ReactIs = require('react-is')
const ReactDOMServer = require('react-dom/server')

const app = require('../app').getApp()

describe('SMMDB', () => {
  it(`renders for each location`, async () => {
    console.warn = () => {}
    const module = require('../../../pkg/server')
    const renderToString = module.renderToString

    app.getLocations().forEach(location => {
      const jsx = app.jsx(location)
      
      const expected = ReactDOMServer.renderToString(jsx)

      expect(renderToString(React, ReactIs, jsx)).toEqual(expected)
    })
  })
})
