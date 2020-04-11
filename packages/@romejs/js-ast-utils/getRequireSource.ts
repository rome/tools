/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import doesNodeMatchPattern from './doesNodeMatchPattern';

const ROME_DEFAULT_REQUIRE = ['Rome', 'requireDefault'];
const ROME_NAMESPACE_REQUIRE = ['Rome', 'requireNamespace'];

export default function getRequireSource(
  node: undefined | AnyNode,
  scope: Scope,
  allowStaticMember: boolean = false,
): undefined | string {
  if (node === undefined) {
    return undefined;
  }

  if (allowStaticMember && node.type === 'MemberExpression' &&
        node.property.type ===
        'StaticMemberProperty') {
    node = node.object;
  }

  if (node.type !== 'CallExpression') {
    return undefined;
  }

  const {arguments: args, callee} = node;

  const [firstArg] = args;
  if (args.length !== 1 || firstArg.type !== 'StringLiteral') {
    return undefined;
  }

  const validRequireCallee = callee.type === 'ReferenceIdentifier' &&
      callee.name ===
      'require' && scope.getBinding('require') === undefined;

  const validRomeRequreCallee = (doesNodeMatchPattern(
      callee,
      ROME_DEFAULT_REQUIRE,
    ) || doesNodeMatchPattern(callee, ROME_NAMESPACE_REQUIRE)) &&
      scope.getBinding('Rome') ===
      undefined;

  if (validRequireCallee || validRomeRequreCallee) {
    return firstArg.value;
  }

  return undefined;
}
