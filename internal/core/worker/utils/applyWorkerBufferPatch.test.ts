import {test} from "rome";
import {applyWorkerBufferPatch} from "./applyWorkerBufferPatch";
import {WorkerBufferPatch} from "@internal/core/common/bridges/WorkerBridge";
import {ob1Coerce0} from "@internal/ob1";
import {dedent} from "@internal/string-utils";

function makeRange(
	startLine: number,
	startChar: number,
	endLine: number,
	endChar: number,
) {
	return {
		start: {
			line: ob1Coerce0(startLine),
			character: ob1Coerce0(startChar),
		},
		end: {
			line: ob1Coerce0(endLine),
			character: ob1Coerce0(endChar),
		},
	};
}

test(
	"applyWorkerBufferPatch can insert text",
	async (t) => {
		const original = "this is a test";
		const patch = {
			range: makeRange(0, 10, 0, 10),
			text: "patch ",
		};

		t.inlineSnapshot(
			applyWorkerBufferPatch(original, patch),
			"this is a patch test",
		);
	},
);

test(
	"applyWorkerBufferPatch can append text",
	async (t) => {
		const original = "rome";
		const patch: WorkerBufferPatch = {
			range: makeRange(0, 4, 0, 4),
			text: "\nfoo",
		};

		t.inlineSnapshot(applyWorkerBufferPatch(original, patch), "rome\nfoo");
	},
);

test(
	"applyWorkerBufferPatch handles characters represented by two code units",
	async (t) => {
		const original = dedent`
			test
			a𐐀b
			rome
		`;
		const patch: WorkerBufferPatch = {
			range: makeRange(1, 3, 1, 4),
			text: "foo",
		};

		t.inlineSnapshot(
			applyWorkerBufferPatch(original, patch),
			"test\na\u{10400}foo\nrome",
		);
	},
);

test(
	"applyWorkerBufferPatch can patch multiline text",
	async (t) => {
		const original = dedent`
			let foo = "test";
			foo = "wrong";
			console.log(foo);
		`;
		const patch = {
			range: makeRange(0, 11, 1, 12),
			text: "right",
		};

		t.inlineSnapshot(
			applyWorkerBufferPatch(original, patch),
			'let foo = "right";\nconsole.log(foo);',
		);
	},
);

test(
	"applyWorkerBufferPatch can delete a line",
	async (t) => {
		const original = dedent`
			let foo = "test";
			foo = "wrong";
			console.log(foo);
		`;
		const patch = {
			range: makeRange(1, 0, 2, 0),
			text: "",
		};

		t.inlineSnapshot(
			applyWorkerBufferPatch(original, patch),
			'let foo = "test";\nconsole.log(foo);',
		);
	},
);

test(
	"applyWorkerBufferPatch handles all end-of-line sequences",
	async (t) => {
		const original = "zero\none\r\ntwo\rthree\r\nfour\nfive";
		const patch = {
			range: makeRange(4, 0, 4, 4),
			text: "FOUR",
		};

		t.inlineSnapshot(
			applyWorkerBufferPatch(original, patch),
			"zero\none\r\ntwo\rthree\r\nFOUR\nfive",
		);
	},
);

test(
	"applyWorkerBufferPatch returns undefined for start > end",
	async (t) => {
		const original = "test";
		const patch = {
			range: makeRange(2, 0, 1, 0),
			text: "wrong",
		};

		t.is(applyWorkerBufferPatch(original, patch), undefined);
	},
);

test(
	"applyWorkerBufferPatch returns undefined for unfound start position",
	async (t) => {
		const original = "zero\none\ntwo\nthree";
		const patch = {
			range: makeRange(1, 99, 2, 0),
			text: "wrong",
		};

		t.is(applyWorkerBufferPatch(original, patch), undefined);
	},
);

test(
	"applyWorkerBufferPatch returns undefined for unfound end position",
	async (t) => {
		const original = "zero\none\ntwo\nthree";
		const patch = {
			range: makeRange(2, 0, 2, 99),
			text: "wrong",
		};

		t.is(applyWorkerBufferPatch(original, patch), undefined);
	},
);
