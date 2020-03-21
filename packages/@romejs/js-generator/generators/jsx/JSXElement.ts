/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {JSXElement, jsxElement, AnyNode} from '@romejs/js-ast';

export default function JSXElement(generator: Generator, node: AnyNode) {
  node = jsxElement.assert(node);
  jsxElement.assert(node);
  generator.token('<');
  generator.print(node.name, node);
  generator.print(node.typeArguments, node);

  if (node.attributes.length > 0) {
    generator.space();
    generator.printJoin(node.attributes, node, {
      after: spaceSeparator,
    });
  }

  if (node.selfClosing === true) {
    generator.space();
    generator.token('/>');
    return;
  } else {
    generator.token('>');
  }

  generator.indent();
  for (const child of node.children) {
    generator.print(child, node);
  }
  generator.dedent();

  generator.token('</');
  generator.print(node.name, node);
  generator.token('>');
}

function spaceSeparator(generator: Generator, isLast: boolean) {
  if (isLast) {
    return;
  }

  generator.space();
}
