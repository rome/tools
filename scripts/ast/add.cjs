/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require("../_setup.cjs");

const path = require("path");
const fs = require("fs");

const {
	formatterFolder,
	analysisFolder,
	astFolder,
} = require("../_constants.cjs");

const {write, getBuilderName} = require("../_utils.cjs");

const language = process.argv[2];
const nodeType = process.argv[3];
const category = process.argv[4];
if (language === undefined || nodeType === undefined || category === undefined) {
	console.error("node scripts/ast/add.cjs [language] [node-type] [category]");
	process.exit(1);
}

const builderName = getBuilderName(nodeType);

// Write AST def
let file = `import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type ${nodeType} = NodeBaseWithComments & {
	type: "${nodeType}";
};

export const ${builderName} = createBuilder<${nodeType}>("${nodeType}", {
	bindingKeys: {},
	visitorKeys: {},
});
`;
const fileLoc = path.join(astFolder, language, category, `${nodeType}.ts`);
if (fs.existsSync(fileLoc, "utf8")) {
	console.log("Already have", nodeType);
	process.exit();
}
write(fileLoc, file);

// Write builder
const builderDefFile = path.join(
	formatterFolder,
	language,
	category,
	`${nodeType}.ts`,
);
const builderContent = `import {${nodeType}} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

export default function ${nodeType}(builder: Builder, node: ${nodeType}): Token {
	throw new Error("unimplemented");
}
`;
write(builderDefFile, builderContent);

// Write analysis
if (language === "js") {
	const analysisDefFile = path.join(analysisFolder, category, `${nodeType}.ts`);
	const analysisContent = `import {AnyNode, ${nodeType}, ${builderName}} from "@romefrontend/ast";

	export default function ${nodeType}(node: AnyNode) {
		node = ${builderName}.assert(node);
		throw new Error("unimplemented");
	}
	`;
	write(analysisDefFile, analysisContent);
}

require("./update.cjs");
