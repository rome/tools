/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {commandCategories} from '../../commands';
import {createMasterCommand} from '../../commands';

export default createMasterCommand({
  category: commandCategories.CODE_QUALITY,
  description: 'install dependencies, run lint and tests',

  async default(): Promise<void> {},
});
