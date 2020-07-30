/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {toCamelCase} from "./toCamelCase";
import {test} from "rome";

test(
	"toCamelCase",
	(t) => {
		t.inlineSnapshot(toCamelCase("rometest"), "rometest");
		t.inlineSnapshot(toCamelCase("rome test"), "romeTest");
		t.inlineSnapshot(toCamelCase("RoMe TeSt"), "RoMeTeSt");
		t.inlineSnapshot(toCamelCase("ROME TEST"), "ROMETEST");
	},
);

test(
	"toCamelCase forcePascal",
	(t) => {
		const opts = {forcePascal: true};
		t.inlineSnapshot(toCamelCase("rometest", opts), "Rometest");
		t.inlineSnapshot(toCamelCase("rome test", opts), "RomeTest");
		t.inlineSnapshot(toCamelCase("RoMe TeSt", opts), "RoMeTeSt");
		t.inlineSnapshot(toCamelCase("Rome Test", opts), "RomeTest");
		t.inlineSnapshot(toCamelCase("ROME TEST", opts), "ROMETEST");
	},
);

test(
	"toCamelCase allowShouty",
	(t) => {
		const opts = {allowShouty: true};
		t.inlineSnapshot(toCamelCase("ROME_yes_TEST", opts), "ROME_yes_TEST");
		t.inlineSnapshot(toCamelCase("__yes__", opts), "__yes__");
		t.inlineSnapshot(toCamelCase("__yes_bar__", opts), "__yesBar__");
	},
);

test(
	"toCamelCase allowPascal",
	(t) => {
		const opts = {allowPascal: true};
		t.inlineSnapshot(toCamelCase("RomeTest", opts), "RomeTest");
	},
);
