/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/cli-layout";
import {
	PathPattern,
	matchPath,
	matchPathPatterns,
	parsePathPattern,
} from "@internal/path-match";
import {test} from "rome";
import {createAbsoluteFilePath} from "@internal/path";

const DOCUMENTS = "/Users/sebmck/Documents";

function _parsePathPattern(input: string): PathPattern {
	return parsePathPattern({input});
}

test(
	"match",
	(t) => {
		// Basic
		t.true(
			matchPath(createAbsoluteFilePath(DOCUMENTS), _parsePathPattern("sebmck")),
		);
		t.true(
			matchPath(
				createAbsoluteFilePath(DOCUMENTS),
				_parsePathPattern("sebmck/Documents"),
			),
		);
		t.true(
			matchPath(createAbsoluteFilePath(DOCUMENTS), _parsePathPattern("Users")),
		);
		t.false(matchPath(createAbsoluteFilePath(DOCUMENTS), _parsePathPattern("")));
		t.false(
			matchPath(
				createAbsoluteFilePath(DOCUMENTS),
				_parsePathPattern("# comment"),
			),
		);
		t.true(
			matchPath(createAbsoluteFilePath(DOCUMENTS), _parsePathPattern("sebmck")),
		);

		// Single stars
		t.true(
			matchPath(
				createAbsoluteFilePath(DOCUMENTS),
				_parsePathPattern("/Users/*/Documents"),
			),
		);
		t.true(
			matchPath(
				createAbsoluteFilePath(DOCUMENTS),
				_parsePathPattern("/Users/*mck/Documents"),
			),
		);
		t.true(
			matchPath(
				createAbsoluteFilePath(DOCUMENTS),
				_parsePathPattern("/Users/se*ck/Documents"),
			),
		);
		t.true(
			matchPath(
				createAbsoluteFilePath(DOCUMENTS),
				_parsePathPattern("/Users/seb*/Documents"),
			),
		);
		t.true(
			matchPath(
				createAbsoluteFilePath("/Projects/rome/index.js"),
				_parsePathPattern("*.js"),
			),
		);

		// Double stars
		t.true(
			matchPath(
				createAbsoluteFilePath(DOCUMENTS),
				_parsePathPattern("**/Documents"),
			),
		);
		t.true(
			matchPath(
				createAbsoluteFilePath("/Users/sebmck/Documents/Projects"),
				_parsePathPattern("/Users/**/Projects"),
			),
		);

		// Negate
		t.true(
			matchPath(
				createAbsoluteFilePath("/website/styles/site.css"),
				_parsePathPattern("*.css"),
			),
		);
		t.false(
			matchPath(
				createAbsoluteFilePath("/website/styles/site.css"),
				_parsePathPattern("!*.css"),
			),
		);
	},
);

test(
	"matchPathPatterns",
	(t) => {
		t.true(
			matchPathPatterns(
				createAbsoluteFilePath("/scripts/foo.js"),
				[_parsePathPattern("scripts"), _parsePathPattern("styles")],
			) === "EXPLICIT_MATCH",
		);

		t.false(
			matchPathPatterns(
				createAbsoluteFilePath("/scripts/foo.js"),
				[_parsePathPattern("scripts"), _parsePathPattern("!scripts/*.js")],
			) === "EXPLICIT_MATCH",
		);

		t.true(
			matchPathPatterns(
				createAbsoluteFilePath("/scripts/foo.js"),
				[_parsePathPattern("scripts")],
			) === "EXPLICIT_MATCH",
		);
	},
);
