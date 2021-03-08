import ReactDOMServer from "react-dom/server";
import routes from "./src/routes";
import App from "./src/App";
import {createAbsoluteFilePath} from "@internal/path";

export async function main() {
	const root = createAbsoluteFilePath(__dirname);
	const buildDirectory = root.append("build");
	const staticDirectory = root.append("build");
	const indexPath = root.append("build");

	await buildDirectory.createDirectory();

	for (const route of routes.keys()) {
		const routeBuildPath = buildDirectory.append(route, "index.html");
		//await indexPath.copyFileTo(routeBuildPath);
		console.log(routeBuildPath.join());
	}

	ReactDOMServer;
	staticDirectory;
	indexPath;
	App;
}
