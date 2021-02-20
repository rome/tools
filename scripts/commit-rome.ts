import {TEMP_PATH} from "@internal/path";
import {ROOT} from "./_utils";
import {main as buildRelease} from "./build-release";

export async function main() {
	const tempFolder = TEMP_PATH.append("vendor-rome");

	try {
		await buildRelease([tempFolder.join()]);

		const outPath = ROOT.append("scripts/vendor/rome.cjs");
		await tempFolder.append("bin/rome/index.js").copyFileTo(outPath);
		await tempFolder.append("bin/rome/index.js.map").copyFileTo(outPath.addExtension(".map"));
	} finally {
		await tempFolder.removeDirectory();
	}
}
