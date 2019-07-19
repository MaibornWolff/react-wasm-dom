import { h } from ".";

const App = (): JSX.Element => {
  console.log("APP");
  return (
    <div>
      <h1>Awesome Rust WASM App</h1>
      <p>App mounted</p>
    </div>
  );
};

export default App;
