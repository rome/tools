/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Path} from '@romejs/js-compiler';
import {template} from '@romejs/js-ast-utils';
import {AnyNode, stringLiteral} from '@romejs/js-ast';
import {getOptions, getModuleId} from '../_utils';

export default {
  name: 'asyncImportTransform',
  enter(path: Path): AnyNode {
    const {node, context} = path;
    const opts = getOptions(context);

    // desugar import('source') to Rome.import(moduleId)
    if (node.type === 'ImportCall' && node.argument.type === 'StringLiteral') {
      const moduleId = getModuleId(node.argument.value, opts);
      if (moduleId !== undefined) {
        const id = stringLiteral.create({
          loc: node.argument.loc,
          value: moduleId,
        });
        return template.expression`Rome.import(${id})`;
      }
    }

    return node;
  },
};
