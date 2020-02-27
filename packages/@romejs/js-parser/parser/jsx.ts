/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {types as tt} from '../tokenizer/types';
import {Position, SourceLocation} from '@romejs/parser-core';
import {JSParser} from '../parser';
import {
  JSXElement,
  JSXIdentifier,
  JSXNamespacedName,
  StringLiteral,
  JSXFragment,
  JSXExpressionContainer,
  JSXSpreadChild,
  JSXEmptyExpression,
  JSXSpreadAttribute,
  JSXAttribute,
  JSXText,
} from '@romejs/js-ast';
import {
  parseExpression,
  parseMaybeAssign,
  parseTSTypeArguments,
  parseStringLiteral,
} from './index';
import {PartialDiagnosticAdvice} from '@romejs/diagnostics';

// Transforms JSX element name to string.
function getQualifiedJSXName(node: JSXElement['name'] | JSXIdentifier): string {
  if (node === undefined) {
    return '';
  }

  switch (node.type) {
    case 'JSXIdentifier':
    case 'JSXReferenceIdentifier':
      return node.name;

    case 'JSXNamespacedName':
      return node.namespace.name + ':' + node.name.name;

    case 'JSXMemberExpression':
      return (
        getQualifiedJSXName(node.object) +
        '.' +
        getQualifiedJSXName(node.property)
      );
  }
}

// Parse next token as JSX identifier
function parseJSXIdentifier(parser: JSParser): JSXIdentifier {
  const start = parser.getPosition();
  let name;
  if (parser.match(tt.jsxName)) {
    name = String(parser.state.tokenValue);
  } else if (parser.state.tokenType.keyword !== undefined) {
    name = parser.state.tokenType.keyword;
  } else {
    parser.addDiagnostic({
      message: 'Unknown JSX identifier token',
    });
    name = '';
  }
  parser.next();
  return {
    loc: parser.finishLoc(start),
    type: 'JSXIdentifier',
    name,
  };
}

// Parse namespaced identifier.
function parseJSXNamespacedName(
  parser: JSParser,
): JSXIdentifier | JSXNamespacedName {
  const start = parser.getPosition();

  const namespace = parseJSXIdentifier(parser);
  if (!parser.eat(tt.colon)) {
    return namespace;
  }

  const name = parseJSXIdentifier(parser);
  return {
    loc: parser.finishLoc(start),
    type: 'JSXNamespacedName',
    name,
    namespace,
  };
}

// Parses element name in any form - namespaced, member
// or single identifier.
function parseJSXElementName(parser: JSParser): JSXElement['name'] {
  const start = parser.getPosition();

  const namespacedName = parseJSXNamespacedName(parser);

  let node: JSXElement['name'];
  if (namespacedName.type === 'JSXIdentifier') {
    node = {
      ...namespacedName,
      type: 'JSXReferenceIdentifier',
    };
  } else {
    node = namespacedName;
  }

  while (parser.eat(tt.dot)) {
    const property = parseJSXIdentifier(parser);
    node = {
      loc: parser.finishLoc(start),
      type: 'JSXMemberExpression',
      object: node,
      property,
    };
  }

  return node;
}

// Parses any type of JSX attribute value.
function parseJSXAttributeValue(
  parser: JSParser,
): StringLiteral | JSXElement | JSXFragment | JSXExpressionContainer {
  let node;
  switch (parser.state.tokenType) {
    case tt.braceL:
      node = parseJSXExpressionContainer(parser);
      if (node.expression.type === 'JSXEmptyExpression') {
        parser.addDiagnostic({
          loc: node.loc,
          message:
            'JSX attributes must only be assigned a non-empty expression',
        });
      }
      return node;

    case tt.jsxTagStart:
      return parseJSXElement(parser);

    case tt.string:
      return parseStringLiteral(parser);

    default: {
      parser.addDiagnostic({
        message:
          'JSX value should be either an expression or a quoted JSX text',
      });
      return {
        type: 'StringLiteral',
        loc: parser.finishLoc(parser.getPosition()),
        value: '?',
      };
    }
  }
}

