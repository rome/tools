import RSERBufferWriter from "./RSERBufferWriter";
import RSERBufferAssembler from "./RSERBufferAssembler";
import RSERBufferParser from "./RSERBufferParser";
import {
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	createAbsoluteFilePath,
} from "@internal/path";
import {test} from "rome";

test(
	"value types",
	(t) => {
		const cases = [
			"foobar",
			[1, 2, 3, "a", "b", "c"],
			new Set([1, 2, 3, "a", "b", "c"]),
			new Map([[1, 2], ["a", "b"]]),
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
			const {payloadLength, messageLength} = RSERBufferAssembler.measure(val);
			const writer = RSERBufferWriter.allocate(messageLength);
			writer.encodeHeader(payloadLength);
			writer.encodeValue(val);
			t.is(writer.getWritableSize(), 0);

			const parser = new RSERBufferParser(writer.view);
			t.true(typeof parser.decodeHeader() === "number");

			const decoded = parser.decodeValue();
			t.looksLike(decoded, val);
		}
	},
);
