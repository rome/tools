import RSERBufferParser from "./RSERBufferParser";
import {
	AnyRSERFilePathMap,
	RSERArray,
	RSERMap,
	RSERObject,
	RSERSet,
	RSERValue,
} from "./types";
import {
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	createAbsoluteFilePath,
} from "@internal/path";
import {TestHelper, test} from "rome";
import {encodeRSERBuffer} from "./index";

function assert(t: TestHelper, val: RSERValue) {
	const buf = encodeRSERBuffer(val);

	const parser = new RSERBufferParser(new DataView(buf));
	t.true(typeof parser.decodeHeader() === "number");

	const decoded = parser.decodeValue();
	t.looksLike(decoded, val);
}

test(
	"value types",
	(t) => {
		const cases: Array<RSERValue> = [
			"foobar",
			[1, 2, 3, "a", "b", "c"],
			new Set([1, 2, 3, "a", "b", "c"]),
			new Map([[1, 2], [3, 4]]),
			Symbol.for("test"),
			new Date("2020-08-13T08:00:27.235Z"),
			true,
			false,
			null,
			undefined,
			127,
			32_767,
			2_147_483_647,
			125n,
			Math.PI,
			NaN,
			+Infinity,
			-Infinity,
			-0,
			createAbsoluteFilePath("/foo"),
			new AbsoluteFilePathSet([
				createAbsoluteFilePath("/foo"),
				createAbsoluteFilePath("/bar"),
			]),
			new AbsoluteFilePathMap([
				[createAbsoluteFilePath("/foo"), 1],
				[createAbsoluteFilePath("/bar"), 2],
			]),
			/foo/,
			/bar/g,
		];

		for (const val of cases) {
			assert(t, val);
		}
	},
);

test(
	"circular types",
	(t) => {
		const set: RSERSet = new Set();
		set.add(set);
		set.add(2);
		assert(t, set);

		const arr: RSERArray = [];
		arr.push(arr);
		assert(t, arr);

		const map: RSERMap = new Map();
		map.set("foo", map);
		assert(t, map);

		const obj: RSERObject = {};
		obj.foo = obj;
		assert(t, obj);

		const pathMap: AnyRSERFilePathMap = new AbsoluteFilePathMap();
		pathMap.set(createAbsoluteFilePath("/"), pathMap);
		assert(t, pathMap);
	},
);
