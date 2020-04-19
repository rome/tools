/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {parseJS} from '@romejs/js-parser';
import {createFixtureTests} from '@romejs/test-helpers';
import {ConstProgramSyntax} from '@romejs/js-ast';
import {removeCarriageReturn} from '@romejs/string-utils';
import { createAbsoluteFilePath } from '@romejs/path';

const promise = createFixtureTests(
  (fixture, t) => {
    const {options, files} = fixture;

    // Get the input JS
    const inputFile = files.get('input.js') || files.get('input.mjs') ||
      files.get('input.ts') || files.get('input.tsx');
    if (inputFile === undefined) {
      throw new Error(
        `The fixture ${fixture.dir} did not have an input.(mjs|js|ts|tsx)`,
      );
    }

    const sourceTypeProp = options.get('sourceType');
    const sourceType = sourceTypeProp.asString('script');
    if (sourceType !== 'module' && sourceType !== 'script') {
      throw sourceTypeProp.unexpected('Expected either script or module');
    }

    const allowReturnOutsideFunction = options.get('allowReturnOutsideFunction').asBoolean(
      false,
    );
    const filename = inputFile.relative;

    const syntax: Array<ConstProgramSyntax> = options.get('syntax').asArray(true).map(
      (
        item,
      ) => {
        return item.asStringSet(['jsx', 'ts', 'flow']);
      },
    );

    t.addToAdvice({
      type: 'log',
      category: 'info',
      message: 'Parser options',
    });

    t.addToAdvice({
      type: 'inspect',
      data: {
        filename: filename.join(),
        allowReturnOutsideFunction,
        sourceType,
        syntax,
      },
    });

    const inputContent = removeCarriageReturn(inputFile.content.toString());

    const ast = parseJS({
      input: inputContent,
      path: filename,
      allowReturnOutsideFunction,
      sourceType,
      syntax,
    });

    const outputFile = createAbsoluteFilePath(inputFile.absolute.getParent().append(
          `${inputFile.absolute.getExtensionlessBasename()}.test.md`,
        ).join());
    t.snapshot(ast, undefined, outputFile.join());
  },
);

// @ts-ignore Doesn't support top-level await lol
await promise;
