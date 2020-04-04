/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {
  Token,
  concat,
  group,
  hardline,
  ifBreak,
  indent,
  join,
  lineOrSpace,
  space,
} from '../../tokens';
import {AnyNode, UnionTypeAnnotation} from '@romejs/js-ast';

export default function UnionTypeAnnotation(
  builder: Builder,
  node: UnionTypeAnnotation,
  parent: AnyNode,
): Token {
  // Indentation may be handled by the parent node
  const shouldIndent =
    parent.type !== 'TSTypeAssertion' &&
    parent.type !== 'TSTypeParameterDeclaration' &&
    parent.type !== 'TSTypeParameterInstantiation';

  const printed = concat([
    ifBreak(concat([shouldIndent ? hardline : '', '|', space])),
    join(
      concat([lineOrSpace, '|', space]),
      node.types.map((type) => indent(builder.tokenize(type, node))),
    ),
  ]);

  return group(shouldIndent ? indent(printed) : printed);
}
