import { describe, expect, it } from "vitest";
import { BackendKind, Rome } from "../../dist";
import { resolve } from "path";
import { fileURLToPath } from "url";

const target = process.env.CI ? "target/release/rome" : "target/debug/rome";

describe("Rome Deamon formatter", () => {
	it("should format content", async () => {
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../../..",
			target,
		);

		const rome = await Rome.create({
			backendKind: BackendKind.DAEMON,
			pathToBinary: command,
		});
		let result = await rome.formatContent("function f   () {  }", {
			filePath: "example.js",
		});

		expect(result.content).toEqual("function f() {}\n");
		expect(result.diagnostics).toEqual([]);
	});

	it("should not format and have diagnostics", async () => {
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../../..",
			target,
		);

		const rome = await Rome.create({
			backendKind: BackendKind.DAEMON,
			pathToBinary: command,
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
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../../..",
			target,
		);

		const rome = await Rome.create({
			backendKind: BackendKind.DAEMON,
			pathToBinary: command,
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
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../../..",
			target,
		);

		const rome = await Rome.create({
			backendKind: BackendKind.DAEMON,
			pathToBinary: command,
		});
		let result = await rome.formatContent("let a   ; function g () {  }", {
			filePath: "file.js",
			range: [20, 25],
		});

		expect(result.content).toEqual("function g() {}");
		expect(result.diagnostics).toEqual([]);
	});

	it("should not format content with range in debug mode", async () => {
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../../..",
			target,
		);

		const rome = await Rome.create({
			backendKind: BackendKind.DAEMON,
			pathToBinary: command,
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
}, 1500);
