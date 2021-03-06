/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	createAbsoluteFilePath,
	createFilePath,
	createRelativePath,
} from "@internal/path";
import {test} from "rome";

const relativeTests: [string, string, string][] = [
	// Windows paths
	["c:/blah\\blah", "d:/games", "D:\\games"],
	["c:/aaaa/bbbb", "c:/aaaa", ".."],
	["c:/aaaa/bbbb", "c:/cccc", "../../cccc"],
	["c:/aaaa/bbbb", "c:/aaaa/bbbb", "."],
	["c:/aaaa/bbbb", "c:/aaaa/cccc", "../cccc"],
	["c:/aaaa/", "c:/aaaa/cccc", "cccc"],
	["c:/", "c:\\aaaa\\bbbb", "aaaa/bbbb"],
	["c:/aaaa/bbbb", "d:\\", "D:\\"],
	["c:/aaaaa/", "c:/aaaa/cccc", "../aaaa/cccc"],
	["C:\\foo\\bar\\baz\\quux", "C:\\", "../../../.."],
	["C:\\foo\\test", "C:\\foo\\test\\bar\\package.json", "bar/package.json"],
	["C:\\foo\\bar\\baz-quux", "C:\\foo\\bar\\baz", "../baz"],
	["C:\\foo\\bar\\baz", "C:\\foo\\bar\\baz-quux", "../baz-quux"],
	["\\\\foo\\bar", "\\\\foo\\bar\\baz", "baz"],
	["\\\\foo\\bar\\baz", "\\\\foo\\bar", ".."],
	["\\\\foo\\bar\\baz-quux", "\\\\foo\\bar\\baz", "../baz"],
	["\\\\foo\\bar\\baz", "\\\\foo\\bar\\baz-quux", "../baz-quux"],
	["C:\\baz-quux", "C:\\baz", "../baz"],
	["C:\\baz", "C:\\baz-quux", "../baz-quux"],
	["\\\\foo\\baz-quux", "\\\\foo\\baz", "../baz"],
	["\\\\foo\\baz", "\\\\foo\\baz-quux", "../baz-quux"],
	["C:\\baz", "\\\\foo\\bar\\baz", "\\\\foo\\bar\\baz"],
	["\\\\foo\\bar\\baz", "C:\\baz", "C:\\baz"],
	// Posix paths
	["/var/lib", "/var", ".."],
	["/var/lib", "/bin", "../../bin"],
	["/var/lib", "/var/lib", "."],
	["/var/lib", "/var/apache", "../apache"],
	["/var/", "/var/lib", "lib"],
	["/", "/var/lib", "var/lib"],
	["/foo/test", "/foo/test/bar/package.json", "bar/package.json"],
	["/Users/a/web/b/test/mails", "/Users/a/web/b", "../.."],
	["/foo/bar/baz-quux", "/foo/bar/baz", "../baz"],
	["/foo/bar/baz", "/foo/bar/baz-quux", "../baz-quux"],
	["/baz-quux", "/baz", "../baz"],
	["/baz", "/baz-quux", "../baz-quux"],
	["/page1/page2/foo", "/", "../../.."],
];

for (let i = 0; i < relativeTests.length; i++) {
	const [absolute, arg, expected] = relativeTests[i];

	test(
		`relative ${i}: ${absolute}`,
		(t) => {
			t.addToAdvice({
				type: "log",
				category: "info",
				text: "Metadata",
			});

			const relative = createAbsoluteFilePath(absolute).relative(
				createFilePath(arg),
			);

			t.addToAdvice({
				type: "inspect",
				data: {
					in: {
						absolute,
						arg,
						expected,
					},
					out: {
						filename: relative.join(),
						parsed: relative.parsed,
					},
				},
			});

			t.is(relative.join(), expected);
		},
	);
}

const segmentTests: [string, string[]][] = [
	["./../images/test.png", ["..", "images", "test.png"]],
	["foo/", ["foo"]],
];

for (let i = 0; i < segmentTests.length; i++) {
	const [loc, expectedSegments] = segmentTests[i];

	test(
		`segments: ${i}: ${loc}`,
		(t) => {
			t.looksLike(createRelativePath(loc).getSegments(), expectedSegments);
		},
	);
}

test(
	"tilde doesn't expand with relative hint",
	(t) => {
		t.true(createAbsoluteFilePath("~/foo").getSegments()[0] !== "~");
		t.inlineSnapshot(createRelativePath("~/foo").getSegments(), '["~", "foo"]');
		t.inlineSnapshot(
			createAbsoluteFilePath("/bar").append("~/foo").getSegments(),
			'["bar", "~", "foo"]',
		);
	},
);

test(
	"getUnique",
	(t) => {
		t.inlineSnapshot(createRelativePath(".").getUnique().join(), ".");
		t.inlineSnapshot(createRelativePath("./foo").getUnique().join(), "foo");
		t.inlineSnapshot(createRelativePath("foo/").getUnique().join(), "foo");
		t.inlineSnapshot(
			createRelativePath("foo/bar").getUnique().join(),
			"foo/bar",
		);
	},
);
