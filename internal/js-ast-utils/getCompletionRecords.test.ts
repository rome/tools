/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {getCompletionRecords} from "./getCompletionRecords";
import {test} from "rome";
import {parseJS} from "@internal/js-parser";
import {jsFunctionDeclaration} from "@internal/ast";

function helper(input: string) {
	return getCompletionRecords(
		jsFunctionDeclaration.assert(
			parseJS({
				path: "unknown",
				input: `function foo(){${input}}`,
			}).body[0],
		).body,
	);
}

test(
	"invalid",
	async (t) => {
		t.snapshot(helper("{}"));
		t.snapshot(helper(`'foobar';`));
		t.snapshot(helper(`if (bar) {'foobar';}`));
		t.snapshot(helper(`if (bar) {'foobar';} else {}`));
		t.snapshot(helper("switch (foo) {}"));
		t.snapshot(helper(`switch (foo) {case 'bar': {}}`));
		t.snapshot(helper("switch (foo) {default: {}}"));
	},
);

test(
	"completions",
	async (t) => {
		t.snapshot(helper("return false;"));
		t.snapshot(helper("return; invalid;"));
		t.snapshot(helper("if (bar) {return false;}"));
		t.snapshot(helper("if (bar) {return false;} else {return true;}"));
		t.snapshot(helper("switch (foo) {default: {return false;}}"));
		t.snapshot(helper("switch (foo) {default: {return false;}}"));
	},
);

test(
	"mix",
	async (t) => {
		t.snapshot(helper("switch (foo) {default: {if (true) {return false;}}}"));
	},
);
