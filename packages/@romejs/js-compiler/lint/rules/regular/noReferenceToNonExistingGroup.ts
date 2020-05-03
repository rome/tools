/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {AnyNode, RegExpGroupCapture} from '@romejs/js-ast';
import {descriptions} from '@romejs/diagnostics';

function findCaptureGroups(path: Path): Array<RegExpGroupCapture> | undefined {
  const regexLiteral = path.findAncestry((path) =>
    path.node.type === 'RegExpLiteral'
  );
  if (regexLiteral === undefined) {
    return regexLiteral;
  }
  let captureGroups: Array<RegExpGroupCapture> = [];
  regexLiteral.traverse(
    'RegExpLiteral',
    (path) => {
      if (path.node.type === 'RegExpGroupCapture') {
        captureGroups.push(path.node);
      }
    },
  );
  return captureGroups;
}

export default {
  name: 'noReferenceToNonExistingGroup',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    if (node.type === 'RegExpNumericBackReference') {
      const allCaptureGroups = findCaptureGroups(path);
      if (allCaptureGroups === undefined) {
        context.addNodeDiagnostic(
          node,
          descriptions.LINT.NO_REFERENCE_TO_NON_EXISTING_GROUP(
            String(node.value),
          ),
        );
      } else {
        if (node.value > allCaptureGroups.length) {
          context.addNodeDiagnostic(
            node,
            descriptions.LINT.NO_REFERENCE_TO_NON_EXISTING_GROUP(
              String(node.value),
            ),
          );
        }
      }
    }

    return node;
  },
};
