import {TEMP_PATH} from "@internal/path";
import {copyFile, removeDirectory} from "@internal/fs";
import {ROOT} from "./_utils";
import {main as buildRelease} from "./build-release";

export async function main() {
	const tempFolder = TEMP_PATH.append("vendor-rome");

	try {
		await buildRelease([tempFolder.join()]);

		const outPath = ROOT.append("scripts/vendor/rome.cjs");
		await copyFile(tempFolder.append("bin/rome/index.js"), outPath);
		await copyFile(
			tempFolder.append("bin/rome/index.js.map"),
			outPath.addExtension(".map"),
		);
	} finally {
		await removeDirectory(tempFolder);
	}
}
