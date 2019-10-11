import Component from "./Component";

let m: typeof import("../../pkg/react-wasm");

export async function getModule(): Promise<
  typeof import("../../pkg/react-wasm")
> {
  if (m) return m;
  m = await import("../../pkg/react-wasm");
  return m;
}

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
