/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyTSPrimary, JSNodeBase, TSSignatureDeclarationMeta} from '../index';
import {createBuilder} from '../utils';

export type TSCallSignatureDeclaration = JSNodeBase & {
  type: 'TSCallSignatureDeclaration';
  meta: TSSignatureDeclarationMeta;
  typeAnnotation?: AnyTSPrimary;
};

export const tsCallSignatureDeclaration = createBuilder<TSCallSignatureDeclaration>(
  'TSCallSignatureDeclaration',
  {
    bindingKeys: {},
    visitorKeys: {
      meta: true,
      typeAnnotation: true,
    },
  },
);
