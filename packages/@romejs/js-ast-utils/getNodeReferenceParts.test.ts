/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import getNodeReferenceParts from './getNodeReferenceParts';
import template from './template';

test(
  'getNodeReferenceParts',
  (t) => {
    t.inlineSnapshot(
      getNodeReferenceParts(template.expression`foo`),
      "Object {\n  bailed: false\n  parts: Array [\n    Object {\n      value: 'foo'\n      node: ReferenceIdentifier {name: 'foo'}\n    }\n  ]\n}",
    );
    t.inlineSnapshot(
      getNodeReferenceParts(template.expression`foo.bar`),
      "Object {\n  bailed: false\n  parts: Array [\n    Object {\n      value: 'foo'\n      node: ReferenceIdentifier {name: 'foo'}\n    }\n    Object {\n      value: 'bar'\n      node: Identifier {name: 'bar'}\n    }\n  ]\n}",
    );
    t.inlineSnapshot(
      getNodeReferenceParts(template.expression`this.bar`),
      "Object {\n  bailed: false\n  parts: Array [\n    Object {\n      value: 'this'\n      node: ThisExpression {}\n    }\n    Object {\n      value: 'bar'\n      node: Identifier {name: 'bar'}\n    }\n  ]\n}",
    );
    t.inlineSnapshot(
      getNodeReferenceParts(template.expression`this.bar[bar]`),
      "Object {\n  bailed: true\n  parts: Array [\n    Object {\n      value: 'this'\n      node: ThisExpression {}\n    }\n    Object {\n      value: 'bar'\n      node: Identifier {name: 'bar'}\n    }\n  ]\n}",
    );
    t.inlineSnapshot(
      getNodeReferenceParts(template.expression`import.meta`),
      "Object {\n  bailed: false\n  parts: Array [\n    Object {\n      value: 'import'\n      node: MetaProperty {\n        meta: Identifier {name: 'import'}\n        property: Identifier {name: 'meta'}\n      }\n    }\n    Object {\n      value: 'meta'\n      node: MetaProperty {\n        meta: Identifier {name: 'import'}\n        property: Identifier {name: 'meta'}\n      }\n    }\n  ]\n}",
    );
    t.inlineSnapshot(
      getNodeReferenceParts(template.expression`foo['bar']`),
      "Object {\n  bailed: false\n  parts: Array [\n    Object {\n      value: 'foo'\n      node: ReferenceIdentifier {name: 'foo'}\n    }\n    Object {\n      value: 'bar'\n      node: StringLiteral {value: 'bar'}\n    }\n  ]\n}",
    );
    t.inlineSnapshot(
      getNodeReferenceParts(template.expression`foo[bar]`),
      "Object {\n  bailed: true\n  parts: Array [\n    Object {\n      value: 'foo'\n      node: ReferenceIdentifier {name: 'foo'}\n    }\n  ]\n}",
    );
  },
);
