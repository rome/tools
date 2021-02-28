import * as React from "react"
import routes from "./routes";

export default function Router({url}: {url: string}) {
  const Route = routes.get(url);
  if (Route === undefined) {
    return <span>not found</span>;
  }

  return <Route />;
}