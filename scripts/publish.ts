import {TEMP_PATH} from "@internal/path";
import {readFileText, removeDirectory, writeFile} from "@internal/fs";
import {exec, reporter} from "./_utils";
import {main as buildRelease} from "./build-release";
import {markup} from "@internal/markup";

const releaseFolder = TEMP_PATH.append("rome-publish-release");
const releaseManifest = releaseFolder.append("package.json");

async function setName(name: string) {
	const manifest = {
		...JSON.parse(await readFileText(releaseManifest)),
		name,
	};

	await writeFile(releaseManifest, JSON.stringify(manifest, null, "\t") + "\n");
}

async function publishRegistry(registry: string) {
	await exec(
		"npm",
		["publish", "--registry", registry],
		{cwd: releaseFolder.join()},
	);
}

export async function main() {
	try {
		reporter.heading(markup`Building release`);
		await buildRelease([releaseFolder.join()]);

		reporter.heading(markup`Publishing to registry.npmjs.com`);
		await setName("rome");
		await publishRegistry("https://registry.npmjs.org/");

		reporter.heading(markup`Publishing to npm.pkg.github.com`);
		await setName("@romefrontend/rome");
		await publishRegistry("https://npm.pkg.github.com/");
	} finally {
		await removeDirectory(releaseFolder);
	}
}
