/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type FlowVarianceKind = 'plus' | 'minus';

export type FlowVariance =
  & JSNodeBase
  & {
    type: 'FlowVariance';
    kind: FlowVarianceKind;
  };

export const flowVariance = createBuilder<FlowVariance>('FlowVariance', {
  bindingKeys: {},
  visitorKeys: {},
});
