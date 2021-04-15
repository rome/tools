import * as React from "react";
import Homepage from "./pages/Homepage";

type Route = React.FunctionComponent;

type Routes = Map<string, Route>;

const routes: Routes = new Map();
routes.set("/", Homepage);
export default routes;
