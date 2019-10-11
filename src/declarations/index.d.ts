import * as React from "react";

export class Component extends React.Component {}
export const getModule: () => Promise<typeof import("./react-wasm")>;
