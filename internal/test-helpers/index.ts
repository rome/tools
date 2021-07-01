/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import "@internal/core";
import {Consumer, consumeUnknown} from "@internal/consume";
import {json} from "@internal/codec-config";
import {TestHelper, test, testOptions} from "rome";
import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	RelativePath,
	createAbsoluteFilePath,
} from "@internal/path";
import {ExtendedMap} from "@internal/collections";
import {DIAGNOSTIC_CATEGORIES, catchDiagnostics} from "@internal/diagnostics";
import {decodeUTF8} from "@internal/binary";
import {removeCarriageReturn} from "@internal/string-utils";
import {AsyncVoidCallback} from "@internal/typescript-helpers";
import {printDiagnosticsToString} from "@internal/cli-diagnostics";

const dirname = testOptions.dirname ?? "";

async function isFile(path: AbsoluteFilePath): Promise<boolean> {
	return (await path.lstat()).isFile();
}

async function getOptions(dir: AbsoluteFilePath): Promise<Consumer> {
	const optionsLoc = dir.append("options.json");
	const input = await optionsLoc.readFileText();
	return json.consumeValue({
		input,
		path: optionsLoc,
	});
}

export type Fixture = {
	name: string[];
	dir: AbsoluteFilePath;
	options: Consumer;
	files: ExtendedMap<string, FixtureFile>;
};

export type FixtureFile = {
	relative: RelativePath;
	absolute: AbsoluteFilePath;
	content: ArrayBufferView;
	contentAsText: () => string;
};

async function _getFixtures(
	opts: {
		root: AbsoluteFilePath;
		name: undefined | string;
		dir: AbsoluteFilePath;
		parts: string[];
		options: Consumer;
	},
): Promise<Fixture[]> {
	const {name, dir, parts, options: inheritOptions} = opts;

	// Check if directory even exists
	if (await dir.notExists()) {
		throw new Error(`The directory ${dir} doesn't exist`);
	}

	// If the name starts with a dot then we're hidden
	if (name !== undefined && name[0] === ".") {
		return [];
	}

	const paths: AbsoluteFilePathSet = await dir.readDirectory();

	// Get options for this directory
	let ownOptions;
	if (paths.has(dir.append("options.json"))) {
		ownOptions = await getOptions(dir);
	}

	// Merge options
	const options: Consumer =
		ownOptions === undefined
			? inheritOptions
			: consumeUnknown(
					{
						...inheritOptions.asUnknownObject(),
						...ownOptions.asUnknownObject(),
					},
					DIAGNOSTIC_CATEGORIES["tests/fixtureOptions"],
				);

	// An array of directories names that lead to this fixture
	const ownParts = name === undefined ? parts : [...parts, name];

	// Split up all files and directories
	const directories: Set<AbsoluteFilePath> = new Set();
	const files: Set<AbsoluteFilePath> = new Set();
	for (const path of paths) {
		if (await isFile(path)) {
			files.add(path);
		} else {
			directories.add(path);
		}
	}

	// If there's any directories then get the fixtures from all of them
	if (directories.size > 0) {
		let fixtures: Fixture[] = [];

		for (const path of directories) {
			fixtures = fixtures.concat(
				await _getFixtures({
					root: opts.root,
					name: path.getBasename(),
					dir: path,
					parts: ownParts,
					options,
				}),
			);
		}

		return fixtures;
	}

	// Get the contents of all the files
	const fileContents: ExtendedMap<string, FixtureFile> = new ExtendedMap(
		"fileContents",
	);
	for (const path of files) {
		const content = await path.readFile();

		fileContents.set(
			path.getBasename(),
			{
				relative: opts.root.relative(path).assertRelative(),
				absolute: path,
				content,
				contentAsText() {
					return removeCarriageReturn(decodeUTF8(content));
				},
			},
		);
	}

	// Create the fixture
	return [
		{
			name: ownParts,
			dir,
			options,
			files: fileContents,
		},
	];
}

export async function getFixtures(dir: string = dirname): Promise<Fixture[]> {
	const root = createAbsoluteFilePath(dir).append("test-fixtures");
	return _getFixtures({
		root,
		name: undefined,
		dir: root,
		parts: [],
		options: consumeUnknown({}, DIAGNOSTIC_CATEGORIES["tests/fixtureOptions"]),
	});
}

export async function createFixtureTests(
	callback: (fixture: Fixture, t: TestHelper) => void | Promise<void>,
	dir: string = dirname,
	disabled: boolean = false,
): Promise<void> {
	for (const fixture of await getFixtures(dir)) {
		test(
			fixture.name,
			{},
			async (t) => {
				if (disabled) {
					return;
				}

				t.addToAdvice({
					type: "log",
					category: "info",
					text: "Fixture options",
				});

				t.addToAdvice({
					type: "inspect",
					data: fixture.options.asUnknown(),
				});

				t.addToAdvice({
					type: "log",
					category: "info",
					text: "Fixture files",
				});

				t.addToAdvice({
					type: "list",
					list: Array.from(fixture.files, ([basename]) => `${basename}`),
				});

				await callback(fixture, t);

				fixture.options.enforceUsedProperties();
			},
		);
	}
}

export * from "./integration";

export async function assertDiagnostics(
	t: TestHelper,
	callback: AsyncVoidCallback,
): Promise<void> {
	const {diagnostics} = await catchDiagnostics(callback);

	if (diagnostics === undefined) {
		throw new Error("Expected thrown diagnostics");
	} else {
		t.snapshot(
			await printDiagnosticsToString({
				diagnostics,
				suppressions: [],
			}),
		);
	}
}
