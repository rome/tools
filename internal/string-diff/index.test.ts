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
	DIFF_DELETE,
	DIFF_EQUAL,
	DIFF_INSERT,
	Diff,
	bisect,
	cleanupMerge,
	commonPrefix,
	commonSuffix,
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
		t.is(0, commonPrefix("abc", "xyz"));
		t.is(4, commonPrefix("1234abcdef", "1234xyz"));
		t.is(4, commonPrefix("1234", "1234xyz"));
	},
);

test(
	"main",
	async (t) => {
		// Perform a trivial diff.
		// Null case.
		t.looksLike([], main("", "", false));

		// Equality.
		t.looksLike([[DIFF_EQUAL, "abc"]], main("abc", "abc", false));

		// Simple insertion.
		t.looksLike(
			[[DIFF_EQUAL, "ab"], [DIFF_INSERT, "123"], [DIFF_EQUAL, "c"]],
			main("abc", "ab123c", false),
		);

		// Simple deletion.
		t.looksLike(
			[[DIFF_EQUAL, "a"], [DIFF_DELETE, "123"], [DIFF_EQUAL, "bc"]],
			main("a123bc", "abc", false),
		);

		// Two insertions.
		t.looksLike(
			[
				[DIFF_EQUAL, "a"],
				[DIFF_INSERT, "123"],
				[DIFF_EQUAL, "b"],
				[DIFF_INSERT, "456"],
				[DIFF_EQUAL, "c"],
			],
			main("abc", "a123b456c", false),
		);

		// Two deletions.
		t.looksLike(
			[
				[DIFF_EQUAL, "a"],
				[DIFF_DELETE, "123"],
				[DIFF_EQUAL, "b"],
				[DIFF_DELETE, "456"],
				[DIFF_EQUAL, "c"],
			],
			main("a123b456c", "abc", false),
		);

		// Perform a real diff.
		// Simple cases.
		t.looksLike([[DIFF_DELETE, "a"], [DIFF_INSERT, "b"]], main("a", "b", false));

		t.looksLike(
			[
				[DIFF_DELETE, "Apple"],
				[DIFF_INSERT, "Banana"],
				[DIFF_EQUAL, "s are a"],
				[DIFF_INSERT, "lso"],
				[DIFF_EQUAL, " fruit."],
			],
			main("Apples are a fruit.", "Bananas are also fruit.", false),
		);

		t.looksLike(
			[
				[DIFF_DELETE, "a"],
				[DIFF_INSERT, "\u0680"],
				[DIFF_EQUAL, "x"],
				[DIFF_DELETE, "\t"],
				[DIFF_INSERT, "\0"],
			],
			main("ax\t", "\u0680x\0", false),
		);

		// Overlaps.
		t.looksLike(
			[
				[DIFF_DELETE, "1"],
				[DIFF_EQUAL, "a"],
				[DIFF_DELETE, "y"],
				[DIFF_EQUAL, "b"],
				[DIFF_DELETE, "2"],
				[DIFF_INSERT, "xab"],
			],
			main("1ayb2", "abxab", false),
		);

		t.looksLike(
			[[DIFF_INSERT, "xaxcx"], [DIFF_EQUAL, "abc"], [DIFF_DELETE, "y"]],
			main("abcy", "xaxcxabc", false),
		);

		t.looksLike(
			[
				[DIFF_DELETE, "ABCD"],
				[DIFF_EQUAL, "a"],
				[DIFF_DELETE, "="],
				[DIFF_INSERT, "-"],
				[DIFF_EQUAL, "bcd"],
				[DIFF_DELETE, "="],
				[DIFF_INSERT, "-"],
				[DIFF_EQUAL, "efghijklmnopqrs"],
				[DIFF_DELETE, "EFGHIJKLMNOefg"],
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
				[DIFF_INSERT, " "],
				[DIFF_EQUAL, "a"],
				[DIFF_INSERT, "nd"],
				[DIFF_EQUAL, " [[Pennsylvania]]"],
				[DIFF_DELETE, " and [[New"],
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
				[DIFF_DELETE, "c"],
				[DIFF_INSERT, "m"],
				[DIFF_EQUAL, "a"],
				[DIFF_DELETE, "t"],
				[DIFF_INSERT, "p"],
			],
			bisect(a, b),
		);
	},
);

