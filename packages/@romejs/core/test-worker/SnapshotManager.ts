/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath} from '@romejs/path';
import {writeFile, readFileText, createDirectory, exists} from '@romejs/fs';
import {Dict} from '@romejs/typescript-helpers';

const SNAPSHOTS_DIR = '__rsnapshots__';
const SNAPSHOT_EXT = '.rsnap';

export default class SnapshotManager {
  constructor(testFilename: AbsoluteFilePath, forceUpdate: boolean) {
    this.testFilename = testFilename.join();

    const snapshotFolder = testFilename.getParent().append(SNAPSHOTS_DIR);
    this.snapshotFolder = snapshotFolder;

    this.snapshotFilename = snapshotFolder.append(
      testFilename.getBasename() + SNAPSHOT_EXT,
    );

    this.forceUpdate = forceUpdate;

    this.snapshots = new Map();
  }

  testFilename: string;
  forceUpdate: boolean;
  snapshotFolder: AbsoluteFilePath;
  snapshotFilename: AbsoluteFilePath;
  snapshots: Map<string, string>;

  async load() {
    const {snapshotFilename} = this;
    if (!(await exists(snapshotFilename))) {
      return undefined;
    }

    // If we're force updating, pretend that no snapshots exist on disk
    if (this.forceUpdate) {
      return undefined;
    }

    const file = await readFileText(snapshotFilename);
    const json = JSON.parse(file);

    for (const key in json) {
      this.snapshots.set(key, String(json[key]));
    }
  }

  async save() {
    // No point producing an empty snapshot file
    if (this.snapshots.size === 0) {
      return undefined;
    }

    const {snapshotFolder, snapshotFilename} = this;

    // Create snapshots directory if it doesn't exist
    if (!(await exists(snapshotFolder))) {
      await createDirectory(snapshotFolder);
    }

    // Build the snapshot
    const json: Dict<string> = {};
    for (const [key, value] of this.snapshots) {
      json[key] = value;
    }
    const formatted = JSON.stringify(json, undefined, '  ');

    // Save the file
    await writeFile(snapshotFilename, formatted);
  }

  toSnapshotKey(testName: string, id: number | string): string {
    if (id === 0) {
      return testName;
    } else {
      return `${testName}: ${id}`;
    }
  }

  get(key: string): undefined | string {
    return this.snapshots.get(key);
  }

  set(key: string, value: string) {
    this.snapshots.set(key, value);
  }
}
