/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  JSNodeBase,
  TSSignatureDeclarationMeta,
  AnyObjectPropertyKey,
  AnyTSPrimary,
} from '../index';
import {createBuilder} from '../utils';

export type TSMethodSignature = 
  & JSNodeBase
  & {
    key: AnyObjectPropertyKey;
    type: 'TSMethodSignature';
    optional?: boolean;
    meta: TSSignatureDeclarationMeta;
    typeAnnotation?: AnyTSPrimary;
  };

export const tsMethodSignature = createBuilder<TSMethodSignature>(
  'TSMethodSignature',
  {
    bindingKeys: {},
    visitorKeys: {
      key: true,
      meta: true,
      typeAnnotation: true,
    },
  },
);
