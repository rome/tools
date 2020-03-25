/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  BaseTokens,
  createParser,
  ParserOptions,
  ParserUnexpectedOptions,
  ValueToken,
  TokenValues,
  isDigit,
  isESIdentifierStart,
  isESIdentifierChar,
  ComplexToken,
  Position,
} from '@romejs/parser-core';
import {
  AnyRegExpBodyItem,
  RegExpGroupCapture,
  RegExpCharSet,
  RegExpCharSetRange,
  RegExpQuantified,
  RegExpGroupNonCapture,
  AnyRegExpEscapedCharacter,
  RegExpSubExpression,
  RegExpAlternation,
  AnyRegExpExpression,
} from '@romejs/js-ast';
import {Diagnostics, descriptions} from '@romejs/diagnostics';
import {Number0, get0, add} from '@romejs/ob1';

type Operator =
  | '^'
  | '$'
  | '.'
  | '['
  | ']'
  | '('
  | ')'
  | '?'
  | '{'
  | '}'
  | '+'
  | '*'
  | '|';

type Tokens =
  & BaseTokens
  & {
    Operator: ValueToken<'Operator', Operator>;
    Character: ComplexToken<'Character', {
      value: string;
      escaped: boolean;
    }>;
    EscapedCharacter: ValueToken<'EscapedCharacter',
      | 'd'
      | 'D'
      | 'b'
      | 'B'
      | 's'
      | 'S'
      | 'w'
      | 'W'>;
    NumericBackReferenceCharacter: ComplexToken<
      'NumericBackReferenceCharacter',
      {
        value: number;
        escaped: boolean;
      }
    >;
  };

type GroupModifiers =
  | undefined
  | {
    type: 'NON_CAPTURE';
    kind: RegExpGroupNonCapture['kind'];
  }
  | {
    type: 'NAMED_CAPTURE';
    name: string;
  };

type RegExpParserOptions = ParserOptions & {unicode: boolean};

function isHex(str: string): boolean {
  return !/[^0-9a-fA-F]/.test(str);
}

function isOct(str: string): boolean {
  const OCT_REGEX = /^[0-7]+$/;
  return OCT_REGEX.test(str);
}

function getCodePoint(char: string): number {
  if (char.length === 1) {
    const point = char.codePointAt(0);
    if (point !== undefined) {
      return point;
    }
  }

  throw new Error('Input was not 1 character long');
}

