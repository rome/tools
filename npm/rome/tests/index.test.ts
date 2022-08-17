import { describe, it, expect } from "vitest";
import { Rome } from "../src/index";

describe("Rome formatter", () => {
	it("should not format files", async () => {
		const rome = new Rome();

		let result = await rome.formatFiles(["./path/to/file.js"]);

		expect(result.content).toEqual("");
		expect(result.errors).toEqual([]);
	});

	it("should not format content", async () => {
		const rome = new Rome();

		let result = await rome.formatContent("function f() {}");

		expect(result.content).toEqual("");
		expect(result.errors).toEqual([]);
	});

	it("should not format content with range", async () => {
		const rome = new Rome();

		let result = await rome.formatContent("function f() {}", {
			range: [5, 10],
		});

		expect(result.content).toEqual("");
		expect(result.errors).toEqual([]);
	});
});

describe("Rome parser", () => {
	it("should not parse content", async () => {
		const rome = new Rome();

		let result = await rome.parseContent("function f() {}", {
			filePath: "example.js",
		});

		expect(result.ast).toEqual("");
		expect(result.cst).toEqual("");
		expect(result.errors).toEqual([]);
	});
});
