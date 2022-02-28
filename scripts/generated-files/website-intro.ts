import {ROOT, modifyGeneratedFile} from "../_utils";
import {escapeRegex} from "@internal/string-escape";

export async function main() {
	await modifyGeneratedFile(
		{
			path: ROOT.append("website", "src", "_includes", "docs", "intro.md"),
			scriptName: "generated-files/website-intro",
		},
		async () => {
			const readme = await ROOT.append("README.md").readFileText();
			const introMatch = readme.match(
				/<!-- INTRO START -->\n([\s\S]*?)\n<!-- INTRO END -->/,
			);
			if (introMatch == null) {
				throw new Error("Could not find introduction");
			}

			let intro = introMatch[1];

			// Replace absolute URLs
			intro = intro.replace(
				new RegExp(escapeRegex("https://rome.tools/#"), "g"),
				"#",
			);
			intro = intro.replace(
				new RegExp(escapeRegex("https://rome.tools/"), "g"),
				"/",
			);

			return {lines: [intro]};
		},
	);
}
