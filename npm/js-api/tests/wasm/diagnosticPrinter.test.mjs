import { describe, expect, it } from "vitest";
import { DiagnosticPrinter } from "@rometools/wasm-nodejs";

describe("Rome WebAssembly DiagnosticPrinter", () => {
	it("should format content", async () => {
		const SOURCE_CODE = `const variable = expr();

if(expr()) {
    statement();
}`;
		const printer = new DiagnosticPrinter("file.js", SOURCE_CODE);

		printer.print({
			advices: {
				advices: [],
			},
			category: "parse",
			description: "error description content",
			location: {
				path: {
					File: {
						PathAndId: {
							file_id: 0,
							path: "file.js",
						},
					},
				},
				source_code: null,
				span: [31, 37],
			},
			message: [
				{
					content: "error message content",
					elements: [],
				},
			],
			severity: "Error",
			source: null,
			tags: [],
			verbose_advices: {
				advices: [],
			},
		});

		printer.print({
			advices: {
				advices: [],
			},
			category: "parse",
			description: "error description content",
			location: {
				path: {
					File: {
						PathAndId: {
							file_id: 0,
							path: "file.js",
						},
					},
				},
				source_code: null,
				span: [46, 58],
			},
			message: [
				{
					content: "error message content",
					elements: [],
				},
			],
			severity: "Error",
			source: null,
			tags: [],
			verbose_advices: {
				advices: [],
			},
		});

		const html = printer.finish();
		expect(html).toMatchSnapshot("HTML diagnostic");
	});
});
