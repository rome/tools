/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Program} from '@romejs/js-ast';
import {
  JSParserUserOptions,
  normalizeOptions,
  JSParserOptions,
} from './options';
import {Token} from './tokenizer/index';
import {types as tokTypes} from './tokenizer/types';
import createParser from './parser';
import './tokenizer/context';

export function parseJS(userOptions: JSParserUserOptions): Program {
  const options: JSParserOptions = normalizeOptions(userOptions);
  return createParser(options).parse();
}

export function tokenizeJS(
  input: string,
  userOptions: JSParserUserOptions,
): Array<Token> {
  const options: JSParserOptions = normalizeOptions(userOptions);
  const parser = createParser({...options, tokens: true, input});
  parser.parse();

  const diagnostics = parser.getDiagnostics();
  let tokens: Array<Token> = parser.state.tokens;

  // If we have any diagnostics, then mark anything from the first as invalid
  if (diagnostics.length > 0) {
    const firstDiag = diagnostics[0];
    const invalidStart = firstDiag.location.start;
    const invalidEnd = firstDiag.location.end;
    if (invalidStart === undefined || invalidEnd === undefined) {
      throw new Error('All parser diagnostics are expected to have a start/end');
    }

    const invalidStartIndex = invalidStart.index;

    const invalidToken: Token = {
      type: tokTypes.invalid,
      start: invalidStart.index,
      end: invalidEnd.index,
      loc: {
        filename: parser.filename,
        start: invalidStart,
        end: invalidEnd,
      },
    };

    // Remove all tokens after our invalid one
    tokens = tokens.filter((token) => {
      return token.loc.start.index >= invalidStartIndex;
    });

    tokens.push(invalidToken);
  }

  return tokens;
}

export {Token};

export {tokTypes};

export {keywords as keywordTokTypes} from './tokenizer/types';
export * from './xhtmlEntities';
