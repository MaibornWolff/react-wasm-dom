let React;
let ReactDOMServer;

describe('ReactDOMServer', () => {
  beforeEach(() => {
    jest.resetModules();
    React = require('react');
    ReactIs = require('react-is');
    // ReactDOMServer = require('react-dom/server');
    ReactDOMServer = require('../../pkg/server');
  });

  it('should work with React.Fragment', () => {
    class FragTest extends React.Component {
      render() {
        return <div>Fragment Class</div>
      }
    }
    let html = <div>
      <div>No Fragment</div>
      <>
        <div>Fragment 1</div>
        <div>Fragment 2</div>
        <FragTest />
        <div>Fragment 3</div>
      </>
    </div>
    let res = ReactDOMServer.renderToString(React, ReactIs, html);

    expect(res).toEqual(
      '<div data-reactroot="">' +
      '<div>No Fragment</div>' +
      '<div>Fragment 1</div>' +
      '<div>Fragment 2</div>' +
      '<div>Fragment Class</div>' +
      '<div>Fragment 3</div></div>'
    )
  });
});