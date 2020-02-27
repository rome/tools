/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ClassProperty, classProperty, AnyNode} from '@romejs/js-ast';

export default function ClassProperty(generator: Generator, node: AnyNode) {
  node = classProperty.assert(node);

  if (node.value === undefined && !generator.options.typeAnnotations) {
    // A ClassProperty with no value is a type annotation
    return;
  }

  const {meta} = node;

  if (meta.static === true) {
    generator.word('static');
    generator.space();
  }

  generator.print(node.key, node);
  generator.printTypeColon(node.typeAnnotation, node);

  if (node.value) {
    generator.space();
    generator.token('=');
    generator.space();
    generator.print(node.value, node);
  }
  generator.semicolon();
}
