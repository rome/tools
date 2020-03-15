/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticPointer, DiagnosticCategory} from '@romejs/diagnostics';
import {
  JSONParserResult,
  JSONParserOptions,
  Tokens,
  Comments,
  PathToComments,
  PathComments,
  JSONValue,
  JSONObject,
} from './types';
import {
  ConsumePath,
  ConsumeContext,
  ConsumeSourceLocationRequestTarget,
} from '@romejs/consume';
import {unescapeString} from '@romejs/string-escape';
import messages from './messages';
import {
  isAlpha,
  isDigit,
  isEscaped,
  Position,
  SourceLocation,
  createParser,
} from '@romejs/parser-core';
import {inc, Number0, add, get0, sub} from '@romejs/ob1';

// Words can't start with a digit
function isWordStartChar(char: string): boolean {
  return isAlpha(char) || char === '_' || char === '$';
}

// But a digit can appear inside of a word
function isWordChar(char: string): boolean {
  return isWordStartChar(char) || isDigit(char);
}

// Check if an input string is a valid word, this is used by the stringifier to
// determine if a property key should be quoted
export function isValidWord(word: string): boolean {
  if (word.length === 0 || isWordStartChar(word[0]) === false) {
    return false;
  }

  for (const char of word) {
    if (isWordChar(char) === false) {
      return false;
    }
  }

  return true;
}

// Check if a character is a part of a string, returning false for a newline or unescaped quote char
function isStringValueChar(
  char: string,
  index: Number0,
  input: string,
): boolean {
  if (char === '\n') {
    return false;
  }

  if (char === '"' && !isEscaped(index, input)) {
    return false;
  }

  return true;
}

// Turn a path into a string key we can use
export function toPathKey(parts: Array<string>) {
  // Right now this could conflict weirdly with properties with dots in them if they cause collisions
  // We have this method abstracted so we can make changes later if it's necessary (probably not worth it)
  return parts.join('.');
}

function isntNewline(char: string): boolean {
  return char !== '\n';
}

function isntBlockCommentEnd(
  char: string,
  index: Number0,
  input: string,
): boolean {
  const nextChar = input[get0(index) + 1];
  return char !== '*' && nextChar !== '/';
}

// Used for Number token validation, allow underscore as a separatore
function isNumberChar(char: string): boolean {
  return isDigit(char) || char === '_';
}

type PathInfo = {
  originalValue: unknown;
  keyStart: Position;
  keyEnd: Position;
  valueStart: Position;
  valueEnd: Position;
};

