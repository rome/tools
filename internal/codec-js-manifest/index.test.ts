import {test} from "rome";
import {normalizeManifest} from ".";
import {consumeUnknown} from "@internal/consume";
import {AbsoluteFilePath} from "@internal/path";

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
			"parse/manifest",
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
			"parse/manifest",
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
			"parse/manifest",
		);
		t.snapshot(await normalizeManifest(PATH, manifest, []));
	},
);
