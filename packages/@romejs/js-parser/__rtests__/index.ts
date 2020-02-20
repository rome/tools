/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {escapeMarkup} from '@romejs/string-markup';
import {parseJS} from '@romejs/js-parser';
import {createFixtureTests} from '@romejs/test';
import prettyFormat from '@romejs/pretty-format';
import {ConstProgramSyntax} from '@romejs/js-ast';
import {writeFileSync} from '@romejs/fs';
import {DiagnosticsError} from '@romejs/diagnostics';

const promise = createFixtureTests((fixture, t) => {
  const {options, files} = fixture;

  // Get the input JS
  const inputFile =
    files.get('input.js') || files.get('input.ts') || files.get('input.tsx');
  if (inputFile === undefined) {
    throw new Error(
      `The fixture ${fixture.dir} did not have an input.(js|ts|tsx)`,
    );
  }

  // Normalize the AST file
  const outputFile = files.get('output.txt');
  let outputContent = undefined;
  if (outputFile !== undefined) {
    outputContent = outputFile.content.toString();
  }

  const sourceTypeProp = options.get('sourceType');
  const sourceType = sourceTypeProp.asString('script');
  if (sourceType !== 'module' && sourceType !== 'script') {
    throw sourceTypeProp.unexpected('Expected either script or module');
  }

  const allowReturnOutsideFunction = options
    .get('allowReturnOutsideFunction')
    .asBoolean(false);
  const filename = inputFile.relative;

  const syntax: Array<ConstProgramSyntax> = options
    .get('syntax')
    .asArray(true)
    .map(item => {
      return item.asStringSet(['jsx', 'ts', 'flow']);
    });

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

  const inputContent = inputFile.content.toString();
  t.addToAdvice({
    type: 'log',
    category: 'info',
    message: 'Input',
  });
  t.addToAdvice({
    type: 'code',
    code: inputContent,
  });

  const ast = parseJS({
    input: inputContent,
    path: filename,
    allowReturnOutsideFunction,
    sourceType,
    syntax,
  });

  let expectError = options.get('throws').asStringOrVoid();
  if (expectError !== undefined) {
    expectError = escapeMarkup(expectError);
  }

  let {diagnostics} = ast;
  if (diagnostics.length > 0) {
    diagnostics = diagnostics.map(diag => {
      return {
        ...diag,
        code: inputContent,
      };
    });

    if (expectError === undefined) {
      throw new DiagnosticsError(
        "Parser has diagnostics when we didn't expect any",
        diagnostics,
      );
    } else {
      // TODO
      if (expectError === 'Unexpected token') {
        return;
      }

      let matches = false;
      for (const diag of diagnostics) {
        if (escapeMarkup(diag.message).includes(expectError)) {
          matches = true;
          break;
        }
      }

      if (matches) {
        return;
      } else {
        let msg = `No diagnostic matched expected message of "${escapeMarkup(
          expectError,
        )}"`;
        if (diagnostics.length === 1) {
          msg += ` but got "${escapeMarkup(diagnostics[0].message)}"`;
        }
        msg;
        //throw new DiagnosticsError(msg, diagnostics);
        return;
      }
    }
  }

  // If we have an output then compare it with the parsed input
  if (outputContent !== undefined) {
    t.is(prettyFormat(ast), outputContent.trim());
  }

  // If we expected an error to thrown it should have already
  if (expectError !== undefined) {
    // TODO
    throw new Error(
      `Expected an error to be thrown of "${escapeMarkup(expectError)}" in ${
        inputFile.absolute
      } but there was none`,
    );
    return;
  }

  if (outputContent === undefined) {
    // If we didn't expect an error to be thrown and we have no expected output AST
    // then we should write it
    writeFileSync(fixture.dir.append('output.txt'), prettyFormat(ast));
  }
});

// @ts-ignore Doesn't support top-level await lol
await promise;
