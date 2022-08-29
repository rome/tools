import { describe, it, expect } from "vitest";
import { BackendKind, Rome } from "../dist";

describe("Rome WebAssembly formatter", () => {
	it("should not format files", async () => {
		const rome = await Rome.create({
			backendKind: BackendKind.NODE,
		});

		let result = await rome.formatFiles(["./path/to/file.js"]);

		expect(result.content).toEqual("");
		expect(result.diagnostics).toEqual([]);
	});

	it("should not format files in debug mode", async () => {
		const rome = await Rome.create({
			backendKind: BackendKind.NODE,
		});

		let result = await rome.formatFiles(["./path/to/file.js"], {
			debug: true,
		});

		expect(result.content).toEqual("");
		expect(result.diagnostics).toEqual([]);
		expect(result.ir).toEqual("");
	});

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
			'"[\\"function\\", \\" \\", \\"f\\", group([\\"(\\", \\")\\"]), \\" \\", \\"{\\", \\"}\\", hard_line_break]"',
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
			  group([\\"let\\", \\" \\", \\"a\\"]),
			  \\";\\",
			  hard_line_break,
			  \\"function\\",
			  \\" \\",
			  \\"g\\",
			  group([\\"(\\", \\")\\"]),
			  \\" \\",
			  \\"{\\",
			  \\"}\\",
			  hard_line_break
			]"
		`,
		);
	});
});

describe("Rome parser", () => {
	it("should not parse content", async () => {
		const rome = await Rome.create({
			backendKind: BackendKind.NODE,
		});

		let result = await rome.parseContent("function f() {}", {
			filePath: "example.js",
		});

		expect(result.ast).toEqual("");
		expect(result.cst).toEqual("");
		expect(result.diagnostics).toEqual([]);
	});
});
