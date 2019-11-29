# ReactWasmDOM

An experimental drop-in replacement for ReactDOM, written in Rust, compiled to WebAssembly.

## Status

There are currently only plans to support `renderToString`, which is already half-functional.
HTML markup should be generated (correctly), but no attributes will be rendered.

You can see a working [SSR example right here](./examples/ssr).

## Installation

`npm install react-wasm-dom`

## Usage

Just replace `renderToString` from ReactDOM with the equivalent provided function.

```tsx
import { renderToString } from "react-dom/server";

// ....

const markup = renderToString(<App />);
```

becomes

```tsx
import("react-wasm-dom/server").then(({ renderToString }) => {
  // ....

  const markup = renderToString(<App />);
});
```

## Caveats

- Only modern module bundlers are supported. Otherwise the library won't load successfully under Node

## Roadmap

- Unit Tests
- HTML attribute rendering
