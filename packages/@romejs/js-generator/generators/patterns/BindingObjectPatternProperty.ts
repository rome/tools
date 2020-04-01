/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens} from '../../tokens';
import {bindingObjectPatternProperty, AnyNode} from '@romejs/js-ast';
import ObjectProperty from '../objects/ObjectProperty';

export default function BindingObjectPatternProperty(
  generator: Generator,
  node: AnyNode,
): Tokens {
  node = bindingObjectPatternProperty.assert(node);
  return ObjectProperty(generator, node);
}
