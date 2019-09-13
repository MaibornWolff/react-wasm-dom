import { h } from ".";

import Component from "./Component";
import Lorem from "./Lorem";

export class Body extends Component {
  public render() {
    return (
      <p>
        <p>App mounted</p>
        <Lorem />
      </p>
    );
  }
}