export const createRegExpParser = createParser((ParserCore) =>
  class RegExpParser extends ParserCore<Tokens, void> {
    constructor(opts: RegExpParserOptions) {
      super(opts, 'parse/regex');
      this.diagnostics = [];
      this.unicode = opts.unicode;
    }

    diagnostics: Diagnostics;
    unicode: boolean;

    addDiagnostic(opts: ParserUnexpectedOptions) {
      this.diagnostics.push(this.createDiagnostic(opts));
    }

    unexpected() {
      throw new Error('No throwing');
    }

    tokenize(index: Number0, input: string): TokenValues<Tokens> {
      const char = input[get0(index)];

      if (char === '\\') {
        let end = add(index, 2);

        const nextChar = input[get0(index) + 1];
        switch (nextChar) {
          case 't':
            return this.finishComplexToken('Character', {
              escaped: false,
              value: '\t',
            }, end);

          case 'n':
            return this.finishComplexToken('Character', {
              escaped: false,
              value: '\n',
            }, end);

          case 'r':
            return this.finishComplexToken('Character', {
              escaped: false,
              value: '\r',
            }, end);

          case 'v':
            return this.finishComplexToken('Character', {
              escaped: false,
              value: '\x0b',
            }, end);

          case 'f':
            return this.finishComplexToken('Character', {
              escaped: false,
              value: '\f',
            }, end);

          case 'b':
            return this.finishComplexToken('Character', {
              escaped: false,
              value: '\b',
            }, end);

          case 'd':
          case 'D':
          case 'B':
          case 's':
          case 'S':
          case 'w':
          case 'W':
            return this.finishValueToken('EscapedCharacter', nextChar, end);

          case 'k':
            if (this.unicode) {
              // TODO named group back reference https://github.com/tc39/proposal-regexp-named-groups#backreferences
            }

            return this.finishComplexToken('Character', {
              value: 'k',
              escaped: true,
            }, end);

          case 'p':
            if (this.unicode) {
              // TODO unicode property escapes https://github.com/tc39/proposal-regexp-unicode-property-escapes
            }

            return this.finishComplexToken('Character', {
              value: 'p',
              escaped: true,
            }, end);

          case 'P':
            if (this.unicode) {
              // TODO unicode property escapes https://github.com/tc39/proposal-regexp-unicode-property-escapes
            }

            return this.finishComplexToken('Character', {
              value: 'P',
              escaped: true,
            }, end);

          case 'c':
            // TODO???
            return this.finishComplexToken('Character', {
              value: 'c',
              escaped: true,
            }, end);

          case '0':
            // TODO octal

            return this.finishComplexToken('Character', {
              value: String.fromCharCode(0),
              escaped: true,
            }, end);

          case 'x':
            {
              const possibleHex = input.slice(get0(index) + 1, 2);

              // \xhh
              if (possibleHex.length === 2 && isHex(possibleHex)) {
                end = add(end, 2);

                return this.finishComplexToken('Character', {
                  value: String.fromCharCode(parseInt(possibleHex, 16)),
                  escaped: true,
                }, end);
              }

              return this.finishComplexToken('Character', {
                value: 'x',
                escaped: true,
              }, end);
            }

          case 'u':
            {
              // Get the next 4 characters after \u
              const possibleHex = input.slice(get0(index) + 2, get0(index) + 6);

              // \uhhhh
              if (possibleHex.length === 4 && isHex(possibleHex)) {
                end = add(end, 4);

                return this.finishComplexToken('Character', {
                  value: String.fromCharCode(parseInt(possibleHex, 16)),
                  escaped: true,
                }, end);
              }

              if (this.unicode) {
                // TODO \u{hhhh} or \u{hhhhh}
              }

              return this.finishComplexToken('Character', {
                value: 'u',
                escaped: true,
              }, end);
            }

          // Redundant escaping
          default:
            // TODO dangling backslash

            let char = nextChar;
            let backReference = '';
            let nextIndex: Number0 = add(index, 1);
            while (isDigit(char)) {
              backReference += char;
              // stop at max octal ascii in case of octal escape
              if (parseInt(backReference) > 377) {
                backReference = backReference.slice(0, backReference.length - 1);
                break;
              }
              nextIndex = add(nextIndex, 1);
              char = input[get0(nextIndex)];
            }

            if (backReference !== '') {
              const referenceValue = parseInt(backReference, 10);
              // back reference allowed are 1 - 99
              if (referenceValue >= 1 && referenceValue <= 99) {
                return this.finishComplexToken(
                  'NumericBackReferenceCharacter',
                  {
                    value: parseInt(backReference, 10),
                    escaped: true,
                  },
                  nextIndex,
                );
              } else {
                backReference = backReference.slice(0, backReference.length - 1);
                nextIndex = add(nextIndex, -1);
                return this.finishComplexToken(
                  'NumericBackReferenceCharacter',
                  {
                    value: parseInt(backReference, 10),
                    escaped: true,
                  },
                  nextIndex,
                );
              }
            }

            return this.finishComplexToken('Character', {
              value: nextChar,
              escaped: true,
            }, end);
        }
      }

      switch (char) {
        case '$':
        case '^':
        case '.':
        case '?':
        case '}':
        case '+':
        case '|':
        case '*':
        case '[':
        case ']':
        case '(':
        case ')':
          return this.finishValueToken('Operator', char);
      }

      return this.finishComplexToken('Character', {
        value: char,
        escaped: false,
      });
    }

    getGroupModifiers(): GroupModifiers {
      const token = this.getToken();

      if (token.type === 'Character') {
        switch (token.value) {
          case ':':
            this.nextToken();
            return {
              type: 'NON_CAPTURE',
              kind: undefined,
            };

          case '=':
            this.nextToken();
            return {
              type: 'NON_CAPTURE',
              kind: 'positive-lookahead',
            };

          case '!':
            this.nextToken();
            return {
              type: 'NON_CAPTURE',
              kind: 'negative-lookahead',
            };

          case '<':
            const nextToken = this.lookaheadToken();

            if (nextToken.type === 'Character') {
              switch (nextToken.value) {
                case '!':
                  this.nextToken();
                  this.nextToken();
                  return {
                    type: 'NON_CAPTURE',
                    kind: 'negative-lookbehind',
                  };

                case '=':
                  this.nextToken();
                  this.nextToken();
                  return {
                    type: 'NON_CAPTURE',
                    kind: 'positive-lookbehind',
                  };
              }

              if (isESIdentifierStart(nextToken.value)) {
                let name = '';

                // 1 is for the <
                let skipCount = 1;
                let targetToken: TokenValues<Tokens> = nextToken;
                while (targetToken.type === 'Character' && isESIdentifierChar(
                  targetToken.value,
                )) {
                  name += targetToken.value;
                  targetToken = this.lookaheadToken(targetToken.end);
                  skipCount++;
                }

                if (targetToken.type === 'Character' && targetToken.value === '>') {
                  // Skip through all the name tokens

                  // This is kinda a hacky solution, and slower than it could be
                  for (let i = 0; i < skipCount; i++) {
                    this.nextToken();
                  }

                  return {
                    type: 'NAMED_CAPTURE',
                    name,
                  };
                }
              }
            }
        }
      }

      this.addDiagnostic({
        ...descriptions.REGEX_PARSER.INVALID_CAPTURE_GROUP_MODIFIER,
        token,
      });
    }

    matchOperator(op: string): boolean {
      const token = this.getToken();
      return token.type === 'Operator' && token.value === op;
    }

    eatOperator(op: string): boolean {
      if (this.matchOperator(op)) {
        this.nextToken();
        return true;
      } else {
        return false;
      }
    }

    parseGroupCapture(): RegExpGroupCapture | RegExpGroupNonCapture {
      const start = this.getPosition();
      this.nextToken();

      let modifiers: GroupModifiers;
      if (this.eatOperator('?')) {
        modifiers = this.getGroupModifiers();
      }

      const expression = this.parseExpression(() => !this.matchOperator(')'));

      if (!this.eatOperator(')')) {
        this.addDiagnostic({
          ...descriptions.REGEX_PARSER.UNCLOSED_GROUP,
          start,
        });
      }

      if (modifiers !== undefined && modifiers.type === 'NON_CAPTURE') {
        return {
          type: 'RegExpGroupNonCapture',
          expression,
          kind: modifiers.kind,
          loc: this.finishLoc(start),
        };
      } else {
        let name = modifiers !== undefined ? modifiers.name : undefined;
        return {
          type: 'RegExpGroupCapture',
          expression,
          name,
          loc: this.finishLoc(start),
        };
      }
    }

    parseCharSet(): RegExpCharSet {
      const start = this.getPosition();
      this.nextToken();

      const body: RegExpCharSet['body'] = [];
      const invert = this.eatOperator('^');

      while (!this.matchToken('EOF') && !this.matchOperator(']')) {
        const part = this.parseCharacterOrRange();
        body.push(part);
      }

      if (!this.eatOperator(']')) {
        this.addDiagnostic({
          ...descriptions.REGEX_PARSER.UNCLOSED_CHAR_SET,
          start,
        });
      }

      return {
        type: 'RegExpCharSet',
        invert,
        body,
        loc: this.finishLoc(start),
      };
    }

    getCharacterFromToken(token: TokenValues<Tokens>): string {
      switch (token.type) {
        case 'Character':
        case 'Operator':
          return token.value;

        case 'SOF':
        case 'EOF':
        case 'Invalid':
          throw new Error('Unnecessary');

        default:
          throw new Error('Never');
      }
    }

    parseCharacter(): AnyRegExpEscapedCharacter {
      const token = this.getToken();

      if (token.type === 'Character') {
        this.nextToken();
        return {
          type: 'RegExpCharacter',
          value: token.value,
          loc: this.finishLocFromToken(token),
        };
      }

      if (token.type === 'NumericBackReferenceCharacter') {
        this.nextToken();
        const value = token.value;
        // \8 \9 are treated as escape char
        if (value === 8 || value === 9) {
          return {
            type: 'RegExpCharacter',
            value: String(value),
            loc: this.finishLocFromToken(token),
          };
        }

        // octal escapes
        if (isOct(String(value))) {
          const octal = parseInt(String(value), 8);
          return {
            type: 'RegExpCharacter',
            value: String.fromCharCode(octal),
            loc: this.finishLocFromToken(token),
          };
        }

        return {
          type: 'RegExpNumericBackReference',
          value: token.value,
          loc: this.finishLocFromToken(token),
        };
      }

      if (token.type === 'EscapedCharacter') {
        this.nextToken();

        const loc = this.finishLocFromToken(token);
        switch (token.value) {
          case 'd':
            return {
              type: 'RegExpDigitCharacter',
              loc,
            };

          case 'D':
            return {
              type: 'RegExpNonDigitCharacter',
              loc,
            };

          case 'b':
            return {
              type: 'RegExpWordBoundaryCharacter',
              loc,
            };

          case 'B':
            return {
              type: 'RegExpNonWordBoundaryCharacter',
              loc,
            };

          case 's':
            return {
              type: 'RegExpWhiteSpaceCharacter',
              loc,
            };

          case 'S':
            return {
              type: 'RegExpNonWhiteSpaceCharacter',
              loc,
            };

          case 'w':
            return {
              type: 'RegExpWordCharacter',
              loc,
            };

          case 'W':
            return {
              type: 'RegExpNonWordCharacter',
              loc,
            };
        }
      }

      this.nextToken();
      return {
        type: 'RegExpCharacter',
        value: this.getCharacterFromToken(token),
        loc: this.finishLocFromToken(token),
      };
    }

    parseCharacterOrRange(): AnyRegExpEscapedCharacter | RegExpCharSetRange {
      const startPos = this.getPosition();
      let start = this.parseCharacter();

      // Range
      const nextToken = this.getToken();
      if (start.type === 'RegExpCharacter' && nextToken.type === 'Character' &&
        nextToken.value === '-' && !nextToken.escaped) {
        const lookaheadToken = this.lookaheadToken();
        if (lookaheadToken.type === 'Character') {
          // Skip dash
          this.nextToken();

          let end = this.parseCharacter();

          const loc = this.finishLoc(startPos);

          if (
            start.type === 'RegExpCharacter' && end.type === 'RegExpCharacter' &&
              getCodePoint(end.value) < getCodePoint(start.value)
          ) {
            this.addDiagnostic({
              ...descriptions.REGEX_PARSER.REVERSED_CHAR_SET_RANGE,
              loc,
            });
            const _end = end;
            end = start;
            start = _end;
          }

          return {
            type: 'RegExpCharSetRange',
            loc,
            start,
            end,
          };
        }
      }

      return start;
    }

    parseDigits(): undefined | number {
      let digits = '';
      let token = this.getToken();
      while (token.type === 'Character' && isDigit(token.value)) {
        digits += token.value;
        token = this.nextToken();
      }

      if (digits.length === 0) {
        return undefined;
      } else {
        return Number(digits);
      }
    }

    parseQuantifier():
      | undefined
      | {
        min: number;
        max?: number;
      } {
      if (this.eatOperator('?')) {
        return {
          min: 0,
          max: 1,
        };
      }

      if (this.eatOperator('*')) {
        return {
          min: 0,
          max: undefined,
        };
      }

      if (this.eatOperator('+')) {
        return {
          min: 1,
          max: undefined,
        };
      }

      if (this.matchOperator('{')) {
        const snapshot = this.save();

        this.nextToken();

        const min = this.parseDigits();

        if (min !== undefined) {
          const nextToken = this.getToken();
          if (nextToken.type === 'Character' && nextToken.value === ',') {
            this.nextToken();
            const max = this.parseDigits();

            const endToken = this.getToken();
            if (endToken.type === 'Operator' && endToken.value === '}') {
              return {
                min,
                max,
              };
            }
          } else if (nextToken.type === 'Operator' && nextToken.value === '}') {
            return {
              min,
              max: min,
            };
          }
        }

        this.restore(snapshot);
      }
    }

    parseBodyItem(): undefined | AnyRegExpBodyItem {
      const start = this.getPosition();

      const prefix = this.parseBodyItemPrefix();
      if (prefix === undefined) {
        return undefined;
      }

      let target = prefix;

      while (true) {
        const quantifier = this.parseQuantifier();
        if (quantifier === undefined) {
          break;
        }

        const lazy = this.eatOperator('?');

        const quantified: RegExpQuantified = {
          type: 'RegExpQuantified',
          loc: this.finishLoc(start),
          target,
          lazy,
          ...quantifier,
        };

        target = quantified;
      }

      return target;
    }

    parseOperator(token: Tokens['Operator']): undefined | AnyRegExpBodyItem {
      switch (token.value) {
        case '$':
          this.nextToken();
          return {
            type: 'RegExpEndCharacter',
            loc: this.finishLocFromToken(token),
          };

        case '^':
          this.nextToken();
          return {
            type: 'RegExpStartCharacter',
            loc: this.finishLocFromToken(token),
          };

        case '.':
          this.nextToken();
          return {
            type: 'RegExpAnyCharacter',
            loc: this.finishLocFromToken(token),
          };

        case '[':
          return this.parseCharSet();

        case '(':
          return this.parseGroupCapture();

        case ')':
          this.nextToken();
          this.addDiagnostic({
            ...descriptions.REGEX_PARSER.UNOPENED_GROUP,
            token,
          });
          return;

        case '?':
        case '*':
        case '+':
          this.nextToken();
          this.addDiagnostic({
            ...descriptions.REGEX_PARSER.INVALID_QUANTIFIER_TARGET,
            token,
          });
          return;

        case ']':
        case '}':
          return this.parseCharacter();
      }
    }

    parseBodyItemPrefix(): undefined | AnyRegExpBodyItem {
      const token = this.getToken();

      switch (token.type) {
        case 'Operator':
          return this.parseOperator(token);

        case 'EscapedCharacter':
        case 'Character':
        case 'NumericBackReferenceCharacter':
          return this.parseCharacter();
      }

      this.addDiagnostic({
        ...descriptions.REGEX_PARSER.UNKNOWN_REGEX_PART,
        token,
      });
    }

    parseExpression(
      whileCallback?: () => boolean,
    ): RegExpSubExpression | RegExpAlternation {
      const alternations: Array<{
        start: Position;
        end: Position;
        body: Array<AnyRegExpBodyItem>;
      }> = [];
      let body: Array<AnyRegExpBodyItem> = [];

      const start = this.getPosition();
      let alternateStart = start;

      while (!this.matchToken('EOF') && (whileCallback === undefined ||
      whileCallback())) {
        if (this.eatOperator('|')) {
          alternations.push({
            start: alternateStart,
            end: this.getPosition(),
            body,
          });
          alternateStart = this.getPosition();
          body = [];
          continue;
        }

        const part = this.parseBodyItem();
        if (part !== undefined) {
          body.push(part);
        }
      }

      alternations.push({
        body,
        start: alternateStart,
        end: this.getPosition(),
      });

      let expression: undefined | RegExpSubExpression | RegExpAlternation;

      while (alternations.length > 0) {
        const alternation = alternations.shift();
        if (alternation === undefined) {
          throw new Error('Impossible. We check it above.');
        }

        const sub: RegExpSubExpression = {
          type: 'RegExpSubExpression',
          body: alternation.body,
          loc: this.finishLocAt(alternation.start, alternation.end),
        };

        if (expression === undefined) {
          expression = sub;
        } else {
          const alternationNode: RegExpAlternation = {
            type: 'RegExpAlternation',
            left: expression,
            right: sub,
            loc: this.finishLocAt(this.getLoc(expression).start, alternation.end),
          };

          expression = alternationNode;
        }
      }

      if (expression === undefined) {
        throw new Error(
          'Impossible. We should always have at least one alternation that will set this.',
        );
      }

      return expression;
    }

    parse(): {
      expression: AnyRegExpExpression;
      diagnostics: Diagnostics;
    } {
      return {
        expression: this.parseExpression(),
        diagnostics: this.diagnostics,
      };
    }
  }
);
