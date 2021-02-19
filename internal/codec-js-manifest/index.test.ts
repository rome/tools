import {test} from "rome";
import {normalizeManifest} from ".";
import {consumeUnknown} from "@internal/consume";
import {AbsoluteFilePath} from "@internal/path";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

const PATH = {} as AbsoluteFilePath;

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
		t.snapshot(await normalizeManifest(PATH, manifest, []));
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
		t.snapshot(await normalizeManifest(PATH, manifest, []));
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
		t.snapshot(await normalizeManifest(PATH, manifest, []));
	},
);
