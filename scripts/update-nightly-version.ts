import {updateVersion} from "./_utils";
import {VERSION} from "@internal/core";

export async function main() {
	// Build a version number with a tag unique to the current day
	const [version] = VERSION.split("-");
	const date = new Date();
	const dateParts = [date.getFullYear(), date.getMonth(), date.getDate()];
	const newVersion = `${version}-nightly.${dateParts.join(".")}`;

	// And then update it
	await updateVersion(newVersion);
}
