interface ComponentProps {
  children?: JSX.Element | JSX.Element[];
}

export default class Component<
  P extends Record<string, unknown> = {},
  S extends Record<string, unknown> = {}
> {
  public constructor(props: P & ComponentProps) {
    this.props = props;
    this.state = {} as S;
  }

  public props: P & ComponentProps;

  protected state: S;

  public render(): JSX.Element {
    throw new Error();
  }
}
