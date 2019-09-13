import { h } from ".";

import { Body } from "./Body";

const App = (): JSX.Element => {
  console.log("APP");
  return (
    <div>
      <h1>Awesome Rust WASM App</h1>
      <Body />
    </div>
  );
};

export default App;
