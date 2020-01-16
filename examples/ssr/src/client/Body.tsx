import * as React from "react";

import Lorem from "./Lorem";
export class Body extends React.Component {
  public render() {
    return (
      <p>
        <p>App mounted</p>
        <Lorem />
        <>
          <div>Fragment1</div>
          <div>Fragment2</div>
          <div>Fragment3</div>
          <div>Fragment4</div>
        </>
      </p>
    );
  }
}
