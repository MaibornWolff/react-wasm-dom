# ReactWasmDOM

An experimental drop-in replacement for ReactDOM, written in Rust, compiled to
WebAssembly.

## Status

There are currently only plans to support `renderToString` / Server-side
Rendering (SSR).

You can see a working [SSR example right here](./examples/ssr).

## Installation

`react-wasm-dom` is already reserved on NPM, if we ever decide to publish it. If
you want to play around with it, you would have to clone this repositry and make
a production build. Then you can copy the package or link to it.

## Build & Run

- Install Rust with the WebAssembly toolchain and Node v16

- Install wasm-pack:

`cargo install --git https://github.com/rust-wasm/wasm-pack`

- Install dependencies

`yarn install`

- Compile library in production mode:

`yarn build`

- Navigate to the SSR example and build it:

```sh
$ cd examples/ssr
$ yarn start
```

- Navigate to http://localhost:8080

## Usage

Just replace `renderToString` from ReactDOM with the equivalent provided
function.

```tsx
import * as React from "react";
import { renderToString } from "react-dom/server";

// ....

const markup = renderToString(<App />);
```

becomes

```tsx
import * as React from "react";
import * as ReactIs from "react-is";
import("react-wasm-dom/server").then(({ renderToString }) => {
  // ....

  const markup = renderToString(React, ReactIs, <App />);
});
```

## Caveats

- Only modern module bundlers are supported. Otherwise the library won't load
  successfully under Node. Tested with Webpack 5 (see
  [SSR example](./examples/ssr)).
- the API of `renderToString` slightly differs from the original one, because
  WebAssembly by design only has access to what it gets passed to. Since we need
  to call functions from `React` and `ReactIs`, we need to pass them into the
  WebAssembly memory and generate bindings for them.

## Correctness

We want to generate the exact same HTML markup as the original ReactDOM. To
ensure this, we copied (almost) all Unit Tests from the React repository that
are relevant for SSR and slightly modified them to be runnable with this
library.

You can run all Unit Tests via `yarn test`.

## Performance

To have a realistic benchmark, we generated a special JS bundle from this
[production website](https://github.com/Tarnadas/smmdb). This bundle exports
some functions so that we can easily get all the routes that we want to render
with their respective JSX. Here is the link to [the benchmark](./bench/smmdb)
and its [result](./bench/smmdb/result.txt).

## Todos

- Wait for
  [Interface Types Proposal](https://github.com/WebAssembly/interface-types) to
  land in v8 and recheck performance.
- more Unit Tests
- more examples
