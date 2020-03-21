/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Generator from '../../Generator';
import {FlowOpaqueType, flowOpaqueType, AnyNode} from '@romejs/js-ast';

export default function FlowOpaqueType(generator: Generator, node: AnyNode) {
  node = node.type === 'FlowDeclareOpaqueType' ? node : flowOpaqueType.assert(
    node,
  );

  generator.word('opaque');
  generator.space();
  generator.word('type');
  generator.space();
  generator.print(node.id, node);
  generator.print(node.typeParameters, node);
  if (node.supertype) {
    generator.token(':');
    generator.space();
    generator.print(node.supertype, node);
  }
  if (node.impltype) {
    generator.space();
    generator.token('=');
    generator.space();
    generator.print(node.impltype, node);
  }
  generator.semicolon();
}
