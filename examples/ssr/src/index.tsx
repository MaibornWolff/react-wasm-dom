import App from "./App";
import Component from "./Component";

import("../../../pkg/react_wasm").then(module => {
  try {
    const res = module.renderToString(<App />)
    console.log(res);
  } catch (err) {
    console.error(err)
  }
});

export function h<
  P extends Record<string, unknown>,
  S extends Record<string, unknown>
>(type: Component<P, S>, props: P, ...children: JSX.Element[]): JSX.Element {
  return {
    type,
    props: props || {},
    children
  };
}
