import {exec, execDev, reporter} from "./_utils";
import {TEMP_PATH} from "@romefrontend/path";
import {markup} from "@romefrontend/cli-layout";

export async function main(args: Array<string>) {
	const outFolder = TEMP_PATH.append("rome-vscode-dev").join();

	reporter.heading(markup`Bundling extension`);
	await execDev(["bundle", "@romefrontend-integration/vscode", outFolder]);

	reporter.heading(markup`Running VSCode`);
	await exec(
		"code",
		["--extensionDevelopmentPath", outFolder, "--disable-extensions", ...args],
	);
}
