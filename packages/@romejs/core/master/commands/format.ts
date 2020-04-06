/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {createMasterCommand, commandCategories} from '../../commands';

import {createUnknownFilePath} from '@romejs/path';

export default createMasterCommand({
  category: commandCategories.INTERNAL,
  description: 'TODO',

  async default(req: MasterRequest): Promise<undefined | string> {
    const {reporter, master} = req;
    const {args} = req.query;
    req.expectArgumentLength(1);

    const filename = await master.resolver.resolveEntryAssertPath({
      ...req.getResolverOptionsFromFlags(),
      source: createUnknownFilePath(args[0]),
    }, {location: req.getDiagnosticPointerFromFlags({type: 'arg', key: 0})});

    const res = await req.requestWorkerFormat(filename);
    if (res === undefined) {
      reporter.error('No formatter for this file');
      return undefined;
    } else {
      reporter.writeAll(res.formatted);
      return res.formatted;
    }
  },
});
