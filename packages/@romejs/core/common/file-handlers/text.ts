/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UNKNOWN_ANALYZE_DEPENDENCIES_RESULT} from '../types/analyzeDependencies';
import {PartialExtensionHandler} from './types';

export const textHandler: PartialExtensionHandler = {
  sourceType: 'module',

  // Mock a single default export
  // We could always just pass this through to analyzeDependencies and get the same result due to the toJavaScript call below,
  // but the return value is predictable so we inline it
  async analyzeDependencies() {
    return {
      ...UNKNOWN_ANALYZE_DEPENDENCIES_RESULT,
      moduleType: 'es',
      exports: [
        {
          type: 'local',
          // TODO we could fake this?
          loc: undefined,
          kind: 'value',
          valueType: 'other',
          name: 'default',
        },
      ],
    };
  },

  async toJavaScript({file, worker}) {
    const src = await worker.readFile(file.real);
    const serial = JSON.stringify(src);
    return {
      sourceText: `export default ${serial};`,
      generated: true,
    };
  },
};
