/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  ComputedMemberProperty,
  StaticMemberProperty,
  computedMemberProperty,
  identifier,
  staticMemberProperty,
  stringLiteral,
} from '@romejs/js-ast';
import isValidIdentifierName from './isValidIdentifierName';

export default function createMemberProperty(
  name: string,
): StaticMemberProperty | ComputedMemberProperty {
  if (isValidIdentifierName(name)) {
    return staticMemberProperty.quick(identifier.quick(name));
  } else {
    return computedMemberProperty.quick(stringLiteral.quick(name));
  }
}
