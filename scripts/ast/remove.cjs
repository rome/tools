/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require("../_setup.cjs");

const path = require("path");

const filename = process.argv[2];
if (filename === undefined) {
	console.error("node remove-ast-type.js [language]/[category]/[nodeType]");
	process.exit(1);
}

const {
	formatterFolder,
	analysisFolder,
	astFolder,
} = require("../_constants.cjs");

const {unlink} = require("../_utils.cjs");

unlink(path.join(formatterFolder, `${filename}.ts`));
unlink(path.join(analysisFolder, `${filename}.ts`));
unlink(path.join(astFolder, `${filename}.ts`));

require("../generated-files/ast.cjs");