// JSXEmptyExpression is unique type since it doesn't actually parse anything,
// and so it should start at the end of last read token (left brace) and finish
// at the beginning of the next one (right brace).
function parseJSXEmptyExpression(parser: JSParser): JSXEmptyExpression {
  return {
    loc: parser.finishLoc(parser.state.lastEndPos),
    type: 'JSXEmptyExpression',
  };
}

// Parse JSX spread child
function parseJSXSpreadChild(parser: JSParser): JSXSpreadChild {
  const start = parser.getPosition();
  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'jsx spread child',
  );
  parser.expect(tt.ellipsis);
  const expression = parseExpression(parser, 'jsx spread child expression');
  parser.expectClosing(openContext);

  return {
    loc: parser.finishLoc(start),
    type: 'JSXSpreadChild',
    expression,
  };
}

// Parses JSX expression enclosed into curly brackets.
function parseJSXExpressionContainer(parser: JSParser): JSXExpressionContainer {
  const start = parser.getPosition();
  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'jsx expression container',
  );
  let expression;
  if (parser.match(tt.braceR)) {
    expression = parseJSXEmptyExpression(parser);
  } else {
    expression = parseExpression(parser, 'jsx inner expression container');
  }
  parser.expectClosing(openContext);
  return {
    loc: parser.finishLoc(start),
    type: 'JSXExpressionContainer',
    expression,
  };
}

// Parses following JSX attribute name-value pair.
function parseJSXAttribute(
  parser: JSParser,
): JSXSpreadAttribute | JSXAttribute {
  const start = parser.getPosition();

  if (parser.match(tt.braceL)) {
    const openContext = parser.expectOpening(
      tt.braceL,
      tt.braceR,
      'jsx attribute spread',
    );
    parser.expect(tt.ellipsis);
    const argument = parseMaybeAssign(parser, 'jsx attribute spread');
    parser.expectClosing(openContext);
    return {
      loc: parser.finishLoc(start),
      type: 'JSXSpreadAttribute',
      argument,
    };
  }

  const name = parseJSXNamespacedName(parser);
  const value = parser.eat(tt.eq) ? parseJSXAttributeValue(parser) : undefined;
  return {
    loc: parser.finishLoc(start),
    type: 'JSXAttribute',
    name,
    value,
  };
}

type OpeningElementDef = {
  name: undefined | JSXElement['name'];
  typeArguments: JSXElement['typeArguments'];
  attributes: JSXElement['attributes'];
  selfClosing: boolean;
  loc: SourceLocation;
};

// Parses JSX opening tag starting after "<".
function parseJSXOpeningElementAt(
  parser: JSParser,
  start: Position,
): OpeningElementDef {
  if (parser.match(tt.jsxTagEnd)) {
    parser.expect(tt.jsxTagEnd);
    return {
      typeArguments: undefined,
      name: undefined,
      loc: {
        filename: parser.filename,
        start,
        end: parser.getPosition(),
      },
      attributes: [],
      selfClosing: false,
    };
  }

  const attributes = [];
  const name = parseJSXElementName(parser);

  let typeArguments;
  if (parser.isRelational('<')) {
    if (!parser.isSyntaxEnabled('ts')) {
      parser.addDiagnostic({
        message: 'JSX element type arguments are only allowed in TS',
      });
    }

    typeArguments = parseTSTypeArguments(parser);
  }

  // We need to check for isRelational('>') here as the above type arguments parsing can put the tokenizer
  // into an unusual state for: <foo<bar>></foo>
  while (
    !parser.match(tt.slash) &&
    !parser.match(tt.jsxTagEnd) &&
    !parser.atEOF()
  ) {
    attributes.push(parseJSXAttribute(parser));
  }
  const selfClosing = parser.eat(tt.slash);
  if (!parser.eat(tt.jsxTagEnd)) {
    parser.addDiagnostic({
      message: 'Unclosed JSX element open',
    });
  }
  return {
    typeArguments,
    name,
    attributes,
    selfClosing,
    loc: parser.getLoc(name),
  };
}

