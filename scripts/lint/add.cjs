/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require("../_setup.cjs");

const {write} = require("../_utils.cjs");
const {lintRulesFolder, descriptionsFile} = require("../_constants.cjs");
const path = require("path");
const fs = require("fs");

const ruleName = process.argv[2];
const category = process.argv[3];
if (ruleName === undefined || category === undefined) {
	console.error("node scripts/lint/add.cjs [ruleName] [category]");
	process.exit(1);
}

const spacedName = ruleName.replace(/([A-Z+])/g, " $1").trim().toLowerCase();
const descriptionKey = spacedName.toUpperCase().replace(/ /g, "_");
const categoryName = `lint/${ruleName}`;

const ruleLoc = path.join(lintRulesFolder, category, `${ruleName}.ts`);
const testLoc = path.join(lintRulesFolder, category, `${ruleName}.test.ts`);

write(
	ruleLoc,
	`import {Path, TransformExitResult} from "@romejs/js-compiler";
import {descriptions} from "@romejs/diagnostics";

export default {
 name: "${ruleName}",
 enter(path: Path): TransformExitResult {
	 const {node} = path;

	 if (false) {
			path.context.addNodeDiagnostic(
				node,
				descriptions.LINT.${descriptionKey},
			);
	 }

	 return node;
 },
};
`,
);

write(
	testLoc,
	`import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"${spacedName}",
	async (t) => {
		await testLintMultiple(
			t,
			[
				// INVALID
				"",
				// VALID
				"",
			],
			{category: "${categoryName}"},
		);
	},
);
`,
);

// Add description
let descriptions = fs.readFileSync(descriptionsFile, "utf8");
descriptions = descriptions.replace(
	"LINT: {",
	`LINT: {\n		${descriptionKey}: {
			category: "${categoryName}",
			message: "INSERT MESSAGE HERE",
		},`,
);
write(descriptionsFile, descriptions);

require("./update.cjs");
