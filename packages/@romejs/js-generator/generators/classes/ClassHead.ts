/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ClassHead, classHead, AnyNode} from '@romejs/js-ast';

export default function ClassHead(generator: Generator, node: AnyNode) {
  node = classHead.assert(node);

  generator.print(node.typeParameters, node);

  if (node.superClass) {
    generator.space();
    generator.word('extends');
    generator.space();
    generator.print(node.superClass, node);
    generator.print(node.superTypeParameters, node);
  }

  if (node.implements !== undefined && node.implements.length > 0 &&
    generator.options.typeAnnotations) {
    generator.space();
    generator.word('implements');
    generator.space();
    generator.printCommaList(node.implements, node);
  }
}
