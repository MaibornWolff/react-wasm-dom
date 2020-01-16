import * as React from "react";
import * as ReactIs from "react-is";
import * as express from "express";
import * as fs from "fs";
import * as path from "path";

import App from "../client/App";

const app = express();

const index: string[] = fs
  .readFileSync(path.join(__dirname, "../index.html"), {
    encoding: "utf8"
  })
  .split('<div id="root"></div>');

import("../../../../pkg/server").then(module => {
  app.get("/", (_req: express.Request, res: express.Response) => {
    const renderedHtml = module.renderToString(React, ReactIs, <App />);
    const html = index[0] + renderedHtml + index[1];
    res.send(html);
  });

  app.listen(8080, () => {
    console.info("Server listening on port 8080");
  });
});
