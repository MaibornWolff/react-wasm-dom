import * as React from "../../../../src/js";

import App from "./App";

import("../../../../pkg").then(module => {
  try {
    const res = module.renderToString(<App />);
    console.log(res);
  } catch (err) {
    console.error(err);
  }
});
