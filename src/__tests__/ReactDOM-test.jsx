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

  it('should not render syntethic events', () => {
    const jsx = <div onClick={() => {}}></div>

    let res = ReactDOMServer.renderToString(React, ReactIs, jsx);
    expect(res).toEqual('<div data-reactroot=""></div>')
  });

  it('should rename className to class', () => {
    const jsx = <div className='123'></div>

    let res = ReactDOMServer.renderToString(React, ReactIs, jsx);
    expect(res).toEqual('<div class="123" data-reactroot=""></div>')
  });

  it('should ignore undefined style attributes', () => {
    const styles = {display: 'flex', flex: '1 0 auto', zIndex: undefined}
    const jsx = <div style={styles}></div>

    let res = ReactDOMServer.renderToString(React, ReactIs, jsx);
    expect(res).toEqual('<div style="display:flex;flex:1 0 auto" data-reactroot=""></div>')
  });

  it('should not append html comment between text and intrinsic', () => {
    const jsx = <div>1<br />2</div>

    let res = ReactDOMServer.renderToString(React, ReactIs, jsx);
    expect(res).toEqual('<div data-reactroot="">1<br/>2</div>')
  });
});