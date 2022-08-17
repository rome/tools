import { describe, it, expect } from "vitest";
import { Rome } from "../src/index";

describe("Rome WebAssembly formatter", () => {
	it("should not format files", async () => {
		const rome = await Rome.create();

		let result = await rome.formatFiles(["./path/to/file.js"]);

		expect(result.content).toEqual("");
		expect(result.errors).toEqual([]);
	});

	it("should not format files in debug mode", async () => {
		const rome = await Rome.create();

		let result = await rome.formatFiles(["./path/to/file.js"], {
			debug: true,
		});

		expect(result.content).toEqual("");
		expect(result.errors).toEqual([]);
		expect(result.ir).toEqual("");
	});

	it("should format content", async () => {
		const rome = await Rome.create();

		let result = await rome.formatContent("function f   () {  }", {
			filePath: "example.js",
		});

		expect(result.content).toEqual("function f() {}\n");
		expect(result.errors).toEqual([]);
	});

	it("should format content in debug mode", async () => {
		const rome = await Rome.create();

		let result = await rome.formatContent("function f() {}", {
			filePath: "example.js",
			debug: true,
		});

		expect(result.content).toEqual("function f() {}\n");
		expect(result.errors).toEqual([]);
		expect(result.ir).toEqual(
			'["function", " ", "f", group(["(", ")"]), " ", "{", "}", hard_line_break]',
		);
	});

	it("should not format content with range", async () => {
		const rome = await Rome.create();

		let result = await rome.formatContent("let a   ; function g () {  }", {
			filePath: "file.js",
			range: [20, 25],
		});

		expect(result.content).toEqual("function g() {}");
		expect(result.errors).toEqual([]);
	});

	it("should not format content with range in debug mode", async () => {
		const rome = await Rome.create();

		let result = await rome.formatContent("let a   ; function g () {  }", {
			filePath: "file.js",
			range: [20, 25],
			debug: true,
		});

		console.log(result.ir);
		expect(result.content).toEqual("function g() {}");
		expect(result.errors).toEqual([]);
		expect(result.ir).toEqual(
			`[
  "let",
  " ",
  group(["a"]),
  ";",
  hard_line_break,
  "function",
  " ",
  "g",
  group(["(", ")"]),
  " ",
  "{",
  "}",
  hard_line_break
]`,
		);
	});
});

describe("Rome parser", () => {
	it("should not parse content", async () => {
		const rome = await Rome.create();

		let result = await rome.parseContent("function f() {}", {
			filePath: "example.js",
		});

		expect(result.ast).toEqual("");
		expect(result.cst).toEqual("");
		expect(result.errors).toEqual([]);
	});
});
