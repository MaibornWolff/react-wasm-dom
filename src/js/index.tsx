import App from "./App";
import Component from "./Component";

import("../../crate/pkg/rust_webpack").then(module => module.render(<App />));

function h<P, S>(
  type: Component<P, S>,
  props: P,
  ...children: JSX.Element[]
): JSX.Element {
  return {
    type,
    props: props || {},
    children
  };
}
