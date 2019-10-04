import * as React from "../../../../src/js";

import Lorem from "./Lorem";

export class Body extends React.Component {
  public render() {
    return (
      <p>
        <p>App mounted</p>
        <Lorem />
      </p>
    );
  }
}
