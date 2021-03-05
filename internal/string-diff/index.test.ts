/**
 * This tests are taken from the original source code and adapted
 * for our modified version.
 *
 * ===
 *
 * Diff Match and Patch -- Test Harness
 * Copyright 2018 The diff-match-patch Authors.
 * https://github.com/google/diff-match-patch
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import {test} from "rome";
import {
	DiffTypes,
	Diff,
	bisect,
	cleanupMerge,
	getCommonPrefix,
	getCommonSuffix,
	generateLineKey,
	halfMatch,
	main,
} from ".";

test(
	"generate line key",
	async (t) => {
		t.is("1:2", generateLineKey(1, 2));
		t.is(":2", generateLineKey(undefined, 2));
		t.is("1:", generateLineKey(1, undefined));
	},
);

test(
	"common prefix",
	async (t) => {
		t.is(0, getCommonPrefix("abc", "xyz"));
		t.is(4, getCommonPrefix("1234abcdef", "1234xyz"));
		t.is(4, getCommonPrefix("1234", "1234xyz"));
	},
);

test(
	"main",
	async (t) => {
		// Perform a trivial diff.
		// Null case.
		t.looksLike([], main("", "", false));

		// Equality.
		t.looksLike([[DiffTypes.EQUAL, "abc"]], main("abc", "abc", false));

		// Simple insertion.
		t.looksLike(
			[[DiffTypes.EQUAL, "ab"], [DiffTypes.INSERT, "123"], [DiffTypes.EQUAL, "c"]],
			main("abc", "ab123c", false),
		);

		// Simple deletion.
		t.looksLike(
			[[DiffTypes.EQUAL, "a"], [DiffTypes.DELETE, "123"], [DiffTypes.EQUAL, "bc"]],
			main("a123bc", "abc", false),
		);

		// Two insertions.
		t.looksLike(
			[
				[DiffTypes.EQUAL, "a"],
				[DiffTypes.INSERT, "123"],
				[DiffTypes.EQUAL, "b"],
				[DiffTypes.INSERT, "456"],
				[DiffTypes.EQUAL, "c"],
			],
			main("abc", "a123b456c", false),
		);

		// Two deletions.
		t.looksLike(
			[
				[DiffTypes.EQUAL, "a"],
				[DiffTypes.DELETE, "123"],
				[DiffTypes.EQUAL, "b"],
				[DiffTypes.DELETE, "456"],
				[DiffTypes.EQUAL, "c"],
			],
			main("a123b456c", "abc", false),
		);

		// Perform a real diff.
		// Simple cases.
		t.looksLike([[DiffTypes.DELETE, "a"], [DiffTypes.INSERT, "b"]], main("a", "b", false));

		t.looksLike(
			[
				[DiffTypes.DELETE, "Apple"],
				[DiffTypes.INSERT, "Banana"],
				[DiffTypes.EQUAL, "s are a"],
				[DiffTypes.INSERT, "lso"],
				[DiffTypes.EQUAL, " fruit."],
			],
			main("Apples are a fruit.", "Bananas are also fruit.", false),
		);

		t.looksLike(
			[
				[DiffTypes.DELETE, "a"],
				[DiffTypes.INSERT, "\u0680"],
				[DiffTypes.EQUAL, "x"],
				[DiffTypes.DELETE, "\t"],
				[DiffTypes.INSERT, "\0"],
			],
			main("ax\t", "\u0680x\0", false),
		);

		// Overlaps.
		t.looksLike(
			[
				[DiffTypes.DELETE, "1"],
				[DiffTypes.EQUAL, "a"],
				[DiffTypes.DELETE, "y"],
				[DiffTypes.EQUAL, "b"],
				[DiffTypes.DELETE, "2"],
				[DiffTypes.INSERT, "xab"],
			],
			main("1ayb2", "abxab", false),
		);

		t.looksLike(
			[[DiffTypes.INSERT, "xaxcx"], [DiffTypes.EQUAL, "abc"], [DiffTypes.DELETE, "y"]],
			main("abcy", "xaxcxabc", false),
		);

		t.looksLike(
			[
				[DiffTypes.DELETE, "ABCD"],
				[DiffTypes.EQUAL, "a"],
				[DiffTypes.DELETE, "="],
				[DiffTypes.INSERT, "-"],
				[DiffTypes.EQUAL, "bcd"],
				[DiffTypes.DELETE, "="],
				[DiffTypes.INSERT, "-"],
				[DiffTypes.EQUAL, "efghijklmnopqrs"],
				[DiffTypes.DELETE, "EFGHIJKLMNOefg"],
			],
			main(
				"ABCDa=bcd=efghijklmnopqrsEFGHIJKLMNOefg",
				"a-bcd-efghijklmnopqrs",
				false,
			),
		);

		// Large equality.
		t.looksLike(
			[
				[DiffTypes.INSERT, " "],
				[DiffTypes.EQUAL, "a"],
				[DiffTypes.INSERT, "nd"],
				[DiffTypes.EQUAL, " [[Pennsylvania]]"],
				[DiffTypes.DELETE, " and [[New"],
			],
			main("a [[Pennsylvania]] and [[New", " and [[Pennsylvania]]", false),
		);

		// Test the linemode speedup.
		// Must be long to pass the 100 char cutoff.
		// Simple line-mode.
		let a = "1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n";
		let b = "abcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\nabcdefghij\n";
		t.looksLike(main(a, b, false), main(a, b, true));

		// Single line-mode.
		a = "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
		b = "abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij";
		t.looksLike(main(a, b, false), main(a, b, true));
	},
);

test(
	"bisect",
	async (t) => {
		const a = "cat";
		const b = "map";
		// Since the resulting diff hasn't been normalized, it would be ok if
		// the insertion and deletion pairs are swapped.
		// If the order changes, tweak this test as required.
		t.looksLike(
			[
				[DiffTypes.DELETE, "c"],
				[DiffTypes.INSERT, "m"],
				[DiffTypes.EQUAL, "a"],
				[DiffTypes.DELETE, "t"],
				[DiffTypes.INSERT, "p"],
			],
			bisect(a, b),
		);
	},
);

test(
	"common sufix",
	async (t) => {
		// Null case.
		t.is(0, getCommonSuffix("abc", "xyz"));

		// Non-null case.
		t.is(4, getCommonSuffix("abcdef1234", "xyz1234"));

		// Whole case.
		t.is(4, getCommonSuffix("1234", "xyz1234"));
	},
);

test(
	"half match",
	async (t) => {
		// No match.
		t.is(undefined, halfMatch("1234567890", "abcdef"));

		t.is(undefined, halfMatch("12345", "23"));

		// Single Match.
		t.looksLike(
			["12", "90", "a", "z", "345678"],
			halfMatch("1234567890", "a345678z"),
		);

		t.looksLike(
			["a", "z", "12", "90", "345678"],
			halfMatch("a345678z", "1234567890"),
		);

		t.looksLike(
			["abc", "z", "1234", "0", "56789"],
			halfMatch("abc56789z", "1234567890"),
		);

		t.looksLike(
			["a", "xyz", "1", "7890", "23456"],
			halfMatch("a23456xyz", "1234567890"),
		);

		// Multiple Matches.
		t.looksLike(
			["12123", "123121", "a", "z", "1234123451234"],
			halfMatch("121231234123451234123121", "a1234123451234z"),
		);

		t.looksLike(
			["", "-=-=-=-=-=", "x", "", "x-=-=-=-=-=-=-="],
			halfMatch("x-=-=-=-=-=-=-=-=-=-=-=-=", "xx-=-=-=-=-=-=-="),
		);

		t.looksLike(
			["-=-=-=-=-=", "", "", "y", "-=-=-=-=-=-=-=y"],
			halfMatch("-=-=-=-=-=-=-=-=-=-=-=-=y", "-=-=-=-=-=-=-=yy"),
		);

		// Non-optimal halfmatch.
		// Optimal diff would be -q+x=H-i+e=lloHe+Hu=llo-Hew+y not -qHillo+x=HelloHe-w+Hulloy
		t.looksLike(
			["qHillo", "w", "x", "Hulloy", "HelloHe"],
			halfMatch("qHilloHelloHew", "xHelloHeHulloy"),
		);
	},
);

test(
	"cleanup merge",
	async (t) => {
		// Null case.
		let diffs: Diff[] = [];
		cleanupMerge(diffs, false);
		t.looksLike([], diffs);

		// No change case.
		diffs = [[DiffTypes.EQUAL, "a"], [DiffTypes.DELETE, "b"], [DiffTypes.INSERT, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike(
			[[DiffTypes.EQUAL, "a"], [DiffTypes.DELETE, "b"], [DiffTypes.INSERT, "c"]],
			diffs,
		);

		// Merge equalities.
		diffs = [[DiffTypes.EQUAL, "a"], [DiffTypes.EQUAL, "b"], [DiffTypes.EQUAL, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DiffTypes.EQUAL, "abc"]], diffs);

		// Merge deletions.
		diffs = [[DiffTypes.DELETE, "a"], [DiffTypes.DELETE, "b"], [DiffTypes.DELETE, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DiffTypes.DELETE, "abc"]], diffs);

		// Merge insertions.
		diffs = [[DiffTypes.INSERT, "a"], [DiffTypes.INSERT, "b"], [DiffTypes.INSERT, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DiffTypes.INSERT, "abc"]], diffs);

		// Merge interweave.
		diffs = [
			[DiffTypes.DELETE, "a"],
			[DiffTypes.INSERT, "b"],
			[DiffTypes.DELETE, "c"],
			[DiffTypes.INSERT, "d"],
			[DiffTypes.EQUAL, "e"],
			[DiffTypes.EQUAL, "f"],
		];
		cleanupMerge(diffs, false);
		t.looksLike(
			[[DiffTypes.DELETE, "ac"], [DiffTypes.INSERT, "bd"], [DiffTypes.EQUAL, "ef"]],
			diffs,
		);

		// Prefix and suffix detection.
		diffs = [[DiffTypes.DELETE, "a"], [DiffTypes.INSERT, "abc"], [DiffTypes.DELETE, "dc"]];
		cleanupMerge(diffs, false);
		t.looksLike(
			[
				[DiffTypes.EQUAL, "a"],
				[DiffTypes.DELETE, "d"],
				[DiffTypes.INSERT, "b"],
				[DiffTypes.EQUAL, "c"],
			],
			diffs,
		);

		// Prefix and suffix detection with equalities.
		diffs = [
			[DiffTypes.EQUAL, "x"],
			[DiffTypes.DELETE, "a"],
			[DiffTypes.INSERT, "abc"],
			[DiffTypes.DELETE, "dc"],
			[DiffTypes.EQUAL, "y"],
		];
		cleanupMerge(diffs, false);
		t.looksLike(
			[
				[DiffTypes.EQUAL, "xa"],
				[DiffTypes.DELETE, "d"],
				[DiffTypes.INSERT, "b"],
				[DiffTypes.EQUAL, "cy"],
			],
			diffs,
		);

		// Slide edit left.
		diffs = [[DiffTypes.EQUAL, "a"], [DiffTypes.INSERT, "ba"], [DiffTypes.EQUAL, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DiffTypes.INSERT, "ab"], [DiffTypes.EQUAL, "ac"]], diffs);

		// Slide edit right.
		diffs = [[DiffTypes.EQUAL, "c"], [DiffTypes.INSERT, "ab"], [DiffTypes.EQUAL, "a"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DiffTypes.EQUAL, "ca"], [DiffTypes.INSERT, "ba"]], diffs);

		// Slide edit left recursive.
		diffs = [
			[DiffTypes.EQUAL, "a"],
			[DiffTypes.DELETE, "b"],
			[DiffTypes.EQUAL, "c"],
			[DiffTypes.DELETE, "ac"],
			[DiffTypes.EQUAL, "x"],
		];
		cleanupMerge(diffs, false);
		t.looksLike([[DiffTypes.DELETE, "abc"], [DiffTypes.EQUAL, "acx"]], diffs);

		// Slide edit right recursive.
		diffs = [
			[DiffTypes.EQUAL, "x"],
			[DiffTypes.DELETE, "ca"],
			[DiffTypes.EQUAL, "c"],
			[DiffTypes.DELETE, "b"],
			[DiffTypes.EQUAL, "a"],
		];
		cleanupMerge(diffs, false);
		t.looksLike([[DiffTypes.EQUAL, "xca"], [DiffTypes.DELETE, "cba"]], diffs);

		// Empty merge.
		diffs = [[DiffTypes.DELETE, "b"], [DiffTypes.INSERT, "ab"], [DiffTypes.EQUAL, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DiffTypes.INSERT, "a"], [DiffTypes.EQUAL, "bc"]], diffs);

		// Empty equality.
		diffs = [[DiffTypes.EQUAL, ""], [DiffTypes.INSERT, "a"], [DiffTypes.EQUAL, "b"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DiffTypes.INSERT, "a"], [DiffTypes.EQUAL, "b"]], diffs);
	},
);
