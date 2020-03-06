/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath} from '@romejs/path';
import {
  writeFile,
  readFileText,
  createDirectory,
  exists,
  unlink,
} from '@romejs/fs';
import {Dict} from '@romejs/typescript-helpers';
import {TestRunnerOptions} from '../master/testing/types';
import TestWorkerRunner from './TestWorkerRunner';
import {PartialDiagnosticAdvice} from '@romejs/diagnostics';

const SNAPSHOTS_DIR = '__rsnapshots__';
const SNAPSHOT_EXT = '.rsnap';

export default class SnapshotManager {
  constructor(runner: TestWorkerRunner, testFilename: AbsoluteFilePath) {
    this.testFilename = testFilename.join();

    const folder = testFilename.getParent().append(SNAPSHOTS_DIR);
    this.folder = folder;

    this.path = folder.append(testFilename.getBasename() + SNAPSHOT_EXT);

    this.runner = runner;
    this.options = runner.options;

    this.exists = false;
    this.raw = undefined;

    this.entries = new Map();
  }

  testFilename: string;
  folder: AbsoluteFilePath;
  path: AbsoluteFilePath;
  entries: Map<string, string>;

  runner: TestWorkerRunner;
  options: TestRunnerOptions;

  raw: undefined | string;
  exists: boolean;

  async emitDiagnostic(message: string, advice?: PartialDiagnosticAdvice) {
    await this.runner.emitDiagnostic({
      category: 'test/snapshots',
      filename: this.path.join(),
      message,
      advice,
    });
  }

  async load() {
    const {path: snapshotFilename} = this;
    if (!(await exists(snapshotFilename))) {
      return;
    }

    this.exists = true;

    // If we're force updating, pretend that no snapshots exist on disk
    if (this.options.updateSnapshots) {
      return;
    }

    const file = await readFileText(snapshotFilename);
    this.raw = file;

    const json = JSON.parse(file);

    for (const key in json) {
      this.entries.set(key, String(json[key]));
    }
  }

  async save() {
    const {folder, path} = this;

    // TODO `only` will mess this up

    // No point producing an empty snapshot file
    if (this.entries.size === 0) {
      if (this.exists) {
        if (this.options.freezeSnapshots) {
          await this.emitDiagnostic('Snapshot should not exist');
        } else {
          // Remove the snapshot file as there were none ran
          await unlink(path);
        }
      }
      return;
    }

    // Build the snapshot
    const json: Dict<string> = {};

    // Get keys and sort them so they're in a predictable order
    const keys = Array.from(this.entries.keys()).sort();

    for (const key of keys) {
      const value = this.entries.get(key);
      if (value === undefined) {
        throw new Error('Impossible');
      }

      json[key] = value;
    }

    const formatted = JSON.stringify(json, undefined, '  ');

    if (this.options.freezeSnapshots) {
      if (!this.exists) {
        await this.emitDiagnostic('Snapshot does not exist');
      } else if (formatted !== this.raw) {
        await this.emitDiagnostic('Snapshots do not match');
      }
    } else if (formatted !== this.raw) {
      // Create snapshots directory if it doesn't exist
      if (!(await exists(folder))) {
        await createDirectory(folder);
      }

      // Save the file
      await writeFile(path, formatted);
    }
  }

  toSnapshotKey(testName: string, id: number | string): string {
    if (id === 0) {
      return testName;
    } else {
      return `${testName}: ${id}`;
    }
  }

  get(key: string): undefined | string {
    return this.entries.get(key);
  }

  set(key: string, value: string) {
    this.entries.set(key, value);
  }
}
