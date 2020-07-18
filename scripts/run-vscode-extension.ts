import {exec, execDev, reporter} from "./_utils";
import {TEMP_PATH} from "@romefrontend/path";

export async function main(args: Array<string>) {
	const outFolder = TEMP_PATH.append("rome-vscode-dev").join();

	reporter.heading("Bundling extension");
	await execDev(["bundle", "@romefrontend-integration/vscode", outFolder]);

	reporter.heading("Running VSCode");
	await exec(
		"code",
		["--extensionDevelopmentPath", outFolder, "--disable-extensions", ...args],
	);
}
