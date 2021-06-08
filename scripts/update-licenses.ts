import {INTERNAL, httpsGet, writeFile} from "./_utils";
import {consumeUnknown} from "@internal/consume";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";
import data from "@internal/codec-spdx-license/data";
import {dedent} from "@internal/string-utils";

const licensesUrl = "https://raw.githubusercontent.com/spdx/license-list-data/master/json/licenses.json";
const licensesFolder = INTERNAL.append("codec-spdx-license");

export async function main() {
	const licenses = await httpsGet(licensesUrl);

	const licensesJSON = consumeUnknown(licenses, DIAGNOSTIC_CATEGORIES.parse);

	// TODO: use .asDate once it is fixed
	const newDate = new Date(licensesJSON.get("releaseDate").asString());
	const currentDate = new Date(data.releaseDate);
	if (newDate > currentDate) {
		await writeFile(
			licensesFolder.append("data.ts"),
			dedent`
				export default ${JSON.stringify(licenses)}
			`,
		);
	}
}
