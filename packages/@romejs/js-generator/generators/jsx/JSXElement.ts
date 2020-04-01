/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {Tokens, space, indent, flatten, operator} from '../../tokens';
import {JSXElement, jsxElement, AnyNode} from '@romejs/js-ast';

export default function JSXElement(generator: Generator, node: AnyNode): Tokens {
  node = jsxElement.assert(node);

  let tokens: Tokens = [
    operator('<'),
    ...generator.print(node.name, node),
    ...generator.print(node.typeArguments, node),
  ];

  if (node.attributes.length > 0) {
    tokens = [
      ...tokens,
      space,
      generator.printJoin(node.attributes, node, {
        newline: true,
        broken: {},
        unbroken: {
          separator: [space],
        },
      }),
    ];
  }

  if (node.selfClosing === true) {
    return [...tokens, space, operator('/>')];
  } else {
    return [
        ...tokens,
        operator('>'),
        indent(flatten(
          node.children.map((child) => generator.print(child, node)),
        )),

        operator('</'),
        ...generator.print(node.name, node),
        operator('>'),
      ];
  }
}
