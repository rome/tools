/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode, MOCK_PARENT} from '@romejs/js-ast';
import Builder from './Builder';
import {PrinterOutput, printTokenToString} from './Printer';

export {Builder};

export type FormatterOptions = {
  typeAnnotations: boolean;
  format?: 'pretty' | 'compact';
  indent?: number;
  comments?: Array<AnyComment>;
  sourceMaps?: boolean;
  sourceText?: string;
};

export function formatJS(
  ast: AnyNode,
  opts: FormatterOptions = {
    typeAnnotations: true,
    format: 'pretty',
  },
): PrinterOutput {
  const builder = new Builder(
    {
      format: opts.format,
      sourceMaps: opts.sourceMaps ?? false,
      typeAnnotations: opts.typeAnnotations,
    },
    ast.type === 'Program' ? ast.comments : opts.comments,
  );
  const token = builder.tokenize(ast, MOCK_PARENT);
  const formatted = printTokenToString(
    token,
    {
      indentWidth: 2,
      printWidth: opts.format === 'pretty' ? 80 : Infinity,
      rootIndent: opts.indent ?? 0,
    },
  );

  return formatted;
}
