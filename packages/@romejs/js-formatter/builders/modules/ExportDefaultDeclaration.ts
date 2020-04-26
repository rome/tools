/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ExportDefaultDeclaration} from '@romejs/js-ast';
import Builder from '../../Builder';
import {Token, concat, space} from '../../tokens';
import {printExportDeclaration} from './ExportLocalDeclaration';

export default function ExportDefaultDeclaration(
  builder: Builder,
  node: ExportDefaultDeclaration,
): Token {
  return concat([
    'export',
    space,
    'default',
    space,
    printExportDeclaration(builder, node),
  ]);
}
