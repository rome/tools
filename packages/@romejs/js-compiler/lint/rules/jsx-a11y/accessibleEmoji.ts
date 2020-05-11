/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import { Path } from '@romejs/js-compiler';
import { AnyNode, JSXAttribute, JSXSpreadAttribute } from '@romejs/js-ast';
import { descriptions } from '@romejs/diagnostics';
function containsImageRole(
  attribute: JSXAttribute | JSXSpreadAttribute
): boolean {
  return (
    attribute.type === 'JSXAttribute' &&
    attribute.name.type === 'JSXIdentifier' &&
    attribute.name.name === 'role' &&
    !!attribute.value &&
    attribute.value.type === 'StringLiteral' &&
    attribute.value.value === 'img'
  );
}

function containsAriaLabel(
  attribute: JSXAttribute | JSXSpreadAttribute
): boolean {
  return (
    attribute.type === 'JSXAttribute' &&
    attribute.name.type === 'JSXIdentifier' &&
    (attribute.name.name === 'aria-label' ||
      attribute.name.name === 'aria-labelledby') &&
    !!attribute.value &&
    attribute.value.type === 'StringLiteral' &&
    attribute.value.value.length > 0
  );
}

function containsEmoji(value: string): boolean {
  return (/(?:[\u2700-\u27bf]|(?:\ud83c[\udde6-\uddff]){2}|[\ud800-\udbff][\udc00-\udfff]|[\u0023-\u0039]\ufe0f?\u20e3|\u3299|\u3297|\u303d|\u3030|\u24c2|\ud83c[\udd70-\udd71]|\ud83c[\udd7e-\udd7f]|\ud83c\udd8e|\ud83c[\udd91-\udd9a]|\ud83c[\udde6-\uddff]|\ud83c[\ude01-\ude02]|\ud83c\ude1a|\ud83c\ude2f|\ud83c[\ude32-\ude3a]|\ud83c[\ude50-\ude51]|\u203c|\u2049|[\u25aa-\u25ab]|\u25b6|\u25c0|[\u25fb-\u25fe]|\u00a9|\u00ae|\u2122|\u2139|\ud83c\udc04|[\u2600-\u26FF]|\u2b05|\u2b06|\u2b07|\u2b1b|\u2b1c|\u2b50|\u2b55|\u231a|\u231b|\u2328|\u23cf|[\u23e9-\u23f3]|[\u23f8-\u23fa]|\ud83c\udccf|\u2934|\u2935|[\u2190-\u21ff])/g).test(value)
}

export default {
  name: 'accessibleEmoji',
  enter(path: Path): AnyNode {
    const { node } = path;

    if (node.type === 'JSXElement' && node.children.length === 1) {
      if (
        (node.children[0].type === 'JSXText' &&
          containsEmoji(node.children[0].value) &&
          node.name.type === 'JSXIdentifier' &&
          (!(node.name.name === 'span') ||
            !node.attributes.find(containsImageRole) ||
            !node.attributes.find(containsAriaLabel)))
      ) {
        path.context.addNodeDiagnostic(
          node,
          descriptions.LINT.ACCESSIBLE_EMOJI
        );
      }
    }

    return node;
  },
};
