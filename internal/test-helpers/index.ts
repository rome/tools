/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer, consumeUnknown} from "@internal/consume";
import {consumeJSON} from "@internal/codec-json";
import {TestHelper, test, testOptions} from "rome";

import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	RelativeFilePath,
	createAbsoluteFilePath,
} from "@internal/path";
import {
	exists,
	lstat,
	readDirectory,
	readFile,
	readFileText,
} from "@internal/fs";
import {ExtendedMap} from "@internal/collections";

const dirname = testOptions.dirname ?? "";

async function isFile(path: AbsoluteFilePath): Promise<boolean> {
	return (await lstat(path)).isFile();
}

async function getOptions(dir: AbsoluteFilePath): Promise<Consumer> {
	const optionsLoc = dir.append("options.json");
	const input = await readFileText(optionsLoc);
	return consumeJSON({
		input,
		path: optionsLoc,
	});
}

export type Fixture = {
	name: Array<string>;
	dir: AbsoluteFilePath;
	options: Consumer;
	files: ExtendedMap<string, FixtureFile>;
};

export type FixtureFile = {
	relative: RelativeFilePath;
	absolute: AbsoluteFilePath;
	content: Buffer;
};

async function _getFixtures(
	opts: {
		root: AbsoluteFilePath;
		name: undefined | string;
		dir: AbsoluteFilePath;
		parts: Array<string>;
		options: Consumer;
	},
): Promise<Array<Fixture>> {
	const {name, dir, parts, options: inheritOptions} = opts;

	// Check if directory even exists
	if (!(await exists(dir))) {
		throw new Error(`The directory ${dir} doesn't exist`);
	}

	// If the name starts with a dot then we're hidden
	if (name !== undefined && name[0] === ".") {
		return [];
	}

	// Get all the filenames in the directory
	const filenames: AbsoluteFilePathSet = await readDirectory(dir);

	// Get options for this directory
	let ownOptions;
	if (filenames.has(dir.append("options.json"))) {
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
					"tests/fixtureOptions",
				);

	// An array of directories names that lead to this fixture
	const ownParts = name === undefined ? parts : [...parts, name];

	// Split up all files and directories
	const directories: Set<AbsoluteFilePath> = new Set();
	const files: Set<AbsoluteFilePath> = new Set();
	for (const path of filenames) {
		if (await isFile(path)) {
			files.add(path);
		} else {
			directories.add(path);
		}
	}

	// If there's any directories then get the fixtures from 'all of them
	if (directories.size > 0) {
		let fixtures: Array<Fixture> = [];

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
		fileContents.set(
			path.getBasename(),
			{
				relative: opts.root.relative(path).assertRelative(),
				absolute: path,
				content: await readFile(path),
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

export async function getFixtures(dir: string): Promise<Array<Fixture>> {
	const root = createAbsoluteFilePath(dir).append("test-fixtures");
	return _getFixtures({
		root,
		name: undefined,
		dir: root,
		parts: [],
		options: consumeUnknown({}, "tests/fixtureOptions"),
	});
}

export async function createFixtureTests(
	callback: (fixture: Fixture, t: TestHelper) => void | Promise<void>,
	dir: string = dirname,
): Promise<void> {
	for (const fixture of await getFixtures(dir)) {
		test(
			fixture.name,
			{},
			async (t) => {
				t.addToAdvice({
					type: "log",
					category: "info",
					text: "Fixture options",
				});

				t.addToAdvice({
					type: "inspect",
					data: fixture.options.asJSONPropertyValue(),
				});

				t.addToAdvice({
					type: "log",
					category: "info",
					text: "Fixture files",
				});

				t.addToAdvice({
					type: "list",
					list: Array.from(
						fixture.files,
						([basename, info]) =>
							`<filelink target="${info.absolute}">${basename}</filelink>`
						,
					),
				});

				await callback(fixture, t);

				fixture.options.enforceUsedProperties();
			},
		);
	}
}

export * from "./integration";
