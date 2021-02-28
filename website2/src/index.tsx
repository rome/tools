import * as React from "react";
import * as ReactDOM from "react-dom";
import Router from "./Router";

const root = document.querySelector("#root")!;
ReactDOM.render(<Router url={location.pathname} />, root);