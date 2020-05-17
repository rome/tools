/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {compareTwoStrings, orderBySimilarity} from "./orderBySimilarity";
import {test} from "rome";

test(
	"compareTwoStrings",
	(t) => {
		const testData = [
			{first: "french", second: "quebec", expected: 0},
			{first: "france", second: "france", expected: 1},
			{first: "fRaNce", second: "france", expected: 0.2},
			{first: "healed", second: "sealed", expected: 0.8},
			{
				first: "web applications",
				second: "applications of the web",
				expected: 0.7878787878787878,
			},
			{
				first: "this will have a typo somewhere",
				second: "this will huve a typo somewhere",
				expected: 0.92,
			},
			{
				first: "Olive-green table for sale, in extremely good condition.",
				second: "For sale: table in very good  condition, olive green in colour.",
				expected: 0.6060606060606061,
			},
			{
				first: "Olive-green table for sale, in extremely good condition.",
				second: "For sale: green Subaru Impreza, 210,000 miles",
				expected: 0.2558139534883721,
			},
			{
				first: "Olive-green table for sale, in extremely good condition.",
				second: "Wanted: mountain bike with at least 21 gears.",
				expected: 0.1411764705882353,
			},
			{
				first: "this has one extra word",
				second: "this has one word",
				expected: 0.7741935483870968,
			},
			{first: "a", second: "a", expected: 1},
			{first: "a", second: "b", expected: 0},
			{first: "", second: "", expected: 1},
			{first: "a", second: "", expected: 0},
			{first: "", second: "a", expected: 0},
			{first: "apple event", second: "apple    event", expected: 1},
			{first: "iphone", second: "iphone x", expected: 0.9090909090909091},
		];

		testData.forEach((td) => {
			t.is(compareTwoStrings(td.first, td.second), td.expected);
		});
	},
);

test(
	"orderBySimilarity",
	(t) => {
		t.looksLike(
			orderBySimilarity("french", ["quebec", "123", "france", "french"]),
			[
				{target: "french", rating: 1},
				{target: "france", rating: 0.4},
				{target: "quebec", rating: 0},
				{target: "123", rating: 0},
			],
		);

		t.looksLike(
			orderBySimilarity("iphone", ["ipod", "iphone 5s", "iphone x"]),
			[
				{target: "iphone x", rating: 0.9090909090909091},
				{target: "iphone 5s", rating: 0.8333333333333334},
				{target: "ipod", rating: 0.25},
			],
		);

		t.looksLike(
			orderBySimilarity(
				"french",
				["quebec", "123", "france", "french"],
				{
					minRating: 0.5,
				},
			),
			[{target: "french", rating: 1}],
		);

		t.looksLike(
			orderBySimilarity(
				"iphone",
				["ipod", "iphone 5s", "iphone x", "IPHONE"],
				{
					minRating: 0.5,
				},
			),
			[
				{target: "iphone x", rating: 0.9090909090909091},
				{target: "iphone 5s", rating: 0.8333333333333334},
			],
		);

		t.looksLike(
			orderBySimilarity(
				"iphone",
				["IPHONE", "iphone 5s", "iphone x"],
				{
					minRating: 0.5,
					ignoreCase: true,
				},
			),
			[
				{target: "IPHONE", rating: 1},
				{target: "iphone x", rating: 0.9090909090909091},
				{target: "iphone 5s", rating: 0.8333333333333334},
			],
		);
	},
);
