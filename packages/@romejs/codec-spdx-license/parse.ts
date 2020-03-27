/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  ParserOptions,
  ComplexNode,
  BaseTokens,
  ValueToken,
  SimpleToken,
  createParser,
  isAlpha,
  isDigit,
} from '@romejs/parser-core';
import {getSPDXLicense, licenseNames} from './index';
import {descriptions} from '@romejs/diagnostics';
import {inc, Number0, get0} from '@romejs/ob1';

//# Tokens
type Tokens =
  & BaseTokens
  & {
    ParenOpen: SimpleToken<'ParenOpen'>;
    ParenClose: SimpleToken<'ParenClose'>;
    Plus: SimpleToken<'Plus'>;
    And: SimpleToken<'And'>;
    With: SimpleToken<'With'>;
    Or: SimpleToken<'Or'>;
    Word: ValueToken<'Word', string>;
  };

//# Nodes
export type ExpressionNode = LicenseNode | AndNode | OrNode;

type AndNode = ComplexNode<'And', {
  left: ExpressionNode;
  right: ExpressionNode;
}>;

type OrNode = ComplexNode<'Or', {
  left: ExpressionNode;
  right: ExpressionNode;
}>;

type LicenseNode = ComplexNode<'License', {
  plus: boolean;
  id: string;
  exception: undefined | string;
}>;

function isWordChar(char: string) {
  return isAlpha(char) || isDigit(char) || char === '-' || char === '.';
}

type SPDXLicenseParserOptions = ParserOptions & {loose?: boolean};

const createSPDXLicenseParser = createParser((ParserCore) =>
  class SPDXLicenseParser extends ParserCore<Tokens, void> {
    constructor(opts: SPDXLicenseParserOptions) {
      super(opts, 'parse/spdxLicense');
      this.loose = opts.loose === true;
    }

    loose: boolean;

    // For some reason Flow will throw an error without the type casts...
    tokenize(index: Number0, input: string) {
      const char = input[get0(index)];

      if (char === '+') {
        return this.finishToken('Plus');
      }

      if (char === '(') {
        return this.finishToken('ParenOpen');
      }

      if (char === ')') {
        return this.finishToken('ParenClose');
      }

      // Skip spaces
      if (char === ' ') {
        return this.lookaheadToken(inc(index));
      }

      if (isWordChar(char)) {
        const [value, end] = this.readInputFrom(index, isWordChar);

        if (value === 'AND') {
          return this.finishToken('And', end);
        } else if (value === 'OR') {
          return this.finishToken('Or', end);
        } else if (value === 'WITH') {
          return this.finishToken('With', end);
        } else {
          return this.finishValueToken('Word', value, end);
        }
      }
    }

    parseLicense(token: Tokens['Word']): LicenseNode {
      const startPos = this.getPosition();
      this.nextToken();

      // Validate id
      const id = token.value;
      let licenseInfo = getSPDXLicense(id);
      const nextToken = this.getToken();

      // Sometimes licenses will be specified as "Apache 2.0" but what they actually meant was "Apache-2.0"

      // In loose mode, just make it equivalent, otherwise, complain
      if (licenseInfo === undefined && nextToken.type === 'Word') {
        const possibleCorrectLicense = `${id}-${nextToken.value}`;
        const possibleLicenseInfo = getSPDXLicense(possibleCorrectLicense);

        if (possibleLicenseInfo !== undefined) {
          if (this.loose) {
            // Just allow it...
            licenseInfo = possibleLicenseInfo;
            this.nextToken();
          } else {
            throw this.unexpected({
              description: descriptions.SPDX.VALID_LICENSE_WITH_MISSING_DASH(
                possibleCorrectLicense,
              ),
              start: this.getPositionFromIndex(token.start),
              end: this.getPositionFromIndex(nextToken.end),
            });
          }
        }
      }

      if (licenseInfo === undefined) {
        throw this.unexpected({
          description: descriptions.SPDX.UNKNOWN_LICENSE(id, licenseNames),
          start: this.getPositionFromIndex(token.start),
          end: this.getPositionFromIndex(token.end),
        });
      }

      // Is this a plus? (wtf is this)
      const plus = this.eatToken('Plus') !== undefined;

      // Get exception
      let exception;
      if (this.eatToken('With')) {
        const token = this.getToken();
        if (token.type === 'Word') {
          exception = token.value;
          this.nextToken();
        } else {
          throw this.unexpected({
            description: descriptions.SPDX.WITH_RIGHT_LICENSE_ONLY,
          });
        }
      }

      return {
        type: 'License',
        loc: this.finishLoc(startPos),
        id,
        exception,
        plus,
      };
    }

    parseExpression(): ExpressionNode {
      const startPos = this.getPosition();
      const startToken = this.getToken();

      let value;

      switch (startToken.type) {
        case 'ParenOpen':
          this.nextToken();
          value = this.parseExpression();
          this.expectToken('ParenClose');
          break;

        case 'Word':
          value = this.parseLicense(startToken);
          break;

        case 'Or':
        case 'And':
          throw this.unexpected({
            description: descriptions.SPDX.OPERATOR_NOT_BETWEEN_EXPRESSION,
          });

        case 'Plus':
          throw this.unexpected({
            description: descriptions.SPDX.PLUS_NOT_AFTER_LICENSE,
          });

        case 'ParenClose':
          throw this.unexpected({
            description: descriptions.SPDX.UNOPENED_PAREN,
          });

        default:
          throw this.unexpected();
      }

      // Parse and/or
      const nextToken = this.getToken();
      switch (nextToken.type) {
        case 'Or':
          this.nextToken();
          return {
            type: 'Or',
            loc: this.finishLoc(startPos),
            left: value,
            right: this.parseExpression(),
          };

        case 'And':
          this.nextToken();
          return {
            type: 'And',
            loc: this.finishLoc(startPos),
            left: value,
            right: this.parseExpression(),
          };

        default:
          return value;
      }
    }

    parse(): ExpressionNode {
      const expr = this.parseExpression();
      this.finalize();
      return expr;
    }
  }
);

export default function parse(opts: SPDXLicenseParserOptions): ExpressionNode {
  return createSPDXLicenseParser(opts).parse();
}
