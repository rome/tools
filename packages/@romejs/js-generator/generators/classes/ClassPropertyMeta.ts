/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {ClassPropertyMeta, classPropertyMeta, AnyNode} from '@romejs/js-ast';

export default function ClassPropertyMeta(generator: Generator, node: AnyNode) {
  node = classPropertyMeta.assert(node);

  if (!generator.options.typeAnnotations) {
    if (node.accessibility) {
      generator.word(node.accessibility);
    }

    if (node.readonly) {
      generator.word('readonly');
    }

    if (node.abstract) {
      generator.word('abstract');
    }
  }

  if (node.static) {
    generator.word('static');
  }
}
