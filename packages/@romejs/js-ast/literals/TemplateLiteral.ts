/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyExpression, JSNodeBase, TemplateElement} from '../index';
import {createBuilder} from '../utils';

export type TemplateLiteral = JSNodeBase & {
  type: 'TemplateLiteral';
  quasis: Array<TemplateElement>;
  expressions: Array<AnyExpression>;
};

export const templateLiteral = createBuilder<TemplateLiteral>(
  'TemplateLiteral',
  {
    bindingKeys: {},
    visitorKeys: {
      quasis: true,
      expressions: true,
    },
  },
);
