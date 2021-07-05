import {AbsoluteFilePath} from "@internal/path";
import {Manifest, normalizeManifest} from "@internal/codec-js-manifest";
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
	let checkDependenciesAndLicense = false;
	const mainProject = await server.projectManager.findLoadedProject(cwd);
	if (mainProject) {
		checkDependenciesAndLicense = mainProject.config.dependencies.enabled;
	}
	if (await manifestPath.exists()) {
		manifest = await normalizeManifest({
			path: manifestPath,
			consumer: json.consumeValue(await manifestPath.readFileTextMeta()),
			projects,
			checkDependenciesAndLicense,
		});
	}
	return manifest;
}
