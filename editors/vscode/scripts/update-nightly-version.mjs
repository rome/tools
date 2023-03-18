import { readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const EXTENSION_ROOT = resolve(fileURLToPath(import.meta.url), "../..");
const MANIFEST_PATH = resolve(EXTENSION_ROOT, "package.json");

function pad(date) {
	if (date < 10) {
		return `0${date}`;
	}
	return `${date}`;
}

// read the package.json file
readFile(MANIFEST_PATH, "utf8")
	.then(async (value) => {
		const manifest = JSON.parse(value);
		const currentVersion = manifest.version;
		const versionAsSemver = currentVersion.split(".");
		// first one is the major
		const currentMajor = parseInt(versionAsSemver[0]);
		// second one is the minor
		const currentMinor = parseInt(versionAsSemver[1]);

		const date = new Date();
		const newMinor = currentMinor + 1;
		const newPatch = [
			pad(date.getFullYear()),
			pad(date.getMonth() + 1),
			pad(date.getDate()),
		].join("");
		// update the version field
		manifest.version = `${currentMajor}.${newMinor}.${newPatch}`;
		try {
			await writeFile(MANIFEST_PATH, JSON.stringify(manifest, null, "\t"));
			console.log(`version=${manifest.version}`);
		} catch {
			console.log(`Could not write the package.json file at ${MANIFEST_PATH}`);
			process.exit(1);
		}
	})
	.catch(() => {
		console.log(`Could not read the package.json file at ${MANIFEST_PATH}`);
		process.exit(1);
	});
