/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {createMasterCommand} from '../../commands';
import {commandCategories} from '../../commands';

export default createMasterCommand({
  category: commandCategories.INTERNAL,
  description: 'TODO',

  async default(req: MasterRequest): Promise<void> {
    req;
  },
});
