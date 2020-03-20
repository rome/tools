/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  JSXIdentifier,
  JSXNamespacedName,
  JSXReferenceIdentifier,
} from '../index';
import {createBuilder} from '../utils';

export type JSXMemberExpression =
  & JSNodeBase
  & {
    type: 'JSXMemberExpression';
    object:
        | JSXMemberExpression
        | JSXIdentifier
        | JSXReferenceIdentifier
        | JSXNamespacedName;
    property: JSXIdentifier;
  };

export const jsxMemberExpression = createBuilder<JSXMemberExpression>(
  'JSXMemberExpression',
  {
    bindingKeys: {},
    visitorKeys: {
      object: true,
      property: true,
    },
  },
);
