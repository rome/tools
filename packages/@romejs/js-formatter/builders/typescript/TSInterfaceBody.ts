/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, TSInterfaceBody, tsInterfaceBody} from '@romejs/js-ast';
import {Builder} from '@romejs/js-formatter';
import {Tokens} from '../../tokens';
import {printTSBraced} from '../utils';

export default function TSInterfaceBody(builder: Builder, node: AnyNode): Tokens {
  node = tsInterfaceBody.assert(node);
  return printTSBraced(builder, node, node.body);
}
