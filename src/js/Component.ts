import * as React from "react";

interface ComponentProps {
  children?: JSX.Element | JSX.Element[];
}

export default class Component<
  P extends Record<string, unknown> = {},
  S extends Record<string, unknown> = {}
> implements React.Component {
  public constructor(props: P & ComponentProps) {
    this.props = props;
    this.state = {} as S;
    this.refs = {};
  }

  public props: P & ComponentProps;

  public state: S;

  public context: any;

  public refs: {
    [key: string]: React.ReactInstance;
  };

  public setState<K extends keyof S>(
    state:
      | ((prevState: Readonly<S>, props: Readonly<P>) => Pick<S, K> | S | null)
      | (Pick<S, K> | S | null),
    callback?: () => void
  ): void {
    // TODO
  }

  public forceUpdate(callback?: () => void): void {
    // TODO
  }

  public render(): JSX.Element {
    throw new Error();
  }
}
