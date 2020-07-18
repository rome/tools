import {TEMP_PATH} from "@romefrontend/path";
import {copyFile, removeDirectory} from "@romefrontend/fs";
import {ROOT} from "./_utils";
import {main as buildRelease} from "./build-release";

export async function main() {
	const tempFolder = TEMP_PATH.append("vendor-rome");

	try {
		buildRelease([tempFolder.join()]);

		const outPath = ROOT.append("scripts/vendor/rome.cjs");
		copyFile(tempFolder.append("bin/rome/index.js"), outPath);
		copyFile(
			tempFolder.append("bin/rome/index.js.map"),
			outPath.addExtension(".map"),
		);
	} finally {
		await removeDirectory(tempFolder);
	}
}
