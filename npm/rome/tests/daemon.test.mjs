import { describe, expect, it } from "vitest";
import { ClientKind, Rome } from "../dist";
import { resolve } from "path";
import { fileURLToPath } from "url";

const target = process.env.CI ? "target/release/rome" : "target/debug/rome";

describe("Rome Deamon formatter", () => {
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
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../..",
			target,
		);

		const rome = await Rome.create(ClientKind.DAEMON, command);
		let result = await rome.formatContent("function f   () {  }", {
			filePath: "example.js",
		});

		expect(result.content).toEqual("function f() {}\n");
		expect(result.errors).toEqual([]);
	});

	it("should format content in debug mode", async () => {
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../..",
			target,
		);

		const rome = await Rome.create(ClientKind.DAEMON, command);

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
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../..",
			target,
		);

		const rome = await Rome.create(ClientKind.DAEMON, command);
		let result = await rome.formatContent("let a   ; function g () {  }", {
			filePath: "file.js",
			range: { start: 20, end: 25 },
		});

		expect(result.content).toEqual("function g() {}");
		expect(result.errors).toEqual([]);
	});

	it("should not format content with range in debug mode", async () => {
		const command = resolve(
			fileURLToPath(import.meta.url),
			"../../../..",
			target,
		);

		const rome = await Rome.create(ClientKind.DAEMON, command);
		let result = await rome.formatContent("let a   ; function g () {  }", {
			filePath: "file.js",
			range: { start: 20, end: 25 },
			debug: true,
		});

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
