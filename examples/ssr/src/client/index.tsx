import * as React from "../../../../pkg";

import App from "./App";

import("../../../../pkg").then(module => {
  try {
    const res = module.renderToString(<App />);
    console.log(res);
  } catch (err) {
    console.error(err);
  }
});
