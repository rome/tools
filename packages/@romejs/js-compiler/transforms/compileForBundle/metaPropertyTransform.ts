/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  stringLiteral,
  MetaProperty,
  AnyExpression,
} from '@romejs/js-ast';
import {template} from '@romejs/js-ast-utils';
import {Path, Context} from '@romejs/js-compiler';

function isImportMeta(node: AnyNode): node is MetaProperty {
  return (
    node.type === 'MetaProperty' &&
    node.meta.name === 'import' &&
    node.property.name === 'meta'
  );
}

function createURLString(context: Context): AnyExpression {
  const str = stringLiteral.create({
    value: `file://${getFilename(context)}`,
  });
  return template.expression`typeof __filename === 'string' ? 'file://' + __filename : ${str}`;
}

function getFilename(context: Context): string {
  const {path} = context;
  if (path === undefined) {
    return '';
  } else {
    return path.join();
  }
}

export default {
  name: 'metaPropertyTransform',
  enter(path: Path): AnyNode {
    const {node, context} = path;

    // Inline __filenamd and __dirname
    /*if (
      node.type === 'ReferenceIdentifier' &&
      (node.type === '__dirname' || node.name === '__filename')
    ) {
      if (node.type === '__dirname') {
        return stringLiteral.create({
          value: pathUtils.dirname(getFilename(context)),
        });
      }

      if (node.type === '__filename') {
        return stringLiteral.create({
          value: getFilename(context),
        });
      }
    }*/

    // Direct reference to import.meta.url
    if (
      node.type === 'MemberExpression' &&
      node.property.type === 'StaticMemberProperty' &&
      isImportMeta(node.object) &&
      node.property.value.type === 'Identifier' &&
      node.property.value.name === 'url'
    ) {
      return createURLString(context);
    }

    // This is an escaped import.meta or else our other transform would have changed it
    if (isImportMeta(node)) {
      return template.expression`({url: ${createURLString(context)}})`;
    }

    return node;
  },
};
