/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from '@romejs/diagnostics';
import {AnyNode} from '@romejs/js-ast';
import {Path} from '@romejs/js-compiler';

function jsxDuplicateProps(node: AnyNode) {
  if (node.type !== 'JSXElement') {
    return false;
  }

  const propNames: {
    [prop: string]: boolean;
  } = {};

  return node.attributes.some((attribute) => {
    if (attribute.type === 'JSXSpreadAttribute') {
      return false;
    }
    if (typeof attribute.name.name !== 'string') {
      return false;
    }
    if (propNames[attribute.name.name]) {
      return true;
    }
    propNames[attribute.name.name] = true;
    return false;
  });
}

export default {
  name: 'jsxNoDuplicateProps',

  enter(path: Path): AnyNode {
    const {node} = path;

    if (jsxDuplicateProps(node)) {
      path.context.addNodeDiagnostic(
        node,
        descriptions.LINT.JSX_NO_DUPLICATE_PROPS,
      );
    }

    return node;
  },
};
