import { fileURLToPath } from "node:url";
import { resolve } from "node:path";
import { equal } from "node:assert";

import { createWorkspaceWithBinary } from "../dist/index.js";

async function testWorkspace() {
	const extension = process.platform === "win32" ? ".exe" : "";
	const command = resolve(
		fileURLToPath(import.meta.url),
		"../../../..",
		`target/release/rome${extension}`,
	);

	const workspace = await createWorkspaceWithBinary(command);

	await workspace.open_file({
		path: {
			path: "test.js",
			id: 0,
		},
		content: "statement()",
		version: 0,
	});

	const printed = await workspace.format_file({
		path: {
			path: "test.js",
			id: 0,
		},
	});

	equal(printed.code, "statement();\n");

	await workspace.close_file({
		path: {
			path: "test.js",
			id: 0,
		},
	});

	workspace.destroy();
}

testWorkspace().then(() => {
	process.exit(0);
}, (err) => {
	console.error(err);
	process.exit(1);
});
