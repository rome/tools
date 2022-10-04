import { describe, expect, it } from "vitest";
import { BackendKind, Rome } from "../../dist";

describe("Rome WebAssembly formatFiles", () => {
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
});
