/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath, AbsoluteFilePathSet} from '@romejs/path';
import {
  writeFile,
  readFileText,
  exists,
  unlink,
  readdir,
  lstat,
} from '@romejs/fs';
import {TestRunnerOptions} from '../master/testing/types';
import TestWorkerRunner from './TestWorkerRunner';
import {descriptions, DiagnosticDescription} from '@romejs/diagnostics';
import createSnapshotParser from './SnapshotParser';

function cleanHeading(key: string): string {
  if (key[0] === '`') {
    key = key.slice(1);
  }

  if (key[key.length - 1] === '`') {
    key = key.slice(0, -1);
  }

  return key.trim();
}

type SnapshotEntry = Map<string, Map<string, {
  language: undefined | string;
  value: string;
}>>;

export default class SnapshotManager {
  constructor(runner: TestWorkerRunner, testPath: AbsoluteFilePath) {
    this.path = testPath.getParent().append(
      `${testPath.getExtensionlessBasename()}.test.md`,
    );
    this.testPath = testPath;

    this.runner = runner;
    this.options = runner.options;

    this.exists = false;
    this.raw = '';

    this.entries = new Map();
    this.outputFiles = new Map();
    this.outputFiles.set(this.path.join(), this.path);
  }

  testPath: AbsoluteFilePath;
  path: AbsoluteFilePath;
  entries: Map<string, SnapshotEntry>;
  outputFiles: Map<string, AbsoluteFilePath>;

  runner: TestWorkerRunner;
  options: TestRunnerOptions;

  raw: string;
  exists: boolean;

  async emitDiagnostic(metadata: DiagnosticDescription) {
    await this.runner.emitDiagnostic({
      description: metadata,
      location: {
        filename: this.path.join(),
      },
    });
  }

  async loadSnapshotFixtures(dir: AbsoluteFilePath) {
    const filenames: AbsoluteFilePathSet = await readdir(dir);
    for (const filename of filenames) {
      const stats = await lstat(filename);
      if (stats.isFile() && filename.join().endsWith('.test.md')) {
        this.loadSnapshot(filename);
      } else if (stats.isDirectory()) {
        this.loadSnapshotFixtures(filename);
      }
    }
  }

  async load() {
    const {path: snapshotFile} = this;
    if (await exists(snapshotFile)) {
      this.loadSnapshot(snapshotFile);
    } else {
      // checks if there are fixtures and load them
      const fixtureDir = snapshotFile.getParent().append('test-fixtures');
      if (await exists(fixtureDir)) {
        this.loadSnapshotFixtures(fixtureDir);
      }
    }
  }

