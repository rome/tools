/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {extractSuppressionsFromProgram} from './suppressions';
import {AnyComment, CommentBlock, MOCK_PROGRAM, program} from '@romejs/js-ast';
import {ob1Coerce1, ob1Number0} from '@romejs/ob1';
import CompilerContext from './lib/CompilerContext';

function extractSuppressionsFromComments(comments: Array<AnyComment>) {
  const ast = program.create({
    ...MOCK_PROGRAM,
    comments,
  });
  const context = new CompilerContext({ast});
  return extractSuppressionsFromProgram(context, ast);
}

function generateComment(value: string, line: number): CommentBlock {
  const pos = {
    index: ob1Number0,
    column: ob1Number0,
    line: ob1Coerce1(line),
  };

  return {
    type: 'CommentBlock',
    value,
    id: '0',
    loc: {
      filename: 'unknown',
      start: pos,
      end: pos,
    },
  };
}

test(
  'single category',
  async (t) => {
    t.snapshot(
      extractSuppressionsFromComments([
        generateComment('rome-disable-line foo', 1),
        generateComment('* rome-disable-line foo', 2),
        generateComment(' * rome-disable-line foo', 3),
        generateComment('* wow\n * rome-disable-line foo', 4),
      ]),
    );
  },
);

test(
  'multiple categories',
  async (t) => {
    t.snapshot(
      extractSuppressionsFromComments([
        generateComment('rome-disable-line foo bar', 1),
        generateComment('* rome-disable-line foo bar', 2),
        generateComment(' * rome-disable-line foo bar', 3),
        generateComment(
          '* wow\n * rome-disable-line foo bar\n* rome-disable-line cat dog',
          4,
        ),
      ]),
    );
  },
);

test(
  'typos',
  async (t) => {
    t.snapshot(
      extractSuppressionsFromComments([
        generateComment('rome-ignore foo bar', 1),
      ]),
    );
  },
);

test(
  'duplicates',
  async (t) => {
    t.snapshot(
      extractSuppressionsFromComments([
        generateComment('rome-disable-line foo foo', 1),
      ]),
    );
  },
);