// Parses JSX closing tag starting after "</".
function parseJSXClosingElementAt(
  parser: JSParser,
  start: Position,
): undefined | JSXElement['name'] {
  if (parser.match(tt.jsxTagEnd)) {
    if (!parser.eat(tt.jsxTagEnd)) {
      parser.addDiagnostic({
        message: 'Unclosed JSX fragment close',
      });
    }

    return undefined;
  }

  const name = parseJSXElementName(parser);

  if (!parser.eat(tt.jsxTagEnd)) {
    parser.addDiagnostic({
      message: 'Unclosed JSX element close',
    });
  }

  return name;
}

function getJSXOpenElementAdvice(
  parser: JSParser,
  def: OpeningElementDef,
): PartialDiagnosticAdvice {
  let message = 'Originated from this opening tag';

  if (def.name !== undefined) {
    message = `Originated from opening tag of <emphasis>${getQualifiedJSXName(
      def.name,
    )}</emphasis>`;
  }

  const {loc} = def;
  return [
    {
      type: 'log',
      category: 'info',
      message: message,
    },
    {
      type: 'frame',
      filename: parser.filename,
      start: loc.start,
      end: loc.end,
    },
  ];
}

function getJSXCloseElementAdvice(
  parser: JSParser,
  name: undefined | JSXElement['name'],
  loc: SourceLocation,
): PartialDiagnosticAdvice {
  let message;
  if (name === undefined) {
    message = 'But found a closing fragment instead';
  } else {
    message = `But found a closing tag of <emphasis>${getQualifiedJSXName(
      name,
    )}</emphasis> instead`;
  }

  return [
    {
      type: 'log',
      category: 'info',
      message: message,
    },
    {
      type: 'frame',
      filename: parser.filename,
      start: loc.start,
      end: loc.end,
    },
  ];
}

function recoverFromUnclosedJSX(parser: JSParser) {
  // jsxOpenTag
  parser.state.context.pop();
  parser.state.exprAllowed = false;
}

