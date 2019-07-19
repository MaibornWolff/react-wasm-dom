declare namespace JSX {
  interface Element<P = any> {
    type: any;
    props: P;
    children: Children;
    ref?: any;
  }

  interface ElementAttributesProperty {
    props: {};
  }

  interface ElementChildrenAttribute {
    children: {};
  }

  interface IntrinsicElements {
    div: IntrinsicElement;
    p: IntrinsicElement;
    span: IntrinsicElement;
    h1: IntrinsicElement;
    h2: IntrinsicElement;
    h3: IntrinsicElement;
    h4: IntrinsicElement;
  }

  type Children = (JSX.Element | string)[] | (JSX.Element | string);

  interface IntrinsicElement {
    children: Children;
  }
}
