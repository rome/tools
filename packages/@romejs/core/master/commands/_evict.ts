/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {commandCategories, createMasterCommand} from '../../commands';

export default createMasterCommand({
  description: 'evict a file from the memory cache',
  category: commandCategories.INTERNAL,

  async default(req: MasterRequest): Promise<void> {
    const {
      master,
      reporter,
      client,
      query: {args},
    } = req;

    const files = args.length === 0
      ? master.fileAllocator.getAllOwnedFilenames() : args;

    for (const file of files) {
      await master.fileAllocator.evict(client.flags.cwd.resolve(file));
      reporter.success(`Evicted ${file}`);
    }

    reporter.info(`Evicted ${String(files.length)} files`);
  },
});