// Parses entire JSX element, including it"s opening tag
// (starting after "<"), attributes, contents and closing tag.
function parseJSXElementAt(
  parser: JSParser,
  start: Position,
): JSXElement | JSXFragment {
  const children = [];
  const openingDef = parseJSXOpeningElementAt(parser, start);

  let closingNameLoc: undefined | SourceLocation;
  let closingName: undefined | JSXElement['name'];

  // Parse children for unclosed elements
  if (openingDef.selfClosing === false) {
    contents: while (true) {
      switch (parser.state.tokenType) {
        case tt.jsxTagStart: {
          const start = parser.getPosition();
          parser.next();
          if (parser.eat(tt.slash)) {
            closingName = parseJSXClosingElementAt(parser, start);
            closingNameLoc = {
              filename: parser.filename,
              start,
              end: parser.getPosition(),
            };
            break contents;
          }
          children.push(parseJSXElementAt(parser, start));
          break;
        }

        case tt.jsxText:
          children.push(parseJSXText(parser));
          break;

        case tt.braceL:
          if (parser.lookaheadState().tokenType === tt.ellipsis) {
            children.push(parseJSXSpreadChild(parser));
          } else {
            children.push(parseJSXExpressionContainer(parser));
          }
          break;

        case tt.eof:
          parser.addDiagnostic({
            message: 'Unclosed JSX element',
            advice: getJSXOpenElementAdvice(parser, openingDef),
          });
          break contents;

        default:
          parser.addDiagnostic({
            message: 'Unknown JSX children start',
            advice: getJSXOpenElementAdvice(parser, openingDef),
          });

          // We don't need to do it for the tt.eof case above because nothing will ever be parsed after
          recoverFromUnclosedJSX(parser);

          break contents;
      }
    }

    // Unclosed element, would have produced an error above but we still want to produce a valid AST and avoid the below error conditions
    if (closingNameLoc === undefined) {
      closingName = openingDef.name;
      closingNameLoc = openingDef.loc;
    }

    // Fragment open, element close
    if (openingDef.name === undefined && closingName !== undefined) {
      parser.addDiagnostic({
        loc: openingDef.loc,
        message: `Expected JSX closing fragment tag`,
        advice: getJSXCloseElementAdvice(parser, closingName, closingNameLoc),
      });
    }

    // Element open, fragment close
    if (openingDef.name !== undefined && closingName === undefined) {
      parser.addDiagnostic({
        loc: openingDef.loc,
        message: `Expected a corresponding JSX closing tag for <emphasis>${getQualifiedJSXName(
          openingDef.name,
        )}</emphasis>`,
        advice: getJSXCloseElementAdvice(parser, closingName, closingNameLoc),
      });
    }

    // Validate element names: Element open, element close
    if (openingDef.name !== undefined && closingName !== undefined) {
      if (
        getQualifiedJSXName(closingName) !==
        getQualifiedJSXName(openingDef.name)
      ) {
        parser.addDiagnostic({
          loc: openingDef.loc,
          message: `Expected a corresponding JSX closing tag for <emphasis>${getQualifiedJSXName(
            openingDef.name,
          )}</emphasis>`,
          advice: getJSXCloseElementAdvice(parser, closingName, closingNameLoc),
        });
      }
    }
  }

  checkAccidentalFragment(parser);

  const openingName = openingDef.name;
  if (openingName === undefined) {
    return {
      loc: parser.finishLoc(start),
      type: 'JSXFragment',
      children,
    };
  } else {
    return {
      loc: parser.finishLoc(start),
      type: 'JSXElement',
      name: openingName,
      typeArguments: openingDef.typeArguments,
      attributes: openingDef.attributes,
      selfClosing: openingDef.selfClosing,
      children,
    };
  }
}

function checkAccidentalFragment(parser: JSParser) {
  if (parser.match(tt.relational) && parser.state.tokenValue === '<') {
    parser.addDiagnostic({
      message:
        'Adjacent JSX elements must be wrapped in an enclosing tag. ' +
        'Did you want a JSX fragment <>...</>?',
    });
  }
}

export function parseJSXText(parser: JSParser): JSXText {
  // No need to assert syntax here because we wont get that far as parseJSXElement would have already been called

  const start = parser.getPosition();
  const value = String(parser.state.tokenValue);
  parser.next();
  return {
    loc: parser.finishLoc(start),
    type: 'JSXText',
    value,
  };
}

// Parses entire JSX element from 'current position.
export function parseJSXElement(parser: JSParser): JSXElement | JSXFragment {
  // Only necessary here as this is the only JSX entry point
  if (!parser.isSyntaxEnabled('jsx')) {
    if (parser.isSyntaxEnabled('ts')) {
      parser.addDiagnostic({
        message: "JSX isn't allowed in regular TypeScript files",
        advice: [
          {
            type: 'log',
            category: 'info',
            message:
              'Change the file extension to <emphasis>.tsx</emphasis> to enable JSX support',
          },
        ],
      });
    } else {
      parser.addDiagnostic({
        message: "JSX syntax isn't enabled",
        advice: [
          {
            type: 'log',
            category: 'info',
            message:
              'Are you using <emphasis>TypeScript</emphasis>? Change the file extension to <emphasis>.tsx</emphasis>',
          },
          {
            type: 'log',
            category: 'info',
            message:
              'Are you using <emphasis>Flow</emphasis>? Add a <emphasis>@flow</emphasis> comment annotation to the top of the file',
          },
          {
            type: 'log',
            category: 'info',
            message:
              'Not using either? Change the file extension to <emphasis>.jsx</emphasis>',
          },
        ],
      });
    }
  }

  const start = parser.getPosition();
  parser.next();
  return parseJSXElementAt(parser, start);
}
