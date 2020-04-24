/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Identifier, JSNodeBase} from '../index';
import {createBuilder} from '../utils';

export type MetaProperty = JSNodeBase & {
  type: 'MetaProperty';
  meta: Identifier;
  property: Identifier;
};

export const metaProperty = createBuilder<MetaProperty>('MetaProperty', {
  bindingKeys: {},
  visitorKeys: {
    meta: true,
    property: true,
  },
});
