import {test} from "rome";
import {
	decode,
	decodeVLQ,
	encode,
	encodeVLQ,
} from "@internal/codec-source-map/base64";

test(
	"verify encode returns the correct character",
	async (t) => {
		t.is(encode(0), "A");
		t.is(encode(35), "j");
		t.is(encode(52), "0");
		t.is(encode(17), "R");
		t.throws(() => {
			encode(65);
		});
	},
);

test(
	"verify decode returns the correct code",
	async (t) => {
		t.is(decode("D".charCodeAt(0)), 3);
		t.is(decode("6".charCodeAt(0)), 58);
		t.is(decode("h".charCodeAt(0)), 33);
		t.is(decode("+".charCodeAt(0)), 62);
		t.is(decode(144), -1);
	},
);

test(
	"verify the input and output of encode through decode are the same",
	async (t) => {
		t.is(decode(encode(7).charCodeAt(0)), 7);
		t.is(decode(encode(9).charCodeAt(0)), 9);
		t.is(encode(decode("z".charCodeAt(0))), "z");
		t.is(encode(decode("/".charCodeAt(0))), "/");
	},
);

test(
	"verify encodeVLQ returns the correct character",
	async (t) => {
		t.is(encodeVLQ(63), "+D");
		t.is(encodeVLQ(113), "iH");
		t.is(encodeVLQ(1_635), "mmD");
		t.is(encodeVLQ(89_454), "82uF");
	},
);

test(
	"verify decodeVLQ returns the correct code",
	async (t) => {
		t.looksLike(decodeVLQ("sJ", 0), [150, 2]);
		t.looksLike(decodeVLQ("4E", 0), [76, 2]);
		t.looksLike(decodeVLQ("6vB", 0), [765, 3]);
		t.looksLike(decodeVLQ("y8lJ", 0), [150_473, 4]);
		t.throws(() => {
			decodeVLQ("dFsg", 6);
		});
	},
);

test(
	"verify the input and output of encodeVLQ through decodeVLQ are the same",
	async (t) => {
		t.is(encodeVLQ(decodeVLQ("wD", 0)[0]), "wD");
		t.is(encodeVLQ(decodeVLQ("zD", 0)[0]), "zD");
		t.is(decodeVLQ(encodeVLQ(46), 0)[0], 46);
		t.is(decodeVLQ(encodeVLQ(765), 0)[0], 765);
	},
);
