/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {
  FlowObjectTypeAnnotation,
  flowObjectTypeAnnotation,
  AnyNode,
} from '@romejs/js-ast';
import {isMultiLine} from '../../node/index';

export default function FlowObjectTypeAnnotation(
  generator: Generator,
  node: AnyNode,
) {
  node = flowObjectTypeAnnotation.assert(node);

  if (node.exact === true) {
    generator.token('{|');
  } else {
    generator.token('{');
  }

  const props = node.properties;
  if (props.length) {
    generator.space();

    generator.printJoin(props, node, {
      indent: true,
      iterator: () => {
        if (props.length !== 1) {
          generator.token(',');
          generator.space();
        }
      },
      statement: isMultiLine(node),
    });

    generator.space();
  }

  if (node.exact === true) {
    generator.token('|}');
  } else {
    generator.token('}');
  }
}
