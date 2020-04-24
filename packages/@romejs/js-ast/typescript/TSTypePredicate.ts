/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyPrimaryType, Identifier, JSNodeBase, TSThisType} from '../index';
import {createBuilder} from '../utils';

export type TSTypePredicate = JSNodeBase & {
  type: 'TSTypePredicate';
  asserts: boolean;
  parameterName: Identifier | TSThisType;
  typeAnnotation?: AnyPrimaryType;
};

export const tsTypePredicate = createBuilder<TSTypePredicate>(
  'TSTypePredicate',
  {
    bindingKeys: {},
    visitorKeys: {
      parameterName: true,
      typeAnnotation: true,
    },
  },
);