export default createParser(
  ParserCore =>
    class JSONParser extends ParserCore<Tokens, void> {
      constructor(opts: JSONParserOptions) {
        super(opts, 'parse/json');
        this.options = opts;
        this.ignoreWhitespaceTokens = true;

        this.hasExtensions =
          this.path !== undefined && this.path.getBasename().endsWith('.rjson');

        this.pathKeys = [];
        this.paths = new Map();
        this.pathToComments = new Map();
        this.consumeDiagnosticCategory =
          opts.consumeDiagnosticCategory === undefined
            ? 'parse/json'
            : opts.consumeDiagnosticCategory;
      }

      pathToComments: PathToComments;
      hasExtensions: boolean;
      pathKeys: ConsumePath;
      paths: Map<string, PathInfo>;
      options: JSONParserOptions;
      consumeDiagnosticCategory: DiagnosticCategory;

      getPathInfo(path: ConsumePath): undefined | PathInfo {
        return this.paths.get(path.join('.'));
      }

      setComments(pathComments: PathComments) {
        const key = this.pathKeys.join('.');

        const existing = this.pathToComments.get(key);
        if (existing === undefined) {
          this.pathToComments.set(key, pathComments);
        } else {
          this.pathToComments.set(key, {
            inner: [...existing.inner, ...pathComments.inner],
            outer: [...existing.outer, ...pathComments.outer],
          });
        }
      }

      setPath(info: PathInfo) {
        this.paths.set(this.pathKeys.join('.'), info);
        this.pathKeys.pop();
      }

      tokenize(index: Number0, input: string) {
        const nextChar = input[get0(index) + 1];
        const char = input[get0(index)];

        // Line comment
        if (char === '/' && nextChar === '/') {
          const commentValueIndex = add(index, 2);
          const [value] = this.readInputFrom(commentValueIndex, isntNewline);
          // (comment content start + comment content length)
          return this.finishValueToken(
            'LineComment',
            value,
            add(commentValueIndex, value.length),
          );
        }

        // BlockComment
        if (char === '/' && nextChar === '*') {
          const commentValueIndex = add(index, 2);
          const [value] = this.readInputFrom(
            commentValueIndex,
            isntBlockCommentEnd,
          );

          // (comment content start + comment content length + 2 characters for comment end)
          const endIndex = add(add(commentValueIndex, value.length), 2);

          // Ensure the comment is closed
          if (
            this.input[get0(endIndex) - 2] !== '*' ||
            this.input[get0(endIndex) - 1] !== '/'
          ) {
            throw this.unexpected({
              message: messages.UNCLOSED_BLOCK_COMMENT(),
              start: this.getPositionFromIndex(endIndex),
            });
          }

          return this.finishValueToken('BlockComment', value, endIndex);
        }

        // Single character token starters
        switch (char) {
          case '"':
            const [value] = this.readInputFrom(inc(index), isStringValueChar);

            // Check for closed string (index is the current token index + string length + closing quote + 1 for the end char)
            const end = add(add(index, value.length), 2);
            if (input[get0(end) - 1] !== '"') {
              throw this.unexpected({
                message: messages.UNCLOSED_STRING(),
                start: this.getPositionFromIndex(end),
              });
            }

            // Don't allow newlines in JSON
            for (let strIndex = 0; strIndex < value.length; strIndex++) {
              const char = value[strIndex];

              if (char === '\n') {
                throw this.unexpected({
                  message: messages.STRING_NEWLINES_IN_JSON(),
                  start: this.getPositionFromIndex(add(index, strIndex)),
                });
              }
            }

            // Unescape the string
            const unescaped = unescapeString(value, (message, strIndex) => {
              throw this.unexpected({
                message,
                start: this.getPositionFromIndex(add(index, strIndex)),
              });
            });

            return this.finishValueToken('String', unescaped, end);

          case "'":
            throw this.unexpected({
              message: messages.SINGLE_QUOTE_USAGE(),
              start: this.getPositionFromIndex(index),
            });

          case '/':
            throw this.unexpected({
              message: messages.REGEX_IN_JSON(),
              start: this.getPositionFromIndex(index),
            });

          case ',':
            return this.finishToken('Comma');

          case '.':
            return this.finishToken('Dot');

          case '-':
            return this.finishToken('Minus');

          case '+':
            return this.finishToken('Plus');

          case ':':
            return this.finishToken('Colon');

          case '{':
            return this.finishToken('BraceOpen');

          case '}':
            return this.finishToken('BraceClose');

          case '[':
            return this.finishToken('BracketOpen');

          case ']':
            return this.finishToken('BracketClose');
        }

        // Numbers
        if (isDigit(char)) {
          const value = this.removeUnderscores(
            index,
            this.readInputFrom(index, isNumberChar)[0],
          );
          const num = Number(value);
          return this.finishValueToken('Number', num, add(index, value.length));
        }

        // Word - boolean, undefined etc
        if (isWordStartChar(char)) {
          const [value] = this.readInputFrom(index, isWordChar);
          return this.finishValueToken('Word', value, add(index, value.length));
        }

        // Unknown character
        return undefined;
      }

      parseObject(firstKeyStart?: Position, firstKey?: string): JSONObject {
        const obj: JSONObject = {};

        let innerComments: Comments = [];
        let isFirstProp = true;

        // These are comments that the next property should take in case the previous accidently took them
        let nextLeadingComments;

        do {
          if (this.matchToken('BraceClose')) {
            break;
          }

          // Eat all the comments that appeared before this property, it's the most common and natural place to put them,
          // and is where we'll print all comments for a property.
          let leadingComments = this.eatComments();

          // Take any leading comments that were left by the previous property
          if (nextLeadingComments !== undefined) {
            leadingComments = [...nextLeadingComments, ...leadingComments];
            nextLeadingComments = undefined;
          }

          // Throw a meainingful error for redundant commas
          if (this.matchToken('Comma')) {
            throw this.unexpected({
              message: messages.REDUNDANT_COMMA(),
            });
          }

          // If there's no property key indicator then delegate any comments we have to object
          const hasKey = isFirstProp && firstKey !== undefined;
          if (
            !hasKey &&
            !this.matchToken('String') &&
            !this.matchToken('Word')
          ) {
            innerComments = [...innerComments, ...leadingComments];
            break;
          }

          const keyStart =
            isFirstProp && firstKeyStart !== undefined
              ? firstKeyStart
              : this.getPosition();

          // Parse the property key
          let key;
          if (isFirstProp && firstKey !== undefined) {
            // If this is the first property and we've been given a property key then use it instead
            key = firstKey;
          } else {
            key = this.parsePropertyKey();
          }
          isFirstProp = false;

          const keyEnd = this.getPosition();
          this.expectToken('Colon');

          // Having comments before the value is a really weird place to put them, but we'll handle it
          // anyway to avoid throwing a parser error. When stringified, the comments will all be before
          // the property.
          const leadingValueComments = this.eatComments();

          this.pathKeys.push(key);

          // Parse the value.
          const valueStart = this.getPosition();
          const value = this.parseExpression();
          const valueEnd = this.getPrevEndPosition();

          // Eat the comments after the expression and associate the comments with them
          let trailingValueComments = this.eatComments();

          // If the next token isn't a comma or closing brace then we've just stolen
          // the leading comments of the next property
          if (!this.matchToken('Comma') && !this.matchToken('BraceClose')) {
            nextLeadingComments = trailingValueComments;
            trailingValueComments = [];
          }

          this.setComments({
            inner: [],
            outer: [
              ...leadingComments,
              ...leadingValueComments,
              ...trailingValueComments,
            ],
          });

          this.setPath({
            keyStart,
            keyEnd,
            valueStart,
            valueEnd,
            originalValue: value,
          });

          // Set the object correctly, accounting for JS weirdness
          if (key === '__proto__') {
            // Need to use defineProperty to avoid triggering the Object.prototype.__proto__ setter
            Object.defineProperty(obj, '__proto__', {
              value,
              configurable: true,
              writable: true,
              enumerable: true,
            });
          } else {
            obj[key] = value;
          }
        } while (this.eatPropertySeparator());

        // Take any loose leading comments
        if (nextLeadingComments !== undefined) {
          innerComments = [...innerComments, ...nextLeadingComments];
        }

        // If we were passed a first key then this was an implicit object so there's no end token
        if (firstKey === undefined) {
          this.expectToken('BraceClose');
        }

        this.setComments({
          inner: innerComments,
          outer: [],
        });

        return obj;
      }

      // Remove underscores from 'a string, this is used for numeric separators eg. 100_000
      removeUnderscores(index: Number0, raw: string): string {
        let str = '';

        for (let i = 0; i < raw.length; i++) {
          const char = raw[i];

          if (char === '_') {
            // Don't allow separators in JSON
            if (!this.hasExtensions) {
              throw this.unexpected({
                message: messages.NUMERIC_SEPARATORS_IN_JSON(),
                start: this.getPositionFromIndex(inc(index)),
              });
            }
          } else {
            str += char;
          }
        }

        return str;
      }

      eatComments(): Comments {
        const comments: Comments = [];

        while (true) {
          const token = this.getToken();

          if (token.type === 'LineComment') {
            comments.push({
              type: 'LineComment',
              value: token.value,
            });
          } else if (token.type === 'BlockComment') {
            comments.push({
              type: 'BlockComment',
              value: token.value,
            });
          } else {
            break;
          }

          // Comments aren't allowed in regular JSON
          if (!this.hasExtensions) {
            throw this.unexpected({message: messages.COMMENTS_IN_JSON()});
          }

          this.nextToken();
        }

        return comments;
      }

      parseArray(): Array<JSONValue> {
        this.expectToken('BracketOpen');

        const arr = [];
        let innerComments: Comments = [];
        let i = 0;

        do {
          if (this.matchToken('BracketClose')) {
            break;
          }

          // Eat all the comments before an element
          const leadingComments = this.eatComments();

          if (this.matchToken('Comma')) {
            throw this.unexpected({
              message: messages.REDUNDANT_COMMA(),
            });
          }

          // If we're at the end of the array then associate these comments with the array
          if (this.matchToken('BracketClose')) {
            innerComments = [...innerComments, ...leadingComments];
            break;
          }

          const start = this.getPosition();
          this.pathKeys.push(i);
          i++;

          // Parse the value
          const item = this.parseExpression();
          arr.push(item);
          const end = this.getPrevEndPosition();

          // Trailing comments are really weird, but let's handle them just like object properties
          const trailingComments = this.eatComments();

          this.setComments({
            outer: [...leadingComments, ...trailingComments],
            inner: [],
          });

          this.setPath({
            originalValue: item,
            keyStart: start,
            keyEnd: end,
            valueStart: start,
            valueEnd: end,
          });

          // Have a meaningful error message when an object is incorrectly using brackets: ["foo": "bar"]
          if (this.matchToken('Colon')) {
            throw this.unexpected({
              message: messages.MISTAKEN_ARRAY_IDENTITY(),
            });
          }
        } while (this.eatPropertySeparator());

        this.expectToken('BracketClose');

        this.setComments({
          inner: innerComments,
          outer: [],
        });

        return arr;
      }

      // Check if the current token is a property separator and eat it if necessary
      eatPropertySeparator(): boolean {
        const token = this.getToken();

        // Implicit commas are only allowed in rjson
        if (this.hasExtensions) {
          // Eat the token, don't care if we're in RJSON
          if (token.type === 'Comma') {
            this.nextToken();
          }

          // An object or array close is an instant failure
          // Doesn't matter what we're parsing since the subsequent tokens will be validated
          if (token.type === 'BraceClose' || token.type === 'BracketClose') {
            return false;
          }

          return true;
        } else {
          if (token.type !== 'Comma') {
            return false;
          }
          // Make sure this isn't a trailing comma
          const lookahead = this.lookaheadToken();
          if (
            lookahead.type === 'BraceClose' ||
            lookahead.type === 'BracketClose'
          ) {
            throw this.unexpected({message: messages.TRAILING_COMMA_IN_JSON()});
          }

          this.nextToken();
          return true;
        }
      }

      parseWord(isStart: boolean): JSONValue {
        const start = this.getPosition();
        const token = this.expectToken('Word');

        switch (token.value) {
          case 'true':
            return true;

          case 'false':
            return false;

          case 'null':
            return null;

          case 'undefined':
            throw this.unexpected({message: messages.UNDEFINED_IN_JSON()});
        }

        if (isStart && this.matchToken('Colon')) {
          if (this.hasExtensions) {
            return this.parseObject(start, token.value);
          } else {
            throw this.unexpected({
              message: messages.IMPLICIT_OBJECT_IN_JSON(),
            });
          }
        }

        throw this.unexpected({
          message: messages.UNKNOWN_WORD_IN_JSON(token.value),
        });
      }

      parseNumber(): number {
        const isNegative = this.eatToken('Minus') !== undefined;

        // Get a string of the current number that we'll parse later
        const token = this.expectToken('Number');
        let value: string = String(token.value);

        // Decimals
        if (this.eatToken('Dot')) {
          value += '.';

          const decimal = this.expectToken('Number');
          value += String(decimal.value);
        }

        // Scientific notation
        const nextToken = this.getToken();
        if (
          nextToken.type === 'Word' &&
          (nextToken.value === 'e' || nextToken.value === 'E')
        ) {
          value += 'e';

          // Operator
          const operator = this.nextToken();
          if (operator.type === 'Minus') {
            value += '-';
          } else if (operator.type === 'Plus') {
            value += '+';
          } else {
            throw this.unexpected();
          }

          // Factor
          this.nextToken();
          const factor = this.expectToken('Number');
          value += String(factor.value);
        }

        // BigInt
        const nextToken2 = this.getToken();
        if (nextToken2.type === 'Word' && nextToken2.value === 'n') {
          throw this.unexpected({
            message: messages.BIGINT_IN_JSON(),
          });
        }

        // Turn the string into an actual number
        let num = Number(value);
        if (isNegative) {
          num = -num;
        }
        return num;
      }

      parsePropertyKey() {
        const token = this.getToken();

        switch (token.type) {
          case 'String':
            this.nextToken();
            return token.value;

          case 'Word':
            if (this.hasExtensions) {
              this.nextToken();
              return token.value;
            } else {
              throw this.unexpected({
                message: messages.PROPERTY_KEY_UNQUOTED_IN_JSON(),
              });
            }

          default:
            throw this.unexpected();
        }
      }

      parseString(isStart: boolean): string | JSONObject {
        const start = this.getPosition();
        const token = this.expectToken('String');

        if (isStart && this.nextToken().type === 'Colon') {
          if (this.hasExtensions) {
            return this.parseObject(start, token.value);
          } else {
            throw this.unexpected({
              message: messages.IMPLICIT_OBJECT_IN_JSON(),
            });
          }
        } else {
          return token.value;
        }
      }

      parseExpression(isStart: boolean = false): JSONValue {
        const token = this.getToken();

        switch (token.type) {
          case 'String':
            return this.parseString(isStart);

          case 'Minus':
          case 'Number':
            return this.parseNumber();

          case 'Word':
            return this.parseWord(isStart);

          case 'BracketOpen':
            return this.parseArray();

          case 'BraceOpen':
            this.nextToken();
            return this.parseObject();

          default:
            throw this.unexpected();
        }
      }

      parseEntry(): JSONValue {
        if (this.matchToken('EOF')) {
          if (this.hasExtensions) {
            // If we're in RJSON mode then an empty input is an implicit object
            return {};
          } else {
            throw this.unexpected({message: messages.EMPTY_INPUT_IN_JSON()});
          }
        } else {
          return this.parseExpression(true);
        }
      }

      parse(): JSONParserResult {
        let expectSyntaxError = false;

        if (!this.hasExtensions) {
          // If we're in regular JSON, try the native JSON.parse
          try {
            const value = JSON.parse(this.input);

            // Lazy parse when we need location information
            let context: undefined | Required<ConsumeContext>;
            const getContext = (): Required<ConsumeContext> => {
              if (context === undefined) {
                const res = this._parse();
                context = res.context;
                return res.context;
              } else {
                return context;
              }
            };

            return {
              context: {
                category: this.consumeDiagnosticCategory,
                getOriginalValue(path) {
                  return getContext().getOriginalValue(path);
                },

                getDiagnosticPointer(keys, target) {
                  return getContext().getDiagnosticPointer(keys, target);
                },
              },
              value,
            };
          } catch (err) {
            // On syntax errors we'll fall back to our parser which is slower, but produces more meaningful errors
            if (err instanceof SyntaxError) {
              expectSyntaxError = true;
            } else {
              throw err;
            }
          }
        }

        const res: JSONParserResult = this._parse();

        if (expectSyntaxError) {
          throw new Error(
            "JSON.parse failed but our custom JSON parser was successful... That doesn't smell right",
          );
        }

        return res;
      }

      _parse(): JSONParserResult {
        const leadingComments = this.eatComments();

        const expr = this.parseEntry();

        const trailingComments = this.eatComments();
        this.setComments({
          inner: [],
          outer: [...leadingComments, ...trailingComments],
        });

        this.finalize();

        const context: Required<ConsumeContext> = {
          category: this.consumeDiagnosticCategory,

          getDiagnosticPointer: (
            keys: ConsumePath,
            target: ConsumeSourceLocationRequestTarget,
          ): undefined | DiagnosticPointer => {
            const info = this.getPathInfo(keys);
            if (info === undefined) {
              return;
            }

            let start = info.keyStart;
            let end = info.valueEnd;

            if (target === 'key') {
              end = info.keyEnd;
            }

            if (target === 'value' || target === 'inner-value') {
              start = info.valueStart;
            }

            let loc: SourceLocation = {
              filename: this.filename,
              start,
              end,
            };

            if (target === 'inner-value') {
              const originalValue = context.getOriginalValue(keys);

              // Remove quote marks for strings
              if (typeof originalValue === 'string') {
                loc = {
                  ...loc,
                  start: {
                    ...loc.start,
                    column: add(loc.start.column, 1),
                  },
                  end: {
                    ...loc.end,
                    column: sub(loc.end.column, 1),
                  },
                };
              }
            }

            return {
              language: 'json',
              ...loc,
              mtime: this.mtime,
              sourceText: undefined,
            };
          },

          getOriginalValue: (keys: ConsumePath) => {
            const info = this.getPathInfo(keys);
            if (info !== undefined) {
              return info.originalValue;
            }
          },
        };

        return {
          value: expr,
          context,
        };
      }
    },
);
