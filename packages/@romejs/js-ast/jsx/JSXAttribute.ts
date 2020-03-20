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
  JSXElement,
  JSXFragment,
  StringLiteral,
  JSXExpressionContainer,
} from '../index';
import {createBuilder} from '../utils';

export type JSXAttribute =
  & JSNodeBase
  & {
    type: 'JSXAttribute';
    name: JSXIdentifier | JSXNamespacedName;
    value?:
        | undefined
        | JSXElement
        | JSXFragment
        | StringLiteral
        | JSXExpressionContainer;
  };

export const jsxAttribute = createBuilder<JSXAttribute>('JSXAttribute', {
  bindingKeys: {},
  visitorKeys: {
    name: true,
    value: true,
  },
});
