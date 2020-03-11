/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {commandCategories} from '../../commands';
import {createMasterCommand} from '../../commands';

export default createMasterCommand({
  category: commandCategories.PROJECT_MANAGEMENT,
  description: 'TODO',

  async default(req: MasterRequest): Promise<void> {
    req.expectArgumentLength(1);

    // TODO
  },
});
