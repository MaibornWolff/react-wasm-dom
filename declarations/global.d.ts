declare namespace JSX {
  interface Element<P = any> {
    type: any;
    props: P;
    children: JSX.Element[];
    ref?: any;
  }

  interface ElementAttributesProperty {
    props: {};
  }

  interface ElementChildrenAttribute {
    children: {};
  }

  interface IntrinsicElements {
    div: {
      children: string;
    };
  }
}
