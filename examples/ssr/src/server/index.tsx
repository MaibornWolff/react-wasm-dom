import * as express from "express";
import * as fs from "fs";
import * as path from "path";

import * as React from "../../../../src/js";
import App from "../client/App";

const app = express();
console.log("DIRNAME", __dirname);
const index: string[] = fs
  .readFileSync(path.join(__dirname, "index.html"), {
    encoding: "utf8"
  })
  .split("<body></body>");

import("../../../../pkg").then(module => {
  app.get("/", (req: express.Request, res: express.Response) => {
    const renderedHtml = module.renderToString(<App />);
    const html = index[0] + "<body>" + renderedHtml + "</body>";
    res.send(html);
  });

  app.listen(8080, () => {
    console.info("Server listening on port 8080");
  });
});
