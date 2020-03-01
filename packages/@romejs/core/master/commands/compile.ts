/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {WorkerCompileResult} from '../../common/bridges/WorkerBridge';
import {createMasterCommand} from '../../commands';
import {commandCategories} from '../../commands';
import {DiagnosticsError} from '@romejs/diagnostics';
import {createUnknownFilePath} from '@romejs/path';
import {Consumer} from '@romejs/consume';
import Bundler from '../bundler/Bundler';

type Flags = {
  bundle: boolean;
};

export default createMasterCommand({
  category: commandCategories.SOURCE_CODE,
  description: 'compile a single file',

  defineFlags(c: Consumer): Flags {
    return {
      bundle: c.get('bundle').asBoolean(false),
    };
  },

  async default(req: MasterRequest, commandFlags: Flags): Promise<void> {
    const {master, reporter} = req;
    const {args} = req.query;
    req.expectArgumentLength(1);

    const resolved = await master.resolver.resolveEntryAssert(
      {
        ...req.getResolverOptionsFromFlags(),
        source: createUnknownFilePath(args[0]),
      },
      {pointer: req.getDiagnosticPointerFromFlags({type: 'arg', key: 0})},
    );

    let res: WorkerCompileResult;
    if (commandFlags.bundle) {
      const bundler = Bundler.createFromMasterRequest(req);
      res = await bundler.compile(resolved.path);
    } else {
      res = await req.requestWorkerCompile(resolved.path, 'compile');
    }

    const {code, diagnostics}: WorkerCompileResult = res;

    if (diagnostics.length > 0) {
      throw new DiagnosticsError('Compile diagnostics', diagnostics);
    }

    reporter.writeAll(code);
  },
});
