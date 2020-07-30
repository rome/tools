import {exec, execDev, reporter} from "./_utils";
import {TEMP_PATH} from "@internal/path";
import {markup} from "@internal/markup";

export async function main(args: Array<string>) {
	const outFolder = TEMP_PATH.append("rome-vscode-dev").join();

	reporter.heading(markup`Bundling extension`);
	await execDev(["bundle", "vscode-rome", outFolder]);

	reporter.heading(markup`Running VSCode`);
	await exec(
		"code",
		["--extensionDevelopmentPath", outFolder, "--disable-extensions", ...args],
	);
}
