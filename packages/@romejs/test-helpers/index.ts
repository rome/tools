/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer, consumeUnknown} from '@romejs/consume';
import {consumeJSON} from '@romejs/codec-json';
import {TestHelper, test, testOptions} from '@romejs-runtime/rome';

import {
  AbsoluteFilePath,
  AbsoluteFilePathSet,
  UnknownFilePath,
  createAbsoluteFilePath,
} from '@romejs/path';
import {exists, lstat, readFile, readFileText, readdir} from '@romejs/fs';

const dirname = testOptions.dirname === undefined ? '' : testOptions.dirname;

async function isFile(path: AbsoluteFilePath): Promise<boolean> {
  return (await lstat(path)).isFile();
}

async function getOptions(dir: AbsoluteFilePath): Promise<Consumer> {
  const optionsLoc = dir.append('options.json');
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
  files: Map<string, FixtureFile>;
};

export type FixtureFile = {
  relative: UnknownFilePath;
  absolute: AbsoluteFilePath;
  content: Buffer;
};

async function _getFixtures(opts: {
  name: undefined | string;
  dir: AbsoluteFilePath;
  parts: Array<string>;
  options: Consumer;
}): Promise<Array<Fixture>> {
  const {name, dir, parts, options: inheritOptions} = opts;

  // Check if directory even exists
  if (!(await exists(dir))) {
    throw new Error(`The directory ${dir} doesn't exist`);
  }

  // If the name starts with a dot then we're hidden
  if (name !== undefined && name[0] === '.') {
    return [];
  }

  // Get all the filenames in the directory
  const filenames: AbsoluteFilePathSet = await readdir(dir);

  // Get options for this folder
  let ownOptions;
  if (filenames.has(dir.append('options.json'))) {
    ownOptions = await getOptions(dir);
  }

  // Merge options
  const options: Consumer = ownOptions === undefined
    ? inheritOptions
    : consumeUnknown({
      ...inheritOptions.asUnknownObject(),
      ...ownOptions.asUnknownObject(),
    }, 'tests/fixtureOptions');

  // An array of folders names that lead to this fixture
  const ownParts = name === undefined ? parts : [...parts, name];

  // Split up all files and folders
  const folders: Set<AbsoluteFilePath> = new Set();
  const files: Set<AbsoluteFilePath> = new Set();
  for (const path of filenames) {
    if (await isFile(path)) {
      files.add(path);
    } else {
      folders.add(path);
    }
  }

  // If there's any folders then get the fixtures from 'all of them
  if (folders.size > 0) {
    let fixtures: Array<Fixture> = [];

    for (const path of folders) {
      fixtures = fixtures.concat(await _getFixtures({
        name: path.getBasename(),
        dir: path,
        parts: ownParts,
        options,
      }));
    }

    return fixtures;
  }

  // Get the contents of all the files
  const fileContents: Map<string, FixtureFile> = new Map();
  for (const path of filenames) {
    fileContents.set(path.getBasename(), {
      relative: dir.relative(path),
      absolute: path,
      content: await readFile(path),
    });
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
  return _getFixtures({
    name: undefined,
    dir: createAbsoluteFilePath(dir).append('test-fixtures'),
    parts: [],
    options: consumeUnknown({}, 'tests/fixtureOptions'),
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
          type: 'log',
          category: 'info',
          message: 'Fixture options',
        });

        t.addToAdvice({
          type: 'inspect',
          data: fixture.options.asJSONPropertyValue(),
        });

        t.addToAdvice({
          type: 'log',
          category: 'info',
          message: 'Fixture files',
        });

        t.addToAdvice(
          {
            type: 'list',
            list: Array.from(
              fixture.files,
              ([basename, info]) => `<filelink target="${info.absolute}">${basename}</filelink>`,
            ),
          },
        );

        await callback(fixture, t);
      },
    );
  }
}
