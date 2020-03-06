/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  createParser,
  ParserOptions,
  BaseTokens,
  NodeBase,
  ValueToken,
  isEscaped,
} from '@romejs/parser-core';
import {add, get0, Number0, inc} from '@romejs/ob1';

type Tokens = BaseTokens & {
  Hashes: ValueToken<'Hashes', number>;
  CodeBlock: ValueToken<
    'CodeBlock',
    {text: string; language: undefined | string}
  >;
  TextLine: ValueToken<'TextLine', string>;
};

type HeadingNode = NodeBase & {
  type: 'Heading';
  text: string;
  level: number;
};

type CodeBlockNode = NodeBase & {
  type: 'CodeBlock';
  language: undefined | string;
  text: string;
};

type Node = HeadingNode | CodeBlockNode;

function isHash(char: string): boolean {
  return char === '#';
}

function isCodeBlockEnd(index: Number0, input: string): boolean {
  return (
    input[get0(index)] === '`' &&
    !isEscaped(index, input) &&
    input[get0(add(index, 1))] === '`' &&
    input[get0(add(index, 2))] === '`'
  );
}

function isInCodeBlock(char: string, index: Number0, input: string): boolean {
  return !isCodeBlockEnd(index, input);
}

function isntNewline(char: string): boolean {
  return char !== '\n';
}

function unescapeTicks(code: string): string {
  return code;
}

export default createParser(
  ParserCore =>
    class SnapshotParser extends ParserCore<Tokens, void> {
      constructor(opts: ParserOptions) {
        super(opts, 'snapshots');
      }

      tokenize(index: Number0, input: string) {
        const char = input[get0(index)];

        switch (char) {
          case '#':
            const [hashes] = this.readInputFrom(index, isHash);
            const level = hashes.length;
            return this.finishValueToken('Hashes', level, add(index, level));

          case ' ':
          case '\t':
          case '\r':
          case '\n':
            return this.lookaheadToken(inc(index));

          case '`':
            const nextChar = input[get0(add(index, 1))];
            const nextNextChar = input[get0(add(index, 2))];

            if (nextChar === '`' && nextNextChar === '`') {
              let codeOffset = add(index, 3);

              let language: undefined | string;
              if (input[get0(codeOffset)] !== '\n') {
                [language, codeOffset] = this.readInputFrom(
                  codeOffset,
                  isntNewline,
                );
              }

              // Expect the first offset character to be a newline
              if (input[get0(codeOffset)] === '\n') {
                // Skip leading newline
                codeOffset = add(codeOffset, 1);
              } else {
                throw this.unexpected({
                  message: 'Newline required after code block',
                  start: this.getPositionFromIndex(codeOffset),
                });
              }

              let [code] = this.readInputFrom(codeOffset, isInCodeBlock);

              let end = add(codeOffset, code.length);

              if (isCodeBlockEnd(end, input)) {
                // Check for trailing newline
                if (code[code.length - 1] === '\n') {
                  // Trim trailing newline
                  code = code.slice(-1);

                  // Skip closing ticks
                  end = add(end, 3);

                  return this.finishValueToken(
                    'CodeBlock',
                    {
                      language,
                      text: unescapeTicks(code),
                    },
                    end,
                  );
                } else {
                  throw this.unexpected({
                    message: 'Newline required before code block end',
                    start: this.getPositionFromIndex(end),
                  });
                }
              } else {
                throw this.unexpected({
                  message: 'Unclosed code block',
                  start: this.getPositionFromIndex(end),
                });
              }
            }
        }

        const [text] = this.readInputFrom(index, isntNewline);
        return this.finishValueToken('TextLine', text, add(index, text.length));
      }

      parse(): Array<Node> {
        const nodes: Array<Node> = [];

        while (!this.isEOF()) {
          const start = this.getPosition();
          const token = this.getToken();

          switch (token.type) {
            case 'Hashes':
              const level = token.value;
              this.nextToken();
              const text = this.expectToken('TextLine').value;
              nodes.push({
                type: 'Heading',
                level,
                text,
                loc: this.finishLoc(start),
              });
              break;

            case 'CodeBlock':
              nodes.push({
                type: 'CodeBlock',
                ...token.value,
                loc: this.finishLoc(start),
              });
              this.nextToken();
              break;

            default:
              throw this.unexpected();
          }
        }

        return nodes;
      }
    },
);
