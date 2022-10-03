import { describe, expect, it } from "vitest";
import { BackendKind, Rome } from "../../dist";

describe("Rome WebAssembly formatContent", () => {
	it("should format content", async () => {
		const rome = await Rome.create({
			backendKind: BackendKind.NODE,
		});

		let result = await rome.formatContent("function f   () {  }", {
			filePath: "example.js",
		});

		expect(result.content).toEqual("function f() {}\n");
		expect(result.diagnostics).toEqual([]);
	});

	it("should not format and have diagnostics", async () => {
		const rome = await Rome.create({
			backendKind: BackendKind.NODE,
		});

		let content = "function   () {  }";
		let result = await rome.formatContent(content, {
			filePath: "example.js",
		});

		expect(result.content).toEqual(content);
		expect(result.diagnostics).toHaveLength(1);
		expect(result.diagnostics[0].title[0].content).toContain(
			"expected a name for the function in a function declaration, but found none",
		);
		expect(result.diagnostics).toMatchSnapshot("syntax error");
	});

	it("should format content in debug mode", async () => {
		const rome = await Rome.create({
			backendKind: BackendKind.NODE,
		});

		let result = await rome.formatContent("function f() {}", {
			filePath: "example.js",
			debug: true,
		});

		expect(result.content).toEqual("function f() {}\n");
		expect(result.diagnostics).toEqual([]);
		expect(result.ir).toMatchInlineSnapshot(
			'"[\\"function f\\", group([\\"()\\"]), \\" {}\\", hard_line_break]"',
		);
	});

	it("should not format content with range", async () => {
		const rome = await Rome.create({
			backendKind: BackendKind.NODE,
		});

		let result = await rome.formatContent("let a   ; function g () {  }", {
			filePath: "file.js",
			range: [20, 25],
		});

		expect(result.content).toEqual("function g() {}");
		expect(result.diagnostics).toEqual([]);
	});

	it("should not format content with range in debug mode", async () => {
		const rome = await Rome.create({
			backendKind: BackendKind.NODE,
		});

		let result = await rome.formatContent("let a   ; function g () {  }", {
			filePath: "file.js",
			range: [20, 25],
			debug: true,
		});

		expect(result.content).toEqual("function g() {}");
		expect(result.diagnostics).toEqual([]);
		expect(result.ir).toMatchInlineSnapshot(
			`
			"[
			  group([\\"let a\\"]),
			  \\";\\",
			  hard_line_break,
			  \\"function g\\",
			  group([\\"()\\"]),
			  \\" {}\\",
			  hard_line_break
			]"
		`,
		);
	});

	it("should format content with custom configuration (8 spaces, single quotes, preserve quotes)", async () => {
		const rome = await Rome.create({
			backendKind: BackendKind.NODE,
		});

		let content = `function   f() { return { "foo": 'bar' }  }`;
		let formatted = `function f() {
        return { 'foo': 'bar' };
}
`;

		await rome.applyConfiguration({
			formatter: {
				indentStyle: "space",
				indentSize: 8,
			},
			javascript: {
				formatter: {
					quoteStyle: "single",
					quoteProperties: "preserve",
				},
			},
		});

		let result = await rome.formatContent(content, {
			filePath: "example.js",
		});

		expect(result.content).toEqual(formatted);
	});
});
