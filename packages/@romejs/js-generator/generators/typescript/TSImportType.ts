/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TSImportType, tsImportType, AnyNode} from '@romejs/js-ast';
import {Generator} from '@romejs/js-generator';

export default function TSImportType(generator: Generator, node: AnyNode) {
  node = tsImportType.assert(node);

  generator.word('import');
  generator.token('(');
  generator.print(node.argument, node);
  generator.token(')');

  if (node.qualifier) {
    generator.token('.');
    generator.print(node.qualifier, node);
  }

  if (node.typeParameters) {
    generator.print(node.typeParameters, node);
  }
}
