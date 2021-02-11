import {AbsoluteFilePath} from "@internal/path";
import {Manifest, normalizeManifest} from "@internal/codec-js-manifest";
import {exists, readFileTextMeta} from "@internal/fs";
import {json} from "@internal/codec-config";
import {Server} from "@internal/core";

export default async function getManifest(
	server: Server,
	cwd: AbsoluteFilePath,
): Promise<Manifest | undefined> {
	const manifestPath = cwd.append("package.json");
	let manifest: undefined | Manifest;
	const projects = await server.projectManager.getProjectHierarchyFromPath(
		manifestPath,
	);
	if (await exists(manifestPath)) {
		manifest = await normalizeManifest(
			manifestPath,
			json.consumeValue(await readFileTextMeta(manifestPath)),
			projects,
		);
	}
	return manifest;
}
