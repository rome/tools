import {test} from "rome";
import {findClosestMatchingStrings} from "./findClosestMatchingStrings";

test(
	"findClosestMatchingStrings",
	(t) => {
		t.looksLike(
			findClosestMatchingStrings("french", ["quebec", "123", "france", "frenc"]),
			["frenc"],
		);
		t.looksLike(
			findClosestMatchingStrings("iphone", ["ipod", "iphone 5s", "iphones x"]),
			["iphone 5s", "iphones x"],
		);

		t.looksLike(
			findClosestMatchingStrings(
				"iphone",
				["ipod", "iphone 5s", "iphones x"],
				0.9,
			),
			[],
		);

		t.looksLike(
			findClosestMatchingStrings(
				"french",
				["quebec", "123", "france", "frenc"],
				0.9,
			),
			[],
		);
		t.looksLike(
			findClosestMatchingStrings(
				"iphone",
				["ipod", "iphone 5s", "iphones x"],
				0.9,
			),
			[],
		);
	},
);