  async loadSnapshot(snapshotFile: AbsoluteFilePath) {
    if (!(await exists(snapshotFile))) {
      return;
    }

    this.exists = true;

    // If we're force updating, pretend that no snapshots exist on disk
    if (this.options.updateSnapshots) {
      return;
    }

    const file = await readFileText(snapshotFile);
    const parser = createSnapshotParser({
      path: snapshotFile,
      input: file,
    });
    this.raw = parser.input;

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
        const testName = cleanHeading(node.text);

        while (nodes.length > 0) {
          const node = nodes[0];

          if (node.type === 'Heading' && node.level === 3) {
            nodes.shift();

            const snapshotName = cleanHeading(node.text);

            const codeBlock = nodes.shift();
            if (codeBlock === undefined || codeBlock.type !== 'CodeBlock') {
              throw parser.unexpected(
                  {
                    description: descriptions.SNAPSHOTS.EXPECTED_CODE_BLOCK_AFTER_HEADING,
                    loc: node.loc,
                  },
                );
            }

            this.set({
              testName,
              snapshotName,
              language: codeBlock.language,
              value: codeBlock.text,
              snapshotFile,
            });
            continue;
          }

          if (node.type === 'CodeBlock') {
            nodes.shift();

            this.set({
              testName,
              snapshotName: '0',
              language: node.language,
              value: node.text,
              snapshotFile,
            });
          }

          break;
        }

        continue;
      }
    }
  }

  buildSnapShot(snapshotEntries: SnapshotEntry): Array<string> {
    // Build the snapshot
    let lines: Array<string> = [];

    function pushNewline() {
      if (lines[lines.length - 1] !== '') {
        lines.push('');
      }
    }

    lines.push(`# \`${this.testPath.getBasename()}\``);
    pushNewline();
    const relativeTestPath = this.runner.projectFolder.relative(this.testPath).join();
    lines.push(
      `**DO NOT MODIFY**. This file has been autogenerated. Run \`rome test ${relativeTestPath} --update-snapshots\` to update.`,
    );
    pushNewline();

    // Get test names and sort them so they are in a predictable
    const testNames = Array.from(snapshotEntries.keys()).sort();

    for (const testName of testNames) {
      const entries = snapshotEntries.get(testName);
      if (entries === undefined) {
        throw new Error('Impossible');
      }

      lines.push(`## \`${testName}\``);
      pushNewline();

      const snapshotNames = Array.from(entries.keys()).sort();

      for (const snapshotName of snapshotNames) {
        const entry = entries.get(snapshotName);
        if (entry === undefined) {
          throw new Error('Impossible');
        }

        const {value} = entry;
        const language = entry.language === undefined ? '' : entry.language;

        // If the test only has one snapshot then omit the heading
        const skipHeading = snapshotName === '0' && snapshotNames.length === 1;
        if (!skipHeading) {
          lines.push(`### \`${snapshotName}\``);
        }

        pushNewline();
        lines.push('```' + language);
        // TODO escape triple backquotes
        lines.push(value);
        lines.push('```');
        pushNewline();
      }
    }
    return lines;
  }

  async save() {
    // If there'a s focused test then we don't write or validate a snapshot
    if (this.runner.hasFocusedTest) {
      return;
    }

    // No point producing an empty snapshot file
    if (this.entries.size === 0) {
      if (this.exists) {
        if (this.options.freezeSnapshots) {
          await this.emitDiagnostic(descriptions.SNAPSHOTS.REDUNDANT);
        } else {
          // Remove the snapshot file as there were none ran
          await unlink(this.path);
        }
      }
      return;
    }

    for (const snapshotFilename of this.entries.keys()) {
      const entries = this.entries.get(snapshotFilename);
      if (entries === undefined) {
        continue;
      }
      const lines = this.buildSnapShot(entries);
      const formatted = lines.join('\n');

      if (this.options.freezeSnapshots) {
        if (!this.exists) {
          await this.emitDiagnostic(descriptions.SNAPSHOTS.MISSING);
        } else if (formatted !== this.raw) {
          await this.emitDiagnostic(descriptions.SNAPSHOTS.INCORRECT(
            this.raw,
            formatted,
          ));
        }
      } else if (formatted !== this.raw) {
        // Save the file
        const snapshotFile = this.outputFiles.get(snapshotFilename);
        if (snapshotFile !== undefined) {
          await writeFile(snapshotFile, formatted);
        }
      }
    }
  }

  get(
    testName: string,
    snapshotName: string,
    snapshotFile?: AbsoluteFilePath,
  ): undefined | string {
    if (snapshotFile === undefined) {
      snapshotFile = this.path;
    }
    const snapshotsForFile = this.entries.get(snapshotFile.join());
    if (snapshotsForFile !== undefined) {
      const entries = snapshotsForFile.get(testName);
      if (entries !== undefined) {
        const entry = entries.get(snapshotName);
        if (entry !== undefined) {
          return entry.value;
        }
      }
    }
    return undefined;
  }

  set({
    value,
    language,
    testName,
    snapshotName,
    snapshotFile,
  }: {
    value: string;
    language: undefined | string;
    testName: string;
    snapshotName: string;
    snapshotFile?: AbsoluteFilePath;
  }) {
    if (snapshotFile === undefined) {
      snapshotFile = this.path;
    }
    const snapshotFileName = snapshotFile.join();
    let snapshotsForFile = this.entries.get(snapshotFileName);
    if (snapshotsForFile === undefined) {
      snapshotsForFile = new Map();
      this.entries.set(snapshotFileName, snapshotsForFile);
    }

    let entries = snapshotsForFile.get(testName);
    if (entries === undefined) {
      entries = new Map();
      snapshotsForFile.set(testName, entries);
    }

    entries.set(snapshotName, {value, language});

    if (!this.outputFiles.has(snapshotFileName)) {
      this.outputFiles.set(snapshotFileName, snapshotFile);
    }
  }
}
