import {PUBLIC_PACKAGES, ROOT, execDev} from "./_utils";
import {runNPMVersion} from "./update-version";
import {VERSION} from "@internal/core";

export async function main() {
	// Build a version number with a tag unique to the current day
	const [version] = VERSION.split("-");
	const date = new Date();
	const dateParts = [date.getFullYear(), date.getMonth(), date.getDate()];
	const newVersion = `${version}-nightly.${dateParts.join(".")}`;

	// And then update it
	await runNPMVersion([newVersion], ROOT);

	// Build a release to the dist folder in the root
	await execDev([
		"bundle",
		PUBLIC_PACKAGES.append("rome").join(),
		ROOT.append("dist").join(),
		"--set-version",
		newVersion,
	]);
}
