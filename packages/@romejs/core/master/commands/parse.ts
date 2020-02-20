/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {Consumer} from '@romejs/consume';
import {DiagnosticsError} from '@romejs/diagnostics';
import {createMasterCommand} from '../../commands';
import {commandCategories} from '../../commands';
import {createUnknownFilePath} from '@romejs/path';
import {ConstSourceType} from '@romejs/js-ast';

type Flags = {
  showDespiteDiagnostics: boolean;
  compact: boolean;
  sourceType: undefined | ConstSourceType;
};

export default createMasterCommand({
  category: commandCategories.SOURCE_CODE,
  description: 'parse a single file and dump its ast',

  defineFlags(c: Consumer): Flags {
    return {
      showDespiteDiagnostics: c.get('showDespiteDiagnostics').asBoolean(false),
      compact: c.get('compact').asBoolean(true),
      sourceType: c.get('sourceType').asStringSetOrVoid(['module', 'script']),
    };
  },

  async default(req: MasterRequest, commandFlags: Flags): Promise<void> {
    const {master, reporter} = req;
    const {args} = req.query;
    const {flags} = req.client;
    req.expectArgumentLength(1);

    const filename = await master.resolver.resolveEntryAssertPath(
      {
        ...req.getResolverOptionsFromFlags(),
        origin: flags.cwd,
        source: createUnknownFilePath(args[0]),
      },
      {pointer: req.getDiagnosticPointerFromFlags({type: 'arg', key: 0})},
    );

    const ast = await req.requestWorkerParse(filename, {
      compact: commandFlags.compact,
      sourceType: commandFlags.sourceType,
    });

    const hasErrors = ast.diagnostics.length > 0;
    if (!hasErrors || commandFlags.showDespiteDiagnostics) {
      reporter.inspect(ast);
    }

    if (hasErrors) {
      throw new DiagnosticsError('Parsing failed', ast.diagnostics);
    }
  },
});