test(
	"common sufix",
	async (t) => {
		// Null case.
		t.is(0, commonSuffix("abc", "xyz"));

		// Non-null case.
		t.is(4, commonSuffix("abcdef1234", "xyz1234"));

		// Whole case.
		t.is(4, commonSuffix("1234", "xyz1234"));
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
		diffs = [[DIFF_EQUAL, "a"], [DIFF_DELETE, "b"], [DIFF_INSERT, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike(
			[[DIFF_EQUAL, "a"], [DIFF_DELETE, "b"], [DIFF_INSERT, "c"]],
			diffs,
		);

		// Merge equalities.
		diffs = [[DIFF_EQUAL, "a"], [DIFF_EQUAL, "b"], [DIFF_EQUAL, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DIFF_EQUAL, "abc"]], diffs);

		// Merge deletions.
		diffs = [[DIFF_DELETE, "a"], [DIFF_DELETE, "b"], [DIFF_DELETE, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DIFF_DELETE, "abc"]], diffs);

		// Merge insertions.
		diffs = [[DIFF_INSERT, "a"], [DIFF_INSERT, "b"], [DIFF_INSERT, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DIFF_INSERT, "abc"]], diffs);

		// Merge interweave.
		diffs = [
			[DIFF_DELETE, "a"],
			[DIFF_INSERT, "b"],
			[DIFF_DELETE, "c"],
			[DIFF_INSERT, "d"],
			[DIFF_EQUAL, "e"],
			[DIFF_EQUAL, "f"],
		];
		cleanupMerge(diffs, false);
		t.looksLike(
			[[DIFF_DELETE, "ac"], [DIFF_INSERT, "bd"], [DIFF_EQUAL, "ef"]],
			diffs,
		);

		// Prefix and suffix detection.
		diffs = [[DIFF_DELETE, "a"], [DIFF_INSERT, "abc"], [DIFF_DELETE, "dc"]];
		cleanupMerge(diffs, false);
		t.looksLike(
			[
				[DIFF_EQUAL, "a"],
				[DIFF_DELETE, "d"],
				[DIFF_INSERT, "b"],
				[DIFF_EQUAL, "c"],
			],
			diffs,
		);

		// Prefix and suffix detection with equalities.
		diffs = [
			[DIFF_EQUAL, "x"],
			[DIFF_DELETE, "a"],
			[DIFF_INSERT, "abc"],
			[DIFF_DELETE, "dc"],
			[DIFF_EQUAL, "y"],
		];
		cleanupMerge(diffs, false);
		t.looksLike(
			[
				[DIFF_EQUAL, "xa"],
				[DIFF_DELETE, "d"],
				[DIFF_INSERT, "b"],
				[DIFF_EQUAL, "cy"],
			],
			diffs,
		);

		// Slide edit left.
		diffs = [[DIFF_EQUAL, "a"], [DIFF_INSERT, "ba"], [DIFF_EQUAL, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DIFF_INSERT, "ab"], [DIFF_EQUAL, "ac"]], diffs);

		// Slide edit right.
		diffs = [[DIFF_EQUAL, "c"], [DIFF_INSERT, "ab"], [DIFF_EQUAL, "a"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DIFF_EQUAL, "ca"], [DIFF_INSERT, "ba"]], diffs);

		// Slide edit left recursive.
		diffs = [
			[DIFF_EQUAL, "a"],
			[DIFF_DELETE, "b"],
			[DIFF_EQUAL, "c"],
			[DIFF_DELETE, "ac"],
			[DIFF_EQUAL, "x"],
		];
		cleanupMerge(diffs, false);
		t.looksLike([[DIFF_DELETE, "abc"], [DIFF_EQUAL, "acx"]], diffs);

		// Slide edit right recursive.
		diffs = [
			[DIFF_EQUAL, "x"],
			[DIFF_DELETE, "ca"],
			[DIFF_EQUAL, "c"],
			[DIFF_DELETE, "b"],
			[DIFF_EQUAL, "a"],
		];
		cleanupMerge(diffs, false);
		t.looksLike([[DIFF_EQUAL, "xca"], [DIFF_DELETE, "cba"]], diffs);

		// Empty merge.
		diffs = [[DIFF_DELETE, "b"], [DIFF_INSERT, "ab"], [DIFF_EQUAL, "c"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DIFF_INSERT, "a"], [DIFF_EQUAL, "bc"]], diffs);

		// Empty equality.
		diffs = [[DIFF_EQUAL, ""], [DIFF_INSERT, "a"], [DIFF_EQUAL, "b"]];
		cleanupMerge(diffs, false);
		t.looksLike([[DIFF_INSERT, "a"], [DIFF_EQUAL, "b"]], diffs);
	},
);
