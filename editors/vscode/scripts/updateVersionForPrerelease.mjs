import { readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const manifestPath = resolve(join("package.json"));

function pad(date) {
	if (date < 10) {
		return `0${date}`;
	}
	return `${date}`;
}

// read the package.json file
readFile(manifestPath, "utf8")
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
			await writeFile(manifestPath, JSON.stringify(manifest, null, "\t"));
			console.log(`version=${manifest.version}`);
		} catch (_e) {
			console.log(`Could not write the package.json file at ${manifestPath}`);
			process.exit(1);
		}
	})
	.catch(() => {
		console.log(`Could not read the package.json file at ${manifestPath}`);
		process.exit(1);
	});
