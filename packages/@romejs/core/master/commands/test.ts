/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {PartialDiagnostics} from '@romejs/diagnostics';
import {SourceMap} from '@romejs/codec-source-map';
import {Consumer} from '@romejs/consume';
import {commandCategories, createMasterCommand} from '../../commands';
import TestRunner from '../testing/TestRunner';
import Bundler from '../bundler/Bundler';
import {AbsoluteFilePath} from '@romejs/path';
import {JS_EXTENSIONS} from '../../common/fileHandlers';
import {TEST_FOLDER_NAME} from '@romejs/core/common/constants';

function isTestFile(path: AbsoluteFilePath): boolean {
  const parts = path.getSegments();

  for (const part of parts) {
    // Don't include files/directories that are prefixed with an underscore
    if (part[0] === '_' && part !== TEST_FOLDER_NAME) {
      return false;
    }

    // Don't ever include node_modules
    if (part === 'node_modules') {
      return false;
    }
  }

  // Make sure we're actually a test file
  for (const part of parts) {
    if (part === TEST_FOLDER_NAME) {
      return true;
    }
  }

  return false;
}

type Flags = {
  coverage: boolean;
  showAllCoverage: boolean;
  updateSnapshots: boolean;
  freezeSnapshots: boolean;
};

export default createMasterCommand({
  category: commandCategories.CODE_QUALITY,
  description: 'run tests',

  defineFlags(c: Consumer): Flags {
    return {
      coverage: c.get('coverage').asBoolean(true),
      showAllCoverage: c.get('showAllCoverage').asBoolean(false),
      updateSnapshots: c.get('updateSnapshots').asBoolean(false),
      freezeSnapshots: c.get('freezeSnapshots').asBoolean(false),
    };
  },

  async default(req: MasterRequest, commandFlags: Flags): Promise<void> {
    const {master, reporter} = req;
    const {flags} = req.client;

    const args: Array<string | AbsoluteFilePath> = [...req.query.args];
    if (args.length === 0) {
      const project = await req.assertClientCwdProject();
      args.push(project.folder);
    }

    const files: Array<AbsoluteFilePath> = [];

    for (const arg of args) {
      const loc = flags.cwd.resolve(arg);
      await master.projectManager.assertProject(loc);
      const matches = master.memoryFs.glob(loc, {extensions: JS_EXTENSIONS});

      for (const path of matches) {
        if (isTestFile(path)) {
          files.push(path);
        }
      }
    }

    if (files.length === 0) {
      reporter.warn('No tests ran');
      return;
    }

    reporter.info(
      `Bundling <number emphasis>${files.length}</number> test files`,
    );

    let addDiagnostics: PartialDiagnostics = [];

    const tests: Map<
      string,
      {
        code: string;
        sourceMap: SourceMap;
        path: AbsoluteFilePath;
      }
    > = new Map();

    const bundler = new Bundler(
      req,
      req.reporter.fork({
        silent: true,
      }),
      req.getBundlerConfigFromFlags({
        mocks: true,
      }),
    );

    for (const [path, res] of await bundler.bundleMultiple(files)) {
      tests.set(path.join(), {
        code: res.entry.js.content,
        sourceMap: res.entry.sourceMap.map,
        path,
      });
    }

    reporter.info(`Running tests`);

    const runner = new TestRunner({
      addDiagnostics,
      options: {
        coverage: commandFlags.coverage,
        showAllCoverage: commandFlags.showAllCoverage,
        updateSnapshots: commandFlags.updateSnapshots,
        freezeSnapshots: commandFlags.freezeSnapshots,
        verboseDiagnostics: req.query.requestFlags.verboseDiagnostics,
      },
      sources: tests,
      request: req,
    });
    await runner.init();
  },
});
