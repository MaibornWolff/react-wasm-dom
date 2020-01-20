/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @emails react-core
 */

 'use strict';

let React;
let ReactIs;
let ReactDOMServer;

describe('escapeTextForBrowser', () => {
  beforeEach(() => {
    jest.resetModules();
    React = require('react');
    ReactIs = require('react-is');
    // ReactDOMServer = require('react-dom/server');
    ReactDOMServer = require('../../pkg/server');
  });

  it('ampersand is escaped when passed as text content', () => {
    const response = ReactDOMServer.renderToString(React, ReactIs, <span>{'&'}</span>);
    expect(response).toMatch('<span data-reactroot="">&amp;</span>');
  });

  it('double quote is escaped when passed as text content', () => {
    const response = ReactDOMServer.renderToString(React, ReactIs, <span>{'"'}</span>);
    expect(response).toMatch('<span data-reactroot="">&quot;</span>');
  });

  it('single quote is escaped when passed as text content', () => {
    const response = ReactDOMServer.renderToString(React, ReactIs, <span>{"'"}</span>);
    expect(response).toMatch('<span data-reactroot="">&#x27;</span>');
  });

  it('greater than entity is escaped when passed as text content', () => {
    const response = ReactDOMServer.renderToString(React, ReactIs, <span>{'>'}</span>);
    expect(response).toMatch('<span data-reactroot="">&gt;</span>');
  });

  it('lower than entity is escaped when passed as text content', () => {
    const response = ReactDOMServer.renderToString(React, ReactIs, <span>{'<'}</span>);
    expect(response).toMatch('<span data-reactroot="">&lt;</span>');
  });

  it('number is correctly passed as text content', () => {
    const response = ReactDOMServer.renderToString(React, ReactIs, <span>{42}</span>);
    expect(response).toMatch('<span data-reactroot="">42</span>');
  });

  it('number is escaped to string when passed as text content', () => {
    const response = ReactDOMServer.renderToString(React, ReactIs, <img data-attr={42} />);
    expect(response).toMatch('<img data-attr="42" data-reactroot=""/>');
  });

  it('escape text content representing a script tag', () => {
    const response = ReactDOMServer.renderToString(
      React,
      ReactIs,
      <span>{'<script type=\'\' src=""></script>'}</span>,
    );
    expect(response).toMatch(
      '<span data-reactroot="">&lt;script type=&#x27;&#x27; ' +
        'src=&quot;&quot;&gt;&lt;/script&gt;</span>',
    );
  });
});