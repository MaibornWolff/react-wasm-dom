import Component from "./Component";

export function createElement<
  P = any,
  T extends string | React.JSXElementConstructor<any> =
    | string
    | React.JSXElementConstructor<any>
>(
  type: React.Factory<P>,
  props: P,
  ...children: React.ReactNode[]
): React.ReactNode {
  return {
    type,
    props: props || {},
    children,
    key: null
  };
}

export { Component };
