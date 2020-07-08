/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {parseJS} from "@romefrontend/js-parser";
import {createFixtureTests} from "@romefrontend/test-helpers";
import {ConstJSProgramSyntax} from "@romefrontend/ast";
import {removeCarriageReturn} from "@romefrontend/string-utils";
import {printDiagnosticsToString} from "@romefrontend/cli-diagnostics";

const promise = createFixtureTests(async (fixture, t) => {
	const {options, files} = fixture;

	// Get the input JS
	const inputFile =
		files.get("input.js") ||
		files.get("input.jsx") ||
		files.get("input.mjs") ||
		files.get("input.ts") ||
		files.get("input.tsx");
	if (inputFile === undefined) {
		throw new Error(
			`The fixture ${fixture.dir} did not have an input.(mjs|js|jsx|ts|tsx)`,
		);
	}

	const ext = inputFile.absolute.getExtensions();

	// Default to script for .js files, otherwise module
	const sourceType = options.get("sourceType").asStringSet(
		["script", "module"],
		ext === ".js" ? "script" : "module",
	);

	const allowReturnOutsideFunction = options.get("allowReturnOutsideFunction").asBoolean(
		false,
	);
	const filename = inputFile.relative;

	const syntax: Array<ConstJSProgramSyntax> = options.get("syntax").asArray(
		true,
	).map((item) => {
		return item.asStringSet(["jsx", "ts"]);
	});

	// Implicit syntax property
	if (!options.has("syntax")) {
		switch (ext) {
			case ".jsx": {
				syntax.push("jsx");
				break;
			}

			case ".ts": {
				syntax.push("ts");
				break;
			}

			case ".tsx": {
				syntax.push("ts");
				syntax.push("jsx");
				break;
			}
		}
	}

	t.addToAdvice({
		type: "log",
		category: "info",
		text: "Parser options",
	});

	t.addToAdvice({
		type: "inspect",
		data: {
			filename: filename.join(),
			allowReturnOutsideFunction,
			sourceType,
			syntax,
		},
	});

	const inputContent = removeCarriageReturn(inputFile.content.toString());

	const ast = parseJS({
		input: inputContent,
		path: filename,
		allowReturnOutsideFunction,
		sourceType,
		syntax,
	});

	const outputFile = inputFile.absolute.getParent().append(
		inputFile.absolute.getExtensionlessBasename(),
	).join();
	t.namedSnapshot("ast", ast, undefined, {filename: outputFile});

	const printedDiagnostics = printDiagnosticsToString({
		diagnostics: ast.diagnostics,
		suppressions: [],
	});
	t.namedSnapshot(
		"diagnostics",
		printedDiagnostics,
		undefined,
		{filename: outputFile},
	);

	if (ast.diagnostics.length === 0) {
		if (options.has("throws")) {
			// TODO: throw new Error(`Expected diagnostics but didn't receive any\n${printedDiagnostics}`);
		}
	} else if (!options.has("throws")) {
		// TODO: throw new Error(`Received diagnostics when we didn't expect any\n${printedDiagnostics}`);
	}
});

// @ts-ignore Doesn't support top-level await lol
await promise;
