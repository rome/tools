import { describe, expect, it } from "vitest";
import { BackendKind, Rome } from "../../dist";
import { resolve } from "path";
import { fileURLToPath } from "url";

const target = process.env.CI ? "target/release/rome" : "target/debug/rome";

describe("Rome Deamon formatter", () => {
	it("should not format files", async () => {
		const pathToBinary = resolve(
			fileURLToPath(import.meta.url),
			"../../../../..",
			target,
		);

		console.log(pathToBinary);

		const rome = await Rome.create({
			backendKind: BackendKind.DAEMON,
			pathToBinary,
		});

		let result = await rome.formatFiles(["./path/to/file.js"]);

		expect(result.content).toEqual("");
		expect(result.diagnostics).toEqual([]);
	});

	it("should not format files in debug mode", async () => {
		const pathToBinary = resolve(
			fileURLToPath(import.meta.url),
			"../../../../..",
			target,
		);

		const rome = await Rome.create({
			backendKind: BackendKind.DAEMON,
			pathToBinary,
		});

		let result = await rome.formatFiles(["./path/to/file.js"], {
			debug: true,
		});

		expect(result.content).toEqual("");
		expect(result.diagnostics).toEqual([]);
		expect(result.ir).toEqual("");
	});
});
