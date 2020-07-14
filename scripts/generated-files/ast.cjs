/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require("../_setup.cjs");

const path = require("path");
const fs = require("fs");

const {readGeneratedFile, write} = require("../_utils.cjs");

const {
	formatterFolder,
	analysisFolder,
	astFolder,
} = require("../_constants.cjs");

let defs = [];

for (const language of fs.readdirSync(astFolder)) {
	const languageLoc = path.join(astFolder, language);

	if (fs.statSync(languageLoc).isFile()) {
		continue;
	}

	for (const category of fs.readdirSync(languageLoc)) {
		const categoryLoc = path.join(languageLoc, category);

		if (fs.statSync(categoryLoc).isFile()) {
			continue;
		}

		for (const basename of fs.readdirSync(categoryLoc)) {
			const nodeType = path.basename(basename, path.extname(basename));
			defs.push({
				category,
				language,
				nodeType,
			});
		}
	}
}

defs = defs.sort((a, b) => {
	return a.nodeType.localeCompare(b.nodeType);
});

function readIndexFile(loc, handlers, raw) {
	let file = readGeneratedFile(loc, raw);

	for (const {iterator, wrapCallback} of handlers) {
		let buff = "";

		for (const def of defs) {
			const defBuff = iterator(def);
			if (defBuff) {
				buff += defBuff;
			}
		}

		if (wrapCallback) {
			buff = wrapCallback(buff);
		}

		file += buff;

		file = file.trim();
		file += "\n\n";
	}

	file = file.trim();
	file += "\n";

	write(loc, file);
}

// Add to ast index
readIndexFile(
	path.join(astFolder, "index.ts"),
	[
		{
			iterator({language, category, nodeType}) {
				return `export * from "./${language}/${category}/${nodeType}";\n`;
			},
		},
	],
);
readIndexFile(
	path.join(astFolder, "index.ts"),
	[
		{
			iterator(def) {
				return `\n	| n.${def.nodeType}`;
			},
			wrapCallback(buff) {
				return `export type AnyNode = ${buff};`;
			},
		},
	],
	true,
);

// Add to builders
readIndexFile(
	path.join(formatterFolder, "index.ts"),
	[
		{
			iterator({language, category, nodeType}) {
				return `import ${nodeType} from "./${language}/${category}/${nodeType}";\nbuilders.set("${nodeType}", ${nodeType});\n\n`;
			},
		},
	],
);

// Add to analysis
readIndexFile(
	path.join(analysisFolder, "index.ts"),
	[
		{
			iterator({language, category, nodeType}) {
				if (language === "js") {
					return `import ${nodeType} from "./${category}/${nodeType}";\nevaluators.set("${nodeType}", ${nodeType});\n\n`;
				} else {
					return "";
				}
			},
		},
	],
);
