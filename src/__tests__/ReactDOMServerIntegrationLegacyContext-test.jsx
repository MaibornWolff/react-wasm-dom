/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @emails react-core
 */

'use strict';

const ReactDOMServerIntegrationUtils = require('../utils/ReactDOMServerIntegrationTestUtils');

let PropTypes;
let React;
let ReactDOM;
let ReactDOMServer;
let ReactTestUtils;

function initModules() {
  // Reset warning cache.
  jest.resetModuleRegistry();
  PropTypes = require('prop-types');
  React = require('react');
  ReactDOM = require('react-dom');
  // ReactDOMServer = require('react-dom/server');
  ReactDOMServer = require('../../pkg/server');
  ReactTestUtils = require('react-dom/test-utils');

  // Make them available to the helpers.
  return {
    ReactDOM,
    ReactDOMServer,
    ReactTestUtils,
  };
}

const {
  resetModules,
  itThrowsWhenRendering,
} = ReactDOMServerIntegrationUtils(initModules);

describe('ReactDOMServerIntegration', () => {
  beforeEach(() => {
    resetModules();
  });

  describe('legacy context', function() {
    let PurpleContext, RedContext;
    beforeEach(() => {
      class Parent extends React.Component {
        getChildContext() {
          return {text: this.props.text};
        }
        render() {
          return this.props.children;
        }
      }
      Parent.childContextTypes = {text: PropTypes.string};

      PurpleContext = props => <Parent text="purple">{props.children}</Parent>;
      RedContext = props => <Parent text="red">{props.children}</Parent>;
    });

    it('renders class child with context', async () => {
      class ClassChildWithContext extends React.Component {
        render() {
          return <div>{this.context.text}</div>;
        }
      }
      ClassChildWithContext.contextTypes = {text: PropTypes.string};

      const res = ReactDOMServer.renderToString(
        React,
        <PurpleContext>
          <ClassChildWithContext />
        </PurpleContext>,
      );
      expect(res).toBe('<div data-reactroot=\"\">purple</div>');
    });

    it('renders stateless child with context', () => {
      function FunctionChildWithContext(props, context) {
        return <div>{context.text}</div>;
      }
      FunctionChildWithContext.contextTypes = {text: PropTypes.string};

      const res = ReactDOMServer.renderToString(
        React,
        <PurpleContext>
          <FunctionChildWithContext />
        </PurpleContext>,
      );
      expect(res).toBe('<div data-reactroot=\"\">purple</div>');
    });

    xit('renders class child without context', () => {
      class ClassChildWithoutContext extends React.Component {
        render() {
          // this should render blank; context isn't passed to this component.
          return <div>{this.context.text}</div>;
        }
      }

      const res = ReactDOMServer.renderToString(
        React,
        <PurpleContext>
          <ClassChildWithoutContext />
        </PurpleContext>,
      );
      expect(res).toBe('');
    });

    xit('stateless child without context', async render => {
      function FunctionChildWithoutContext(props, context) {
        // this should render blank; context isn't passed to this component.
        return <div>{context.text}</div>;
      }

      const e = await render(
        <PurpleContext>
          <FunctionChildWithoutContext />
        </PurpleContext>,
      );
      expect(e.textContent).toBe('');
    });

    xit('class child with wrong context', async render => {
      class ClassChildWithWrongContext extends React.Component {
        render() {
          // this should render blank; context.text isn't passed to this component.
          return <div id="classWrongChild">{this.context.text}</div>;
        }
      }
      ClassChildWithWrongContext.contextTypes = {foo: PropTypes.string};

      const e = await render(
        <PurpleContext>
          <ClassChildWithWrongContext />
        </PurpleContext>,
      );
      expect(e.textContent).toBe('');
    });

    xit('stateless child with wrong context', async render => {
      function FunctionChildWithWrongContext(props, context) {
        // this should render blank; context.text isn't passed to this component.
        return <div id="statelessWrongChild">{context.text}</div>;
      }
      FunctionChildWithWrongContext.contextTypes = {
        foo: PropTypes.string,
      };

      const e = await render(
        <PurpleContext>
          <FunctionChildWithWrongContext />
        </PurpleContext>,
      );
      expect(e.textContent).toBe('');
    });

    it('renders with context passed through to a grandchild', () => {
      function Grandchild(props, context) {
        return <div>{context.text}</div>;
      }
      Grandchild.contextTypes = {text: PropTypes.string};

      const Child = props => <Grandchild />;

      const res = ReactDOMServer.renderToString(
        React,
        <PurpleContext>
          <Child />
        </PurpleContext>,
      );
      expect(res).toBe('<div data-reactroot=\"\">purple</div>');
    });

    it('renders a child context overriding a parent context', () => {
      const Grandchild = (props, context) => {
        return <div>{context.text}</div>;
      };
      Grandchild.contextTypes = {text: PropTypes.string};

      const res = ReactDOMServer.renderToString(
        React,
        <PurpleContext>
          <RedContext>
            <Grandchild />
          </RedContext>
        </PurpleContext>,
      );
      expect(res).toBe('<div data-reactroot=\"\">red</div>');
    });

    it('renders a child context merged with a parent context', () => {
      class Parent extends React.Component {
        getChildContext() {
          return {text1: 'purple'};
        }
        render() {
          return <Child />;
        }
      }
      Parent.childContextTypes = {text1: PropTypes.string};

      class Child extends React.Component {
        getChildContext() {
          return {text2: 'red'};
        }
        render() {
          return <Grandchild />;
        }
      }
      Child.childContextTypes = {text2: PropTypes.string};

      const Grandchild = (props, context) => {
        return (
          <div>
            <div id="first">{context.text1}</div>
            <div id="second">{context.text2}</div>
          </div>
        );
      };
      Grandchild.contextTypes = {
        text1: PropTypes.string,
        text2: PropTypes.string,
      };

      const res = ReactDOMServer.renderToString(React, <Parent />);
      expect(res).toBe('<div data-reactroot=\"\"><div id="first">purple</div><div id="second">red</div></div>');
    });

    xit(
      'renders with a call to componentWillMount before getChildContext',
      () => {
        class WillMountContext extends React.Component {
          getChildContext() {
            return {text: this.state.text};
          }
          UNSAFE_componentWillMount() {
            this.setState({text: 'foo'});
          }
          render() {
            return <Child />;
          }
        }
        WillMountContext.childContextTypes = {text: PropTypes.string};

        const Child = (props, context) => {
          return <div>{context.text}</div>;
        };
        Child.contextTypes = {text: PropTypes.string};

        const res = ReactDOMServer.renderToString(React, <WillMountContext />);
        expect(res).toBe('<div data-reactroot=\"\">foo</div>');
      },
    );

    xit(
      'renders if getChildContext exists but childContextTypes is missing with a warning',
      async render => {
        function HopefulChild(props, context) {
          return context.foo || 'nope';
        }
        HopefulChild.contextTypes = {
          foo: PropTypes.string,
        };
        class ForgetfulParent extends React.Component {
          render() {
            return <HopefulChild />;
          }
          getChildContext() {
            return {foo: 'bar'};
          }
        }
        const e = await render(<ForgetfulParent />, 1);
        expect(e.textContent).toBe('nope');
      },
    );

    xit(
      'throws when rendering if getChildContext returns a value not in childContextTypes',
      render => {
        class MyComponent extends React.Component {
          render() {
            return <div />;
          }
          getChildContext() {
            return {value1: 'foo', value2: 'bar'};
          }
        }
        MyComponent.childContextTypes = {value1: PropTypes.string};
        return render(<MyComponent />);
      },
      'MyComponent.getChildContext(): key "value2" is not defined in childContextTypes.',
    );
  });
});
