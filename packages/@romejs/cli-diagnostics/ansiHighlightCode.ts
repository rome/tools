/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {formatAnsi} from '@romejs/string-ansi';
import {tokenizeJS} from '@romejs/js-parser';
import {get0, Number0} from '@romejs/ob1';
import {DiagnosticLanguage, DiagnosticSourceType} from '@romejs/diagnostics';
import {ConstSourceType} from '@romejs/js-ast';
import {tokenizeJSON} from '@romejs/codec-json';
import {UnknownFilePath, createUnknownFilePath} from '@romejs/path';

// 100KB
const FILE_SIZE_MAX = 100_000;

export type AnsiHighlightOptions = {
  path: UnknownFilePath;
  input: string;
  sourceType: undefined | DiagnosticSourceType;
  language: undefined | DiagnosticLanguage;
};

export default function ansiHighlightCode(opts: AnsiHighlightOptions): string {
  if (opts.input.length > FILE_SIZE_MAX) {
    return opts.input;
  }

  if (opts.language === 'js') {
    // js-parser does not accept an "unknown" sourceType
    return ansiHighlightJS(opts.input, opts.sourceType === undefined ||
    opts.sourceType === 'unknown' ? 'script' : opts.sourceType);
  }

  if (opts.language === 'json') {
    return ansiHighlightJSON(opts.path, opts.input);
  }

  return opts.input;
}

function reduce<Token extends {
  start: Number0;
  end: Number0;
}>(
  input: string,
  tokens: Array<Token>,
  callback: (token: Token, line: string) => string,
): string {
  let prevEnd = 0;
  let buff = '';

  for (const token of tokens) {
    const start = get0(token.start);
    const end = get0(token.end);
    let value = input.slice(start, end);

    // Add on text between tokens
    buff += input.slice(prevEnd, start);
    prevEnd = end;

    // We need to break up the token text into lines, so that we can easily split the highlighted newlines and have the ansi codes be unbroken
    const lines = value.split('\n');

    const values: Array<string> = lines.map((line) => {
      return callback(token, line);
    });

    buff += values.join('\n');
  }

  return buff;
}

function invalidHighlight(line: string): string {
  return formatAnsi.bold(formatAnsi.bgRed(line));
}

function ansiHighlightJSON(path: UnknownFilePath, input: string): string {
  const tokens = tokenizeJSON({
    input,
    // Wont be used anywhere but activates JSON extensions if necessary
    path,
  });

  return reduce(input, tokens, (token, line) => {
    // Try to keep the highlighting in line with JS where possible
    switch (token.type) {
      case 'BlockComment':
      case 'LineComment':
        return formatAnsi.brightBlack(line);

      case 'String':
        return formatAnsi.green(line);

      case 'Number':
        return formatAnsi.magenta(line);

      case 'Word':
        switch (token.value) {
          case 'true':
          case 'false':
          case 'null':
            return formatAnsi.cyan(line);

          default:
            return line;
        }

      case 'Comma':
      case 'Colon':
      case 'Dot':
        return formatAnsi.yellow(line);

      case 'BracketOpen':
      case 'BracketClose':
      case 'BraceOpen':
      case 'BraceClose':
      case 'Minus':
      case 'Plus':
        return line;

      case 'Invalid':
        return invalidHighlight(line);

      // Will never be hit
      case 'EOF':
      case 'SOF':
        return '';
    }
  });
}

function ansiHighlightJS(input: string, sourceType: ConstSourceType): string {
  const tokens = tokenizeJS(input, {
    sourceType,
    // js-parser requires a filename. Doesn't really matter since we'll never be producing an AST or diagnostics
    path: createUnknownFilePath('unknown'),
  });

  return reduce(input, tokens, (token, line) => {
    const {type} = token;

    switch (type.label) {
      case 'break':
      case 'case':
      case 'catch':
      case 'continue':
      case 'debugger':
      case 'default':
      case 'do':
      case 'else':
      case 'finally':
      case 'for':
      case 'function':
      case 'if':
      case 'return':
      case 'switch':
      case 'throw':
      case 'try':
      case 'var':
      case 'const':
      case 'while':
      case 'with':
      case 'new':
      case 'this':
      case 'super':
      case 'class':
      case 'extends':
      case 'export':
      case 'import':
      case 'null':
      case 'true':
      case 'false':
      case 'in':
      case 'instanceof':
      case 'typeof':
      case 'void':
      case 'delete':
        return formatAnsi.cyan(line);

      case 'num':
      case 'bigint':
        return formatAnsi.magenta(line);

      case 'regexp':
        return formatAnsi.magenta(line);

      case 'string':
      case 'template':
      case '`':
        return formatAnsi.green(line);

      case 'invalid':
        return invalidHighlight(line);

      case 'comment':
        return formatAnsi.brightBlack(line);

      case ',':
      case ';':
      case ':':
      case '::':
      case '${':
      case '.':
      case '?':
      case '?.':
        return formatAnsi.yellow(line);

      case '[':
      case ']':
      case '{':
      case '{|':
      case '}':
      case '|}':
      case '(':
      case ')':
        return line;

      case '=>':
      case '...':
      case '@':
      case '#':
      case '=':
      case '_=':
      case '++/--':
      case '!':
      case '~':
      case '??':
      case '||':
      case '&&':
      case '|':
      case '^':
      case '&':
      case '==/!=':
      case '</>':
      case '<</>>':
      case '+/-':
      case '%':
      case '*':
      case '/':
      case '**':
      case 'jsxName':
      case 'jsxText':
      case 'jsxTagStart':
      case 'jsxTagEnd':
      case 'name':
      case 'eof':
        return line;
    }
  });
}
