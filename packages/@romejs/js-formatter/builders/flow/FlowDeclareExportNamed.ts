/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens} from '../../tokens';
import {
  FlowDeclareExportNamed,
  flowDeclareExportNamed,
  AnyNode,
} from '@romejs/js-ast';

export default function FlowDeclareExportNamed(
  builder: Builder,
  node: AnyNode,
): Tokens {
  node = flowDeclareExportNamed.assert(node);

  flowDeclareExportNamed.assert(node);
  throw new Error('unimplemented');
}
