/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Builder from '../../Builder';
import {Tokens, space, indent, operator, concat} from '../../tokens';
import {jsxElement, AnyNode} from '@romejs/js-ast';

export default function JSXElement(builder: Builder, node: AnyNode): Tokens {
  node = jsxElement.assert(node);

  const tokens: Tokens = [
    operator('<'),
    concat(builder.tokenize(node.name, node)),
    concat(builder.tokenize(node.typeArguments, node)),
  ];

  if (node.attributes.length > 0) {
    tokens.push(space, builder.tokenizeJoin(node.attributes, node, {
      newline: true,
      broken: {},
      unbroken: {
        separator: [space],
      },
    }));
  }

  if (node.selfClosing === true && node.children.length === 0) {
    return [concat(tokens), space, operator('/>')];
  } else {
    return [
        concat(tokens),
        operator('>'),
        indent(node.children.map(
          (child) => concat(builder.tokenize(child, node)),
        )),
        operator('</'),
        concat(builder.tokenize(node.name, node)),
        operator('>'),
      ];
  }
}
