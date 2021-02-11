/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/cli-layout";
import {
	PathPattern,
	parsePathPattern,
	parsePathPatternsFile,
} from "@internal/path-match";
import {test} from "rome";
import {dedent} from "@internal/string-utils";

function _parsePathPattern(input: string): PathPattern {
	return parsePathPattern({input});
}

test(
	"pattern",
	async (t) => {
		// Negate and wildcard
		t.inlineSnapshot(
			_parsePathPattern("!foo"),
			'PathPattern {\n\tnegate: true\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:1\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:1-1:1\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:1-1:1\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern(""),
			"PathPattern {\n\tnegate: false\n\troot: false\n\tsegments: Array []\n\tloc: SourceLocation unknown 1:0-1:0\n}",
		);

		// Trailing slash and wildcards
		t.inlineSnapshot(
			_parsePathPattern("/foo/bar"),
			'PathPattern {\n\tnegate: false\n\troot: true\n\tloc: SourceLocation unknown 1:0-1:5\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:1-1:5\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:1-1:4\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:5-1:5\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:5-1:5\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("*/foo/bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:6\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:2-1:6\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:2-1:5\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:6-1:6\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:6-1:6\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("**/foo/bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:7\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:3-1:7\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:3-1:6\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:7-1:7\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:7-1:7\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("**/*foo/bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:8\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:3-1:8\n\t\t\tparts: Array [\n\t\t\t\tWildcard {loc: SourceLocation unknown 1:3-1:4}\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:4-1:7\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:8-1:8\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:8-1:8\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);

		// Random
		t.inlineSnapshot(
			_parsePathPattern("foo"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:0\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:0\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:0\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:3\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:4\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:4-1:4\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo//bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:5\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:5\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:5-1:5\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:5-1:5\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/*/bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:6\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:6\n\t\t\tparts: Array [Wildcard {loc: SourceLocation unknown 1:4-1:5}]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:6-1:6\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:6-1:6\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/**/bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:7\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tWildcardSegment {loc: SourceLocation unknown 1:4-1:7}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:7-1:7\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:7-1:7\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/*bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:5\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:5\n\t\t\tparts: Array [\n\t\t\t\tWildcard {loc: SourceLocation unknown 1:4-1:5}\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:5-1:5\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/bar*"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:7\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:7\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:4-1:7\n\t\t\t\t}\n\t\t\t\tWildcard {loc: SourceLocation unknown 1:7-1:7}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/*bar*"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:8\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:8\n\t\t\tparts: Array [\n\t\t\t\tWildcard {loc: SourceLocation unknown 1:4-1:5}\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:5-1:8\n\t\t\t\t}\n\t\t\t\tWildcard {loc: SourceLocation unknown 1:8-1:8}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/*bar*foob"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:9\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:9\n\t\t\tparts: Array [\n\t\t\t\tWildcard {loc: SourceLocation unknown 1:4-1:5}\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:5-1:8\n\t\t\t\t}\n\t\t\t\tWildcard {loc: SourceLocation unknown 1:8-1:9}\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foob"\n\t\t\t\t\tloc: SourceLocation unknown 1:9-1:9\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);

		// Comments
		t.inlineSnapshot(
			_parsePathPattern("# foobar"),
			'Comment {\n\tvalue: "# foobar"\n\tloc: SourceLocation unknown 1:0-1:0\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/bar # foobar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:4\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar # foobar"\n\t\t\t\t\tloc: SourceLocation unknown 1:4-1:4\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/bar\\#foobar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:4\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar\\\\#foobar"\n\t\t\t\t\tloc: SourceLocation unknown 1:4-1:4\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("foo/\\#foobar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:4\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:3\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:4\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "\\\\#foobar"\n\t\t\t\t\tloc: SourceLocation unknown 1:4-1:4\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);

		// Windows separators
		t.inlineSnapshot(
			_parsePathPattern("\\\\foo\\\\bar"),
			'PathPattern {\n\tnegate: false\n\troot: true\n\tloc: SourceLocation unknown 1:0-1:7\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:2-1:7\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:2-1:5\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:7-1:7\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:7-1:7\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("*\\\\foo\\\\bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:8\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:3-1:8\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:3-1:6\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:8-1:8\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:8-1:8\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("**\\\\foo\\\\bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:9\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:9\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:4-1:7\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:9-1:9\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:9-1:9\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("**\\\\*foo\\\\bar"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:10\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:4-1:10\n\t\t\tparts: Array [\n\t\t\t\tWildcard {loc: SourceLocation unknown 1:4-1:5}\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "foo"\n\t\t\t\t\tloc: SourceLocation unknown 1:5-1:8\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:10-1:10\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "bar"\n\t\t\t\t\tloc: SourceLocation unknown 1:10-1:10\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
		t.inlineSnapshot(
			_parsePathPattern("hello\\\\world"),
			'PathPattern {\n\tnegate: false\n\troot: false\n\tloc: SourceLocation unknown 1:0-1:7\n\tsegments: Array [\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:0-1:7\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "hello"\n\t\t\t\t\tloc: SourceLocation unknown 1:0-1:5\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t\tSegment {\n\t\t\tloc: SourceLocation unknown 1:7-1:7\n\t\t\tparts: Array [\n\t\t\t\tWord {\n\t\t\t\t\tvalue: "world"\n\t\t\t\t\tloc: SourceLocation unknown 1:7-1:7\n\t\t\t\t}\n\t\t\t]\n\t\t}\n\t]\n}',
		);
	},
);

test(
	"patterns file",
	(t) => {
		t.inlineSnapshot(
			parsePathPatternsFile({
				input: dedent`
					# comment
					some/path
					!/some/path
					path#not a comment

					# foo
					yes/i/guess this works?
				`,
			}),
			'Array [\n\tComment {\n\t\tvalue: "# comment"\n\t\tloc: SourceLocation unknown 1:0-1:9\n\t}\n\tPathPattern {\n\t\tnegate: false\n\t\troot: false\n\t\tloc: SourceLocation unknown 2:0-2:9\n\t\tsegments: Array [\n\t\t\tSegment {\n\t\t\t\tloc: SourceLocation unknown 2:0-2:5\n\t\t\t\tparts: Array [\n\t\t\t\t\tWord {\n\t\t\t\t\t\tvalue: "some"\n\t\t\t\t\t\tloc: SourceLocation unknown 2:0-2:4\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t\tSegment {\n\t\t\t\tloc: SourceLocation unknown 2:5-2:9\n\t\t\t\tparts: Array [\n\t\t\t\t\tWord {\n\t\t\t\t\t\tvalue: "path"\n\t\t\t\t\t\tloc: SourceLocation unknown 2:5-2:9\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t]\n\t}\n\tPathPattern {\n\t\tnegate: true\n\t\troot: true\n\t\tloc: SourceLocation unknown 3:0-3:11\n\t\tsegments: Array [\n\t\t\tSegment {\n\t\t\t\tloc: SourceLocation unknown 3:2-3:7\n\t\t\t\tparts: Array [\n\t\t\t\t\tWord {\n\t\t\t\t\t\tvalue: "some"\n\t\t\t\t\t\tloc: SourceLocation unknown 3:2-3:6\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t\tSegment {\n\t\t\t\tloc: SourceLocation unknown 3:7-3:11\n\t\t\t\tparts: Array [\n\t\t\t\t\tWord {\n\t\t\t\t\t\tvalue: "path"\n\t\t\t\t\t\tloc: SourceLocation unknown 3:7-3:11\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t]\n\t}\n\tPathPattern {\n\t\tnegate: false\n\t\troot: false\n\t\tloc: SourceLocation unknown 4:0-4:18\n\t\tsegments: Array [\n\t\t\tSegment {\n\t\t\t\tloc: SourceLocation unknown 4:0-4:18\n\t\t\t\tparts: Array [\n\t\t\t\t\tWord {\n\t\t\t\t\t\tvalue: "path#not a comment"\n\t\t\t\t\t\tloc: SourceLocation unknown 4:0-4:18\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t]\n\t}\n\tComment {\n\t\tvalue: "# foo"\n\t\tloc: SourceLocation unknown 6:0-6:5\n\t}\n\tPathPattern {\n\t\tnegate: false\n\t\troot: false\n\t\tloc: SourceLocation unknown 7:0-7:6\n\t\tsegments: Array [\n\t\t\tSegment {\n\t\t\t\tloc: SourceLocation unknown 7:0-7:4\n\t\t\t\tparts: Array [\n\t\t\t\t\tWord {\n\t\t\t\t\t\tvalue: "yes"\n\t\t\t\t\t\tloc: SourceLocation unknown 7:0-7:3\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t\tSegment {\n\t\t\t\tloc: SourceLocation unknown 7:4-7:6\n\t\t\t\tparts: Array [\n\t\t\t\t\tWord {\n\t\t\t\t\t\tvalue: "i"\n\t\t\t\t\t\tloc: SourceLocation unknown 7:4-7:5\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t\tSegment {\n\t\t\t\tloc: SourceLocation unknown 7:6-7:6\n\t\t\t\tparts: Array [\n\t\t\t\t\tWord {\n\t\t\t\t\t\tvalue: "guess this works?"\n\t\t\t\t\t\tloc: SourceLocation unknown 7:6-7:6\n\t\t\t\t\t}\n\t\t\t\t]\n\t\t\t}\n\t\t]\n\t}\n]',
		);
	},
);
