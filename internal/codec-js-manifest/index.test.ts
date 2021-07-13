import {test} from "rome";
import {normalizeManifest} from ".";
import {consumeUnknown} from "@internal/consume";
import {createAbsoluteFilePath} from "@internal/path";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

const PATH = createAbsoluteFilePath("/test");

function getManifest() {
	return {
		name: "test-name",
	};
}

test(
	"export string condition",
	async (t) => {
		const manifest = consumeUnknown(
			{
				...getManifest(),
				exports: {
					"./foo": "./bar",
				},
			},
			DIAGNOSTIC_CATEGORIES.parse,
			"manifest",
		);
		t.snapshot(
			await normalizeManifest({path: PATH, consumer: manifest, projects: []}),
		);
	},
);

test(
	"export subpath condition",
	async (t) => {
		const manifest = consumeUnknown(
			{
				...getManifest(),
				exports: {
					"./foo": {
						node: "./bar",
					},
				},
			},
			DIAGNOSTIC_CATEGORIES.parse,
			"manifest",
		);
		t.snapshot(
			await normalizeManifest({path: PATH, consumer: manifest, projects: []}),
		);
	},
);

test(
	"export nested condition",
	async (t) => {
		const manifest = consumeUnknown(
			{
				...getManifest(),
				exports: {
					node: {
						"./foo": "./bar",
					},
				},
			},
			DIAGNOSTIC_CATEGORIES.parse,
			"manifest",
		);
		t.snapshot(
			await normalizeManifest({path: PATH, consumer: manifest, projects: []}),
		);
	},
);
