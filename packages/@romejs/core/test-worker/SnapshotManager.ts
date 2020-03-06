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
import {TestRunnerOptions} from '../master/testing/types';
import TestWorkerRunner from './TestWorkerRunner';
import {PartialDiagnosticAdvice} from '@romejs/diagnostics';
import createSnapshotParser from './SnapshotParser';

const SNAPSHOTS_DIR = '__rsnapshots__';

function cleanHeading(key: string): string {
  if (key[0] === '`') {
    key = key.slice(1);
  }

  if (key[key.length - 1] === '`') {
    key = key.slice(-1);
  }

  return key.trim();
}

export default class SnapshotManager {
  constructor(runner: TestWorkerRunner, testPath: AbsoluteFilePath) {
    const folder = testPath.getParent().append(SNAPSHOTS_DIR);
    this.folder = folder;

    this.path = folder.append(testPath.getExtensionlessBasename() + '.snap.md');
    this.testPath = testPath;

    this.runner = runner;
    this.options = runner.options;

    this.exists = false;
    this.raw = undefined;

    this.entries = new Map();
  }

  testPath: AbsoluteFilePath;
  folder: AbsoluteFilePath;
  path: AbsoluteFilePath;
  entries: Map<
    string,
    {
      language: undefined | string;
      value: string;
    }
  >;

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

    const parser = createSnapshotParser({
      path: snapshotFilename,
      input: file,
    });

    const nodes = parser.parse();

    while (nodes.length > 0) {
      const node = nodes.shift();
      if (node === undefined) {
        throw new Error('Impossible');
      }

      if (node.type === 'Heading' && node.level === 1) {
        // Title
        continue;
      }

      if (node.type === 'Heading' && node.level === 2) {
        const codeBlock = nodes.shift();
        if (codeBlock === undefined || codeBlock.type !== 'CodeBlock') {
          throw parser.unexpected({
            message: 'Expected a code block after this heading',
            loc: node.loc,
          });
        }

        this.entries.set(cleanHeading(node.text), {
          language: codeBlock.language,
          value: codeBlock.text,
        });
        continue;
      }

      throw parser.unexpected({
        message: 'Unexpected node',
        loc: node.loc,
      });
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
    let lines: Array<string> = [];
    lines.push(`# \`${this.testPath.getBasename()}\``);
    lines.push('');

    // Get keys and sort them so they're in a predictable order
    const keys = Array.from(this.entries.keys()).sort();

    for (const key of keys) {
      const entry = this.entries.get(key);
      if (entry === undefined) {
        throw new Error('Impossible');
      }

      const {value} = entry;
      const language = entry.language === undefined ? '' : entry.language;

      lines.push(`## \`${key}\``);
      lines.push('');
      lines.push('```' + language);
      // TODO escape triple backquotes
      lines.push(value);
      lines.push('```');
      lines.push('');
    }

    const formatted = lines.join('\n');

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
    const entry = this.entries.get(key);
    if (entry !== undefined) {
      return entry.value;
    }
  }

  set(key: string, value: string, language: undefined | string) {
    this.entries.set(key, {value, language});
  }
}
