/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, SourceLocation} from '@romejs/parser-core';
import {JSParser, OpeningContext} from '../parser';
import {RegExpTokenValue, readRegexp, finishToken} from '../tokenizer/index';
import * as charCodes from '@romejs/string-charcodes';
import {types as tt} from '../tokenizer/types';
import {
  IndexTracker,
  createIndexTracker,
  isKeyword,
  isReservedWord,
  isStrictBindReservedWord,
  isStrictReservedWord,
} from '@romejs/js-parser-utils';
import {
  AnyExpression,
  Identifier,
  PrivateName,
  BlockStatement,
  FunctionHead,
  StaticPropertyKey,
  AnyObjectPropertyKey,
  AnyTargetBindingPattern,
  AnyNode,
  ComputedPropertyKey,
  SpreadElement,
  AmbiguousFlowTypeCastExpression,
  AnyObjectMember,
  BindingObjectPatternProperty,
  TSAsExpression,
  AssignmentOperator,
  BinaryOperator,
  LogicalOperator,
  LogicalExpression,
  BinaryExpression,
  UnaryOperator,
  UnaryExpression,
  UpdateOperator,
  UpdateExpression,
  CallExpression,
  AnyTypeArguments,
  ArrowFunctionExpression,
  TaggedTemplateExpression,
  MetaProperty,
  FunctionExpression,
  BooleanLiteral,
  AnyFlowPredicate,
  AnyPrimaryType,
  NewExpression,
  OptionalCallExpression,
  TemplateLiteral,
  TemplateElement,
  MemberExpression,
  ObjectExpression,
  SpreadProperty,
  BindingObjectPattern,
  BindingIdentifier,
  FlowFunctionTypeAnnotation,
  ObjectMethod,
  ClassMethod,
  ClassPrivateMethod,
  TSDeclareFunction,
  TSDeclareMethod,
  ObjectProperty,
  ReferenceIdentifier,
  AnyBindingPattern,
  AssignmentIdentifier,
  ObjectMethodKind,
  AwaitExpression,
  YieldExpression,
  NullLiteral,
  StringLiteral,
  BigIntLiteral,
  RegExpLiteral,
  ClassMethodKind,
  NumericLiteral,
  ImportCall,
  Super,
  DoExpression,
  ArrayExpression,
} from '@romejs/js-ast';
import {TypeAnnotationAndPredicate} from './type-systems';
import {types as tc} from '../tokenizer/context';
import {
  toReferencedListDeep,
  checkLVal,
  parseBlock,
  parseFunctionParams,
  parseMaybeDefault,
  parseSpread,
  toReferencedList,
  toReferencedListOptional,
  parseJSXElement,
  parseTypeAnnotationAndPredicate,
  parseFlowVariance,
  parseAsyncArrowWithFlowTypeParameters,
  checkCommaAfterRestFromSpread,
  parsePrimaryTypeAnnotation,
  maybeParseTypeParameters,
  isTypeSystemEnabled,
  parseTSTypeAssertion,
  tryTSNextParseConstantContext,
  tsCheckLiteralForConstantContext,
  tsNextThenParseType,
  parseTypeParameters,
  parseTypeCallArguments,
  raiseRestNotLast,
  toAssignmentPattern,
  toFunctionParamsBindingList,
  filterSpread,
  ToReferencedItem,
  parseClassExpression,
  parseFunctionExpression,
} from './index';
import {number0, number0Neg1, Number0, get0, inc} from '@romejs/ob1';
import {splitFunctionParams} from './statement';
import {createRegExpParser} from '@romejs/codec-js-regexp';
import {descriptions} from '@romejs/diagnostics';

// Check if property name clashes with already added.

// Object/class getters and setters are not allowed to clash —

// either with each other or with an init property — and in

// strict mode, init properties are also not allowed to be repeated.
export function checkPropClash(
  parser: JSParser,
  prop: AnyObjectMember | BindingObjectPatternProperty,
  props: Set<string>,
): void {
  if (prop.key.type === 'ComputedPropertyKey' || prop.type === 'ObjectMethod') {
    return undefined;
  }

  const key = prop.key.value;

  // We can only check these for collisions since they're statically known
  if (key.type !== 'Identifier' && key.type !== 'StringLiteral' && key.type !==
  'NumericLiteral') {
    return;
  }

  // It is either an Identifier or a String/NumericLiteral
  const name = key.type === 'Identifier' ? key.name : String(key.value);

  if (name === '__proto__') {
    if (props.has('proto')) {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.PROTO_PROP_REDEFINITION,
        loc: key.loc,
      });
    } else {
      props.add('proto');
    }
  }
}

// ### Expression parsing

// These nest, from the most general expression type at the top to

// 'atomic', nondivisible expression types at the bottom. Most of

// the functions will simply let the function (s) below them parse,

// and, *if* the syntactic construct they handle is present, wrap

// the AST node that the inner parser gave them in another node.

// Parse a full expression. The optional arguments are used to

// forbid the `in` operator (in for loops initialization expressions)

// and provide reference for storing '=' operator inside shorthand

// property assignment in contexts where both object expression

// and object pattern might appear (so it's possible to raise

// delayed syntax error at correct position).
export function parseExpression(
  parser: JSParser,
  context: ExpressionContext,
  noIn?: boolean,
  refShorthandDefaultPos?: IndexTracker,
): AnyExpression {
  const startPos = parser.state.startPos;
  const expr = parseMaybeAssign(parser, context, noIn, refShorthandDefaultPos);
  if (parser.match(tt.comma)) {
    let expressions: Array<AnyExpression> = [expr];
    while (parser.eat(tt.comma)) {
      expressions.push(parseMaybeAssign(
        parser,
        context,
        noIn,
        refShorthandDefaultPos,
      ));
    }

    expressions = filterSpread(parser, toReferencedList(parser, expressions));

    return parser.finishNode(startPos, {
      type: 'SequenceExpression',
      expressions,
    });
  }
  return expr;
}

// Parse an assignment expression. This includes applications of

// operators like `+=`.

// We need to support type parameter declarations for arrow functions. This

// is tricky. There are three situations we need to handle

//

// 1. This is either JSX or an arrow function. We'll try JSX first. If that

//    fails, we'll try an arrow function. If that fails, we'll throw the JSX

//    error.

// 2. This is an arrow function. We'll parse the type parameter declaration,

//    parse the rest, make sure the rest is an arrow function, and go from

//    there

// 3. This is neither. Just call the super method
export function parseMaybeAssign<T extends AnyNode = AnyExpression>(
  parser: JSParser,
  context: ExpressionContext,
  noIn?: boolean,
  refShorthandDefaultPos?: IndexTracker,
  afterLeftParse?: MaybeAssignAfterParse<T>,
  refNeedsArrowPos?: IndexTracker,
): AnyExpression | T {
  const branches = parser.createBranch<AnyExpression | T>();

  // Try parsing as JSX
  if ((parser.isRelational('<') || parser.match(tt.jsxTagStart)) &&
    parser.shouldTokenizeJSX()) {
    branches.add(() => {
      return _parseMaybeAssign(
        parser,
        context,
        noIn,
        refShorthandDefaultPos,
        afterLeftParse,
        refNeedsArrowPos,
      );
    }, {diagnosticsPriority: 1});

    // Remove `tc.j_expr` and `tc.j_oTag` from 'context added

    // by parsing `jsxTagStart` to stop the JSX plugin from

    // messing with the tokens
    const cLength = parser.state.context.length;
    if (parser.state.context[cLength - 1] === tc.jsxOpenTag) {
      parser.state.context.length -= 2;
    }
    finishToken(parser, tt.relational, '<');
  }

  // Try parsing as an arrow function with type parameters
  if (parser.isRelational('<')) {
    branches.add(() => {
      const start = parser.getPosition();
      const typeParameters = parseTypeParameters(parser);
      const arrowExpression = forwardNoArrowParamsConversionAt(
        parser,
        start,
        () =>
          _parseMaybeAssign<T>(
            parser,
            context,
            noIn,
            refShorthandDefaultPos,
            afterLeftParse,
            refNeedsArrowPos,
          ),
      );
      parser.resetStartLocationFromNode(arrowExpression, typeParameters);

      if (arrowExpression.type === 'ArrowFunctionExpression') {
        return {
          ...arrowExpression,
          typeParameters,
        };
      } else {
        parser.addDiagnostic({
          loc: typeParameters.loc,
          description: descriptions.JS_PARSER.EXPECTED_ARROW_AFTER_TYPE_PARAMS,
        });
        return toReferenceIdentifier(parser, parser.createUnknownIdentifier(
          'type params without arrow function',
        ));
      }
    });
  }

  branches.add(() => {
    return _parseMaybeAssign<T>(
      parser,
      context,
      noIn,
      refShorthandDefaultPos,
      afterLeftParse,
      refNeedsArrowPos,
    );
  });

  // Pick the branch with the least amount of errors
  return branches.pick();
}

type MaybeAssignAfterParse<T> = (parser: JSParser, left: AnyExpression, startPos: Position) => T;

function _parseMaybeAssign<T extends AnyNode>(
  parser: JSParser,
  context: ExpressionContext,
  noIn?: boolean,
  refShorthandDefaultPos?: IndexTracker,
  afterLeftParse?: MaybeAssignAfterParse<T>,
  refNeedsArrowPos?: IndexTracker,
): AnyExpression | T {
  const startPos = parser.state.startPos;

  if (parser.isContextual('yield')) {
    if (parser.inScope('GENERATOR')) {
      let left: T | AnyExpression = parseYield(parser, noIn);
      if (afterLeftParse) {
        left = afterLeftParse(parser, left, startPos);
      }
      return left;
    } else {
      // The tokenizer will assume an expression is allowed after

      // `yield`, but this isn't that kind of yield
      parser.state.exprAllowed = false;
    }
  }

  const oldCommaAfterSpreadAt = parser.state.commaAfterSpreadAt;
  parser.state.commaAfterSpreadAt = number0Neg1;

  let failOnShorthandAssign;
  if (refShorthandDefaultPos) {
    failOnShorthandAssign = false;
  } else {
    refShorthandDefaultPos = createIndexTracker();
    failOnShorthandAssign = true;
  }

  if (parser.match(tt.parenL) || parser.match(tt.name)) {
    parser.state.potentialArrowAt = parser.state.startPos.index;
  }

  let left: AnyExpression | T = parseMaybeConditional(
    parser,
    context,
    noIn,
    refShorthandDefaultPos,
    refNeedsArrowPos,
  );
  if (afterLeftParse) {
    left = afterLeftParse(parser, left, startPos);
  }

  if (parser.state.tokenType.isAssign) {
    const operator = (String(parser.state.tokenValue) as AssignmentOperator);
    const leftPatt = toAssignmentPattern(parser, left, 'assignment expression');

    // reset because shorthand default was used correctly
    refShorthandDefaultPos.index = number0;

    checkLVal(parser, leftPatt, undefined, undefined, 'assignment expression');

    // We should never get patterns here...?

    //if (left.type === 'BindingArrayPattern' || left.type === 'BindingObjectPattern') {
    //  checkCommaAfterRestFromSpread(parser);

    //}
    parser.state.commaAfterSpreadAt = oldCommaAfterSpreadAt;

    parser.next();
    const right = parseMaybeAssign(parser, 'assignment right', noIn);
    return parser.finishNode(startPos, {
      type: 'AssignmentExpression',
      operator,
      left: leftPatt,
      right,
    });
  } else if (failOnShorthandAssign && get0(refShorthandDefaultPos.index) > 0) {
    parser.unexpectedToken(parser.getPositionFromIndex(
      refShorthandDefaultPos.index,
    ));
  }

  parser.state.commaAfterSpreadAt = oldCommaAfterSpreadAt;

  return left;
}

export function parseMaybeConditional(
  parser: JSParser,
  context: ExpressionContext,
  noIn: undefined | boolean,
  refShorthandDefaultPos: IndexTracker,
  refNeedsArrowPos?: IndexTracker,
): AnyExpression {
  const startPos = parser.state.startPos;
  const potentialArrowAt = parser.state.potentialArrowAt;
  const expr = parseExpressionOps(parser, context, noIn, refShorthandDefaultPos);

  if (expr.type === 'ArrowFunctionExpression' &&
    parser.getLoc(expr).start.index === potentialArrowAt) {
    return expr;
  }

  if (refShorthandDefaultPos && get0(refShorthandDefaultPos.index) > 0) {
    return expr;
  }

  return parseConditional(parser, expr, noIn, startPos, refNeedsArrowPos);
}

export function tryParseConditionalConsequent(
  parser: JSParser,
): {
  consequent: AnyExpression;
  failed: boolean;
} {
  const brancher = parser.createBranch<{
    consequent: AnyExpression;
    failed: boolean;
  }>();

  brancher.add(() => {
    parser.state.noArrowParamsConversionAt.push(parser.state.startPos.index);
    const consequent = parseMaybeAssign(parser, 'conditional consequent');
    parser.state.noArrowParamsConversionAt.pop();
    return {
      consequent,
      failed: !parser.match(tt.colon),
    };
  });

  return brancher.pick();
}

export function parseConditional(
  parser: JSParser,
  expr: AnyExpression,
  noIn: undefined | boolean,
  startPos: Position,
  refNeedsArrowPos?: IndexTracker,
): AnyExpression {
  if (!parser.match(tt.question)) {
    return expr;
  }

  // This is to handle a case like this: const foo = (foo?: bar) => {};

  // We'll be called due to the `?`, and we should mark ourselves as an

  // expected arrow function if parsing as a regular conditional fails
  if (refNeedsArrowPos) {
    const branch = parser.createBranch<AnyExpression>();

    branch.add(() => _parseConditional(parser, expr, noIn, startPos), {
      maxNewDiagnostics: 0,
    });

    if (branch.hasBranch()) {
      return branch.pick();
    } else {
      refNeedsArrowPos.index = parser.state.startPos.index;
      return expr;
    }
  }

  parser.expect(tt.question);
  const originalNoArrowAt = parser.state.noArrowAt;
  let {consequent} = tryParseConditionalConsequent(parser);
  parser.state.noArrowAt = originalNoArrowAt;

  if (!parser.eat(tt.colon)) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.MISSING_CONDITIONAL_SEPARATOR,
    });
  }

  const alternate = forwardNoArrowParamsConversionAt(parser, startPos, () =>
    parseMaybeAssign(
      parser,
      'conditional alternate',
      noIn,
      undefined,
      undefined,
      undefined,
    )
  );

  return parser.finishNode(startPos, {
    type: 'ConditionalExpression',
    test: expr,
    consequent,
    alternate,
  });
}

export function forwardNoArrowParamsConversionAt<T>(
  parser: JSParser,
  start: Position,
  parse: () => T,
): T {
  if (parser.state.noArrowParamsConversionAt.includes(start.index)) {
    let result: T;
    parser.state.noArrowParamsConversionAt.push(parser.state.startPos.index);
    result = parse();
    parser.state.noArrowParamsConversionAt.pop();
    return result;
  } else {
    return parse();
  }
}

function _parseConditional(
  parser: JSParser,
  expr: AnyExpression,
  noIn: undefined | boolean,
  startPos: Position,
): AnyExpression {
  if (parser.eat(tt.question)) {
    const test = expr;
    const consequent = parseMaybeAssign(parser, 'conditional consequent');
    parser.expect(tt.colon);
    const alternate = parseMaybeAssign(parser, 'conditional alternate', noIn);
    return parser.finishNode(startPos, {
      type: 'ConditionalExpression',
      test,
      consequent,
      alternate,
    });
  }
  return expr;
}

export function parseExpressionOps(
  parser: JSParser,
  context: ExpressionContext,
  noIn: undefined | boolean,
  refShorthandDefaultPos: IndexTracker,
): AnyExpression {
  const startPos = parser.state.startPos;
  const potentialArrowAt = parser.state.potentialArrowAt;
  const expr = parseMaybeUnary(parser, context, refShorthandDefaultPos);

  if (expr.type === 'ArrowFunctionExpression' &&
    parser.getLoc(expr).start.index === potentialArrowAt) {
    return expr;
  }
  if (refShorthandDefaultPos && get0(refShorthandDefaultPos.index) > 0) {
    return expr;
  }

  return parseExpressionOp(parser, context, expr, startPos, -1, noIn);
}

// Parse binary operators with the operator precedence parsing

// algorithm. `left` is the left-hand side of the operator.

// `minPrec` provides context that allows the function to stop and

// defer further parser to one of its callers when it encounters an

// operator that has a lower precedence than the set it is parsing.
export function parseExpressionOp(
  parser: JSParser,
  context: ExpressionContext,
  left: AnyExpression,
  leftStartPos: Position,
  minPrec: number,
  noIn: boolean = false,
): AnyExpression {
  if (tt._in.getBinop() > minPrec && !parser.hasPrecedingLineBreak() &&
    parser.isContextual('as')) {
    const _const = tryTSNextParseConstantContext(parser);

    let typeAnnotation;
    if (_const) {
      tsCheckLiteralForConstantContext(parser, left);
      typeAnnotation = _const;
    } else {
      typeAnnotation = tsNextThenParseType(parser);
    }

    const node: TSAsExpression = parser.finishNode(leftStartPos, {
      type: 'TSAsExpression',
      typeAnnotation,
      expression: left,
    });

    return parseExpressionOp(parser, context, node, leftStartPos, minPrec, noIn);
  }

  const prec = parser.state.tokenType.binop;
  if (prec !== undefined && (!noIn || !parser.match(tt._in))) {
    if (prec > minPrec) {
      const operator = (String(parser.state.tokenValue) as
        | BinaryOperator
        | LogicalOperator);

      if (operator === '**' && left.type === 'UnaryExpression' &&
        !parser.isParenthesized(left)) {
        parser.addDiagnostic({
          loc: left.argument.loc,
          description: descriptions.JS_PARSER.WRAP_EXPONENTIATION,
        });
      }

      const op = parser.state.tokenType;
      parser.next();

      const startPos = parser.state.startPos;

      const right = parseExpressionOp(parser, context, parseMaybeUnary(
        parser,
        context,
      ), startPos, op.rightAssociative ? prec - 1 : prec, noIn);

      let node: LogicalExpression | BinaryExpression;
      if (operator === '||' || operator === '&&' || operator === '??') {
        node = parser.finishNode(leftStartPos, {
          type: 'LogicalExpression',
          left,
          right,
          operator,
        });
      } else {
        node = parser.finishNode(leftStartPos, {
          type: 'BinaryExpression',
          left,
          right,
          operator,
        });
      }

      return parseExpressionOp(
        parser,
        context,
        node,
        leftStartPos,
        minPrec,
        noIn,
      );
    }
  }

  return left;
}

// Parse unary operators, both prefix and postfix.
export function parseMaybeUnary(
  parser: JSParser,
  context: ExpressionContext,
  refShorthandDefaultPos?: IndexTracker,
): AnyExpression {
  if (parser.isSyntaxEnabled('ts') && !parser.isSyntaxEnabled('jsx') &&
    parser.isRelational('<')) {
    return parseTSTypeAssertion(parser);
  }

  if (parser.isContextual('await') && parser.inScope('ASYNC')) {
    return parseAwait(parser);
  }

  if (parser.state.tokenType.prefix) {
    const start = parser.getPosition();
    const update = parser.match(tt.incDec);
    const operator = (String(parser.state.tokenValue) as
      | UnaryOperator
      | UpdateOperator);
    const prefix = true;

    parser.next();

    const argument = parseMaybeUnary(parser, context);

    if (refShorthandDefaultPos && get0(refShorthandDefaultPos.index) > 0) {
      parser.unexpectedToken(parser.getPositionFromIndex(
        refShorthandDefaultPos.index,
      ));
    }

    if (update) {
      checkLVal(parser, argument, undefined, undefined, 'prefix operation');
    } else if (parser.inScope('STRICT') && operator === 'delete') {
      if (argument.type === 'ReferenceIdentifier') {
        parser.addDiagnostic({
          loc: argument.loc,
          description: descriptions.JS_PARSER.DELETE_LOCAL_VARIABLE_IN_STRICT,
        });
      } else if (argument.type === 'MemberExpression' &&
        argument.property.value.type === 'PrivateName') {
        parser.addDiagnostic({
          loc: argument.property.loc,
          description: descriptions.JS_PARSER.DELETE_PRIVATE_FIELD,
        });
      }
    }

    let node: UpdateExpression | UnaryExpression;
    if (update) {
      if (operator !== '++' && operator !== '--') {
        throw new Error('Expected ++/-- operator only for UpdateExpression');
      }

      node = parser.finishNode(start, {
        type: 'UpdateExpression',
        argument,
        operator,
        prefix,
      });
    } else {
      if (operator === '++' || operator === '--') {
        throw new Error('BinaryExpression cannot have ++/-- operator');
      }

      node = parser.finishNode(start, {
        type: 'UnaryExpression',
        argument,
        operator,
        prefix,
      });
    }

    return node;
  }

  const startPos = parser.state.startPos;

  let expr = parseExpressionWithPossibleSubscripts(
    parser,
    context,
    refShorthandDefaultPos,
  );
  if (refShorthandDefaultPos && get0(refShorthandDefaultPos.index) > 0) {
    return expr;
  }

  while (parser.state.tokenType.postfix && !parser.canInsertSemicolon()) {
    const operator = (String(parser.state.tokenValue) as UpdateOperator);
    checkLVal(parser, expr, undefined, undefined, 'postfix operation');
    parser.next();

    const updateNode: UpdateExpression = parser.finishNode(startPos, {
      type: 'UpdateExpression',
      operator,
      prefix: false,
      argument: expr,
    });
    expr = updateNode;
  }

  return expr;
}

// Parse call, dot, and `[]`-subscript expressions.
export function parseExpressionWithPossibleSubscripts(
  parser: JSParser,
  context: ExpressionContext,
  refShorthandDefaultPos?: IndexTracker,
): AnyExpression {
  const startPos = parser.state.startPos;
  const potentialArrowAt = parser.state.potentialArrowAt;
  const expr = parseExpressionAtom(parser, context, refShorthandDefaultPos);

  if (expr.type === 'ArrowFunctionExpression' &&
    parser.getLoc(expr).start.index === potentialArrowAt) {
    return expr;
  }

  if (refShorthandDefaultPos && get0(refShorthandDefaultPos.index) > 0) {
    return expr;
  }

  return parseSubscripts(parser, expr, startPos);
}

export function parseSubscripts(
  parser: JSParser,
  base: AnyExpression,
  startPos: Position,
  noCalls?: boolean,
): AnyExpression {
  const maybeAsyncArrow = atPossibleAsync(parser, base);

  if (base.type === 'ReferenceIdentifier' && base.name === 'async' &&
    parser.state.noArrowAt.includes(startPos.index)) {
    const openContext = parser.expectOpening(
      tt.parenL,
      tt.parenR,
      'call arguments',
    );
    const callee = base;
    const {args} = parseCallExpressionArguments(parser, openContext, false);
    base = parser.finishNode(startPos, {
      type: 'CallExpression',
      callee,
      arguments: args,
    });
  } else if (base.type === 'ReferenceIdentifier' && base.name === 'async' &&
    parser.isRelational('<')) {
    const branch = parser.createBranch<AnyExpression>();
    branch.add(() => parseAsyncArrowWithFlowTypeParameters(parser, startPos));
    branch.add(() =>
      parseExpressionSubscriptsRecursively(
        parser,
        base,
        startPos,
        noCalls,
        maybeAsyncArrow,
      )
    );
    return branch.pick();
  }

  return parseExpressionSubscriptsRecursively(
    parser,
    base,
    startPos,
    noCalls,
    maybeAsyncArrow,
  );
}

function parseExpressionSubscriptsRecursively(
  parser: JSParser,
  base: AnyExpression,
  startPos: Position,
  noCalls: undefined | boolean,
  maybeAsyncArrow: boolean,
): AnyExpression {
  const state: ParseSubscriptState = {
    optionalChainMember: false,
    stop: false,
  };
  do {
    base = parseExpressionSubscript(
      parser,
      base,
      startPos,
      noCalls,
      state,
      maybeAsyncArrow,
    );
  } while (!state.stop);
  return base;
}

type ParseSubscriptState = {
  stop: boolean;
  optionalChainMember: boolean;
};

export function parseExpressionSubscript(
  parser: JSParser,
  base: AnyExpression,
  startPos: Position,
  noCalls: boolean = false,
  state: ParseSubscriptState,
  maybeAsyncArrow: boolean,
): AnyExpression {
  if (!parser.hasPrecedingLineBreak() && parser.match(tt.bang)) {
    parser.state.exprAllowed = false;
    parser.next();

    return parser.finishNode(startPos, {
      type: 'TSNonNullExpression',
      expression: base,
    });
  }

  if (parser.match(tt.questionDot)) {
    state.optionalChainMember = true;

    if (noCalls && parser.lookaheadState().tokenType == tt.parenL) {
      state.stop = true;
      return base;
    }

    parser.next();

    // eg: o.m?.<T>(e);
    if (parser.isRelational('<')) {
      if (noCalls) {
        state.stop = true;
        return base;
      }

      const callee = base;
      const typeArguments = parseTypeCallArguments(parser);
      const openContext = parser.expectOpening(
        tt.parenL,
        tt.parenR,
        'call arguments',
      );
      const {args} = parseCallExpressionArguments(parser, openContext, false);
      return parser.finishNode(startPos, {
        type: 'OptionalCallExpression',
        arguments: args,
        callee,
        typeArguments,
      });
    }

    if (parser.match(tt.bracketL)) {
      const propStart = parser.getPosition();
      const openContext = parser.expectOpening(
        tt.bracketL,
        tt.bracketR,
        'computed property',
      );
      const object = base;
      const property = parseExpression(
        parser,
        'optional member expression property',
      );
      parser.expectClosing(openContext);
      return parser.finishNode(startPos, {
        type: 'MemberExpression',
        object,
        property: parser.finishNode(propStart, {
          type: 'ComputedMemberProperty',
          optional: true,
          value: property,
        }),
      });
    }

    if (parser.match(tt.parenL)) {
      const openContext = parser.expectOpening(
        tt.parenL,
        tt.parenR,
        'call arguments',
      );
      const callee = base;
      const {args} = parseCallExpressionArguments(parser, openContext, false);

      return parser.finishNode(startPos, {
        type: 'OptionalCallExpression',
        callee,
        arguments: args,
      });
    }

    const object = base;
    const property = parseIdentifier(parser, true);

    return parser.finishNode(startPos, {
      type: 'MemberExpression',
      object,
      property: {
        type: 'StaticMemberProperty',
        loc: property.loc,
        optional: true,
        value: property,
      },
    });
  }

  if (parser.eat(tt.dot)) {
    const object = base;
    const property = parseMaybePrivateName(parser);

    return parser.finishNode(startPos, {
      type: 'MemberExpression',
      object,
      property: {
        type: 'StaticMemberProperty',
        loc: property.loc,
        value: property,
      },
    });
  }

  if (parser.match(tt.bracketL)) {
    const propStart = parser.getPosition();
    const openContext = parser.expectOpening(
      tt.bracketL,
      tt.bracketR,
      'computed property',
    );
    const object = base;
    const property = parseExpression(
      parser,
      'member expression computed property',
    );
    parser.expectClosing(openContext);

    return parser.finishNode(startPos, {
      type: 'MemberExpression',
      object,
      property: parser.finishNode(propStart, {
        type: 'ComputedMemberProperty',
        value: property,
      }),
    });
  }

  // Supports: foo<Foo>(); and foo<Foo>``;
  if (parser.isRelational('<') && isTypeSystemEnabled(parser)) {
    const possibleCallExpression = parser.tryBranch(() => {
      const typeArguments = parseTypeCallArguments(parser);

      if (!noCalls && parser.match(tt.parenL)) {
        const openContext = parser.expectOpening(
          tt.parenL,
          tt.parenR,
          'call arguments',
        );
        const {args} = parseCallExpressionArguments(parser, openContext, false);
        const node: CallExpression = parser.finishNode(startPos, {
          type: 'CallExpression',
          arguments: args,
          callee: base,
          typeArguments,
        });
        return node;
      }

      if (parser.match(tt.backQuote)) {
        return parseTaggedTemplateExpression(
          parser,
          startPos,
          base,
          state,
          typeArguments,
        );
      }
    });

    if (possibleCallExpression !== undefined) {
      return possibleCallExpression;
    }
  }

  if (!noCalls && parser.match(tt.parenL)) {
    const oldMaybeInArrowParameters = parser.state.maybeInArrowParameters;
    const oldYieldPos = parser.state.yieldPos;
    const oldAwaitPos = parser.state.awaitPos;
    parser.state.maybeInArrowParameters = true;
    parser.state.yieldPos = number0;
    parser.state.awaitPos = number0;

    const openContext = parser.expectOpening(
      tt.parenL,
      tt.parenR,
      'call arguments',
    );
    const callee = base;

    const oldCommaAfterSpreadAt = parser.state.commaAfterSpreadAt;
    parser.state.commaAfterSpreadAt = number0Neg1;

    let {args, params} = parseCallExpressionArguments(
      parser,
      openContext,
      maybeAsyncArrow,
    );

    if (maybeAsyncArrow && shouldParseAsyncArrow(parser)) {
      state.stop = true;

      checkCommaAfterRestFromSpread(parser);

      const node = parseAsyncArrowFromCallExpression(
        parser,
        startPos,
        params === undefined ? args : params,
      );
      checkYieldAwaitInDefaultParams(parser);
      parser.state.yieldPos = oldYieldPos;
      parser.state.awaitPos = oldAwaitPos;
      return node;
    } else {
      args = toReferencedListDeep(parser, args);

      // We keep the old value if it isn't null, for cases like

      //   (x = async(yield)) => {}
      parser.state.yieldPos = oldYieldPos || parser.state.yieldPos;
      parser.state.awaitPos = oldAwaitPos || parser.state.awaitPos;
    }

    parser.state.maybeInArrowParameters = oldMaybeInArrowParameters;
    parser.state.commaAfterSpreadAt = oldCommaAfterSpreadAt;

    return parser.finishNode(startPos, {
      type: 'CallExpression',
      callee,
      arguments: args,
    });
  }

  if (parser.match(tt.backQuote)) {
    return parseTaggedTemplateExpression(parser, startPos, base, state);
  }

  state.stop = true;
  return base;
}

export function parseTaggedTemplateExpression(
  parser: JSParser,
  startPos: Position,
  tag: AnyExpression,
  state: ParseSubscriptState,
  typeArguments?: AnyTypeArguments,
): TaggedTemplateExpression {
  if (state.optionalChainMember) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.TAGGED_TEMPLATE_IN_OPTIONAL_CHAIN,
    });
  }

  const quasi = parseTemplate(parser, true);
  return parser.finishNode(startPos, {
    type: 'TaggedTemplateExpression',
    tag,
    quasi,
    typeArguments,
  });
}

export function checkYieldAwaitInDefaultParams(parser: JSParser) {
  if (get0(parser.state.yieldPos) > 0 && (parser.state.awaitPos === number0 ||
  parser.state.yieldPos < parser.state.awaitPos)) {
    parser.addDiagnostic({
      index: parser.state.yieldPos,
      description: descriptions.JS_PARSER.YIELD_IN_GENERATOR_PARAMS,
    });
  }

  if (get0(parser.state.awaitPos) > 0) {
    parser.addDiagnostic({
      index: parser.state.awaitPos,
      description: descriptions.JS_PARSER.AWAIT_IN_ASYNC_PARAMS,
    });
  }
}

export function atPossibleAsync(parser: JSParser, base: AnyExpression): boolean {
  const loc = parser.getLoc(base);
  return base.type === 'ReferenceIdentifier' && base.name === 'async' &&
    parser.state.lastEndPos.index === loc.end.index &&
    !parser.canInsertSemicolon() && parser.getRawInput(
    loc.start.index,
    loc.end.index,
  ) === 'async';
}

export function parseCallExpressionArguments(
  parser: JSParser,
  openContext: OpeningContext,
  possibleAsyncArrow: boolean,
  refTrailingCommaPos?: IndexTracker,
): {
  args: CallExpression['arguments'];
  params:
    | undefined
    | Array<AnyExpression | SpreadElement | AmbiguousFlowTypeCastExpression>;
} {
  let callArgs: CallExpression['arguments'] = [];
  let funcParams: Array<
    | AnyExpression
    | SpreadElement
    | AmbiguousFlowTypeCastExpression> = [];

  let innerParenStart;
  let first = true;

  let forceAsyncArrow = false;

  while (true) {
    if (parser.match(openContext.close) || parser.match(tt.eof)) {
      parser.expectClosing(openContext);
      break;
    }

    if (first) {
      first = false;
    } else {
      if (!parser.expect(tt.comma)) {
        break;
      }

      if (parser.eat(openContext.close)) {
        break;
      }
    }

    // we need to make sure that if this is an async arrow functions, that we don't allow inner parens inside the params
    if (parser.match(tt.parenL) && !innerParenStart) {
      innerParenStart = parser.state.startPos;
    }

    const elt = parseCallArgument(
      parser,
      'call expression argument',
      false,
      possibleAsyncArrow ? createIndexTracker() : undefined,
      possibleAsyncArrow ? createIndexTracker() : undefined,
      possibleAsyncArrow ? refTrailingCommaPos : undefined,
    );
    if (elt === undefined) {
      throw new Error('Expected element');
    }

    if (elt.type === 'AmbiguousFlowTypeCastExpression') {
      if (possibleAsyncArrow) {
        // Definitely needs to be an arrow
        forceAsyncArrow = true;

        if (callArgs.length > 0) {
          funcParams = callArgs.slice();
          callArgs = [];
        }

        funcParams.push(elt);
      } else {
        parser.addDiagnostic({
          description: descriptions.JS_PARSER.CONFUSING_CALL_ARGUMENT,
          loc: elt.loc,
        });
      }
      continue;
    }

    if (funcParams.length > 0) {
      funcParams.push(elt);
    } else {
      callArgs.push(elt);
    }
  }

  if (forceAsyncArrow && !shouldParseAsyncArrow(parser)) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.EXPECTED_ARROW_AFTER_ASYNC_TYPE_PARAMS,
    });
  }

  // we found an async arrow function so let's not allow any inner parens
  if (possibleAsyncArrow && innerParenStart !== undefined &&
    shouldParseAsyncArrow(parser)) {
    parser.addDiagnostic({
      start: innerParenStart,
      description: descriptions.JS_PARSER.PARENTHESIZED_FUNCTION_PARAMS,
    });
  }

  return {
    args: callArgs,
    params: funcParams.length === 0 ? undefined : funcParams,
  };
}

export function shouldParseAsyncArrow(parser: JSParser): boolean {
  return parser.match(tt.colon) || parser.match(tt.arrow) &&
    !parser.canInsertSemicolon();
}

export function parseAsyncArrowFromCallExpression(
  parser: JSParser,
  start: Position,
  args: Array<AnyExpression | SpreadElement | AmbiguousFlowTypeCastExpression>,
): ArrowFunctionExpression {
  let returnType;

  if (parser.match(tt.colon)) {
    const oldNoAnonFunctionType = parser.state.noAnonFunctionType;
    parser.state.noAnonFunctionType = true;
    returnType = parsePrimaryTypeAnnotation(parser);
    parser.state.noAnonFunctionType = oldNoAnonFunctionType;
  }

  const oldYield = parser.state.yieldInPossibleArrowParameters;
  parser.state.yieldInPossibleArrowParameters = undefined;
  parser.expect(tt.arrow);
  const node = parseArrowExpression(parser, start, {
    assignmentList: args,
  }, true);
  parser.state.yieldInPossibleArrowParameters = oldYield;
  return {
    ...node,
    head: {
      ...node.head,
      returnType,
    },
  };
}

// Parse a no-call expression (like argument of `new` or `::` operators).
export function parseNoCallExpr(
  parser: JSParser,
  context: ExpressionContext,
): AnyExpression {
  const startPos = parser.state.startPos;
  return parseSubscripts(
    parser,
    parseExpressionAtom(parser, context),
    startPos,
    true,
  );
}

type ExpressionContext =
  | 'await argument'
  | 'export default declaration'
  | 'export from'
  | 'import source'
  | 'return argument'
  | 'switch discriminant'
  | 'case test'
  | 'throw argument'
  | 'flow object property key'
  | 'flow declare module id'
  | 'flow declared predicate'
  | 'class private property'
  | 'class property value'
  | 'assignment right'
  | 'class heritage'
  | 'new callee'
  | 'var init'
  | 'for right'
  | 'for update'
  | 'for test'
  | 'for init'
  | 'with object'
  | 'while test'
  | 'do test'
  | 'if test'
  | 'conditional consequent'
  | 'conditional alternate'
  | 'class private property value'
  | 'statement expression'
  | 'class private property value'
  | 'optional member expression property'
  | 'member expression computed property'
  | 'call expression argument'
  | 'new expression argument'
  | 'template expression value'
  | 'object property value'
  | 'property name'
  | 'function body'
  | 'yield argument'
  | 'array element'
  | 'spread argument'
  | 'assignment pattern right'
  | 'ts export assignment'
  | 'ts external module reference expression'
  | 'ts enum member initializer'
  | 'ts enum member id'
  | 'ts type assertion'
  | 'ts literal type'
  | 'ts import argument'
  | 'jsx inner expression container'
  | 'jsx attribute value'
  | 'jsx spread child expression'
  | 'jsx attribute spread'
  | 'jsx text';

// Parse an atomic expression — either a single token that is an

// expression, an expression started by a keyword like `function` or

// `new`, or an expression wrapped in punctuation like `()`, `[]`,

// or `{}`.
export function parseExpressionAtom(
  parser: JSParser,
  context: ExpressionContext,
  refShorthandDefaultPos?: IndexTracker,
): AnyExpression {
  // If a division operator appears in an expression position, the

  // tokenizer got confused, and we force it to read a regexp instead.
  if (parser.state.tokenType === tt.slash) {
    readRegexp(parser);
  }

  const canBeArrow = parser.state.potentialArrowAt ===
  parser.state.startPos.index;

  // We don't want to match <! as it's the start of a HTML comment
  if (parser.isRelational('<') && parser.input.charCodeAt(get0(
    parser.state.index,
  )) !== charCodes.exclamationMark) {
    // In case we encounter an lt token here it will always be the start of

    // jsx as the lt sign is not allowed in places that expect an expression
    finishToken(parser, tt.jsxTagStart);
    return parseJSXElement(parser);
  }

  switch (parser.state.tokenType) {
    case tt.jsxTagStart:
      return parseJSXElement(parser);

    case tt._super:
      return parseSuper(parser);

    case tt._import:
      return parseImportOrMetaProperty(parser);

    case tt._this:
      {
        const start = parser.getPosition();
        parser.next();
        return parser.finishNode(start, {type: 'ThisExpression'});
      }

    case tt.name:
      {
        const start = parser.getPosition();
        const containsEsc = parser.state.escapePosition !== undefined;
        const id = parseIdentifier(parser);

        if (!containsEsc && id.name === 'async' && parser.match(tt._function) &&
          !parser.canInsertSemicolon()) {
          parser.next();
          return parseFunctionExpression(parser, start, true);
        }

        if (canBeArrow && !containsEsc && id.name === 'async' && parser.match(
          tt.name,
        )) {
          const oldYield = parser.state.yieldInPossibleArrowParameters;
          parser.state.yieldInPossibleArrowParameters = undefined;
          const params = [parseReferenceIdentifier(parser)];
          parser.expect(tt.arrow);
          // let foo = bar => {};
          const node = parseArrowExpression(parser, start, {
            assignmentList: params,
          }, true);
          parser.state.yieldInPossibleArrowParameters = oldYield;
          return node;
        }

        if (canBeArrow && !parser.canInsertSemicolon() && parser.eat(tt.arrow)) {
          const oldYield = parser.state.yieldInPossibleArrowParameters;
          parser.state.yieldInPossibleArrowParameters = undefined;
          const node = parseArrowExpression(parser, start, {
            assignmentList: [toReferenceIdentifier(parser, id)],
          });
          parser.state.yieldInPossibleArrowParameters = oldYield;
          return node;
        }

        return toReferenceIdentifier(parser, id);
      }

    case tt._do:
      return parseDoExpression(parser);

    case tt.regexp:
      return parseRegExpLiteral(parser);

    case tt.num:
      return parseNumericLiteral(parser);

    case tt.bigint:
      return parseBigIntLiteral(parser);

    case tt.string:
      return parseStringLiteral(parser);

    case tt._null:
      return parseNullLiteral(parser);

    case tt._true:
    case tt._false:
      return parseBooleanLiteral(parser);

    case tt.parenL:
      return parseParenAndDistinguishExpression(parser, context, canBeArrow);

    case tt.bracketL:
      return parseArrayExpression(parser, refShorthandDefaultPos);

    case tt.braceL:
      return parseObjectExpression(parser, refShorthandDefaultPos);

    case tt._function:
      return parseFunctionExpressionOrMetaProperty(parser);

    case tt._class:
      {
        const start = parser.getPosition();
        return parseClassExpression(parser, start);
      }

    case tt._new:
      return parseNew(parser);

    case tt.backQuote:
      return parseTemplate(parser, false);

    default:
      {
        const start = parser.getPosition();
        parser.addDiagnostic({
          description: descriptions.JS_PARSER.UNKNOWN_EXPRESSION_ATOM_START(
            context,
          ),
        });
        parser.next();
        return toReferenceIdentifier(parser, parser.createUnknownIdentifier(
          context,
          start,
        ));
      }
  }
}

export function parseBooleanLiteral(parser: JSParser): BooleanLiteral {
  const start = parser.getPosition();
  const value = parser.match(tt._true);
  parser.next();
  return parser.finishNode(start, {
    type: 'BooleanLiteral',
    value,
  });
}

export function parseMaybePrivateName(parser: JSParser): PrivateName | Identifier {
  const isPrivate = parser.match(tt.hash);

  if (isPrivate) {
    const start = parser.getPosition();
    parser.next();
    parser.assertNoSpace(descriptions.JS_PARSER.SPACE_BETWEEN_PRIVATE_HASH);
    const id = parseIdentifier(parser, true);
    return parser.finishNode(start, {
      type: 'PrivateName',
      id,
    });
  } else {
    return parseIdentifier(parser, true);
  }
}

export function parseFunctionExpressionOrMetaProperty(
  parser: JSParser,
): FunctionExpression | MetaProperty {
  const start = parser.getPosition();
  parser.next();

  // We do not do parseIdentifier here because when parseFunctionExpressionOrMetaProperty

  // is called we already know that the current token is a "name" with the value "function"

  // This will improve perf a tiny little bit as we do not do validation but more importantly

  // here is that parseIdentifier will remove an item from the expression stack

  // if "function" or "class" is parsed as identifier (in objects e.g.), which should not happen here.
  const meta = createIdentifier(parser, start, 'function');

  if (parser.inScope('GENERATOR') && parser.eat(tt.dot)) {
    return parseMetaProperty(parser, start, meta, 'sent');
  }

  const node = parseFunctionExpression(parser, start, false);

  if (node.type !== 'FunctionExpression') {
    throw new Error('Expected parseFunction to return a FunctionExpression');
  }

  return node;
}

export function parseMetaProperty(
  parser: JSParser,
  start: Position,
  meta: Identifier,
  propertyName: string,
): MetaProperty {
  if (meta.name === 'function' && propertyName === 'sent' &&
    !parser.isContextual(propertyName)) {
    // They didn't actually say `function.sent`, just `function.`, so a simple error would be less confusing.
    parser.unexpectedToken();
  }

  const escapePosition = parser.state.escapePosition;
  const property = parseIdentifier(parser, true);

  if (property.name === propertyName) {
    parser.banUnicodeEscape(escapePosition, propertyName);
  } else {
    parser.addDiagnostic({
      loc: property.loc,
      description: descriptions.JS_PARSER.INVALID_META_PROPERTY(
        meta.name,
        propertyName,
      ),
    });
  }

  return parser.finishNode(start, {
    type: 'MetaProperty',
    meta,
    property,
  });
}

export function parseImportMetaProperty(parser: JSParser): MetaProperty {
  const start = parser.getPosition();
  const id = parseIdentifier(parser, true);
  parser.expect(tt.dot);
  const node = parseMetaProperty(parser, start, id, 'meta');

  if (!parser.inModule) {
    parser.addDiagnostic({
      loc: node.loc,
      description: descriptions.JS_PARSER.IMPORT_META_OUTSIDE_MODULE,
    });
  }

  return node;
}

export function parseParenExpression(
  parser: JSParser,
  context: ExpressionContext,
): AnyExpression {
  const openContext = parser.expectOpening(tt.parenL, tt.parenR, context);
  const val = parseExpression(parser, context);
  parser.expectClosing(openContext);
  return val;
}

export function parseParenAndDistinguishExpression(
  parser: JSParser,
  context: ExpressionContext,
  canBeArrow: boolean,
): AnyExpression {
  if (parser.state.noArrowAt.includes(parser.state.startPos.index)) {
    canBeArrow = false;
  }

  const startPos = parser.state.startPos;

  const openContext = parser.expectOpening(
    tt.parenL,
    tt.parenR,
    'paren expression',
  );

  const oldMaybeInArrowParameters = parser.state.maybeInArrowParameters;
  const oldYieldPos = parser.state.yieldPos;
  const oldAwaitPos = parser.state.awaitPos;
  const oldYield = parser.state.yieldInPossibleArrowParameters;
  parser.state.maybeInArrowParameters = true;
  parser.state.yieldInPossibleArrowParameters = undefined;
  parser.state.yieldPos = number0;
  parser.state.awaitPos = number0;

  const innerStart = parser.getPosition();
  const exprList: Array<ToReferencedItem> = [];
  const refShorthandDefaultPos: IndexTracker = createIndexTracker();
  const refNeedsArrowPos: IndexTracker = createIndexTracker();
  let first = true;
  let spreadStart;
  let optionalCommaStart;

  while (!parser.match(tt.parenR)) {
    if (first) {
      first = false;
    } else {
      if (!parser.expect(tt.comma, refNeedsArrowPos.index === number0
        ? undefined : parser.getPositionFromIndex(refNeedsArrowPos.index)
      )) {
        break;
      }

      if (parser.match(tt.parenR)) {
        optionalCommaStart = parser.state.startPos;
        break;
      }
    }

    if (parser.match(tt.ellipsis)) {
      const spreadNodeStartPos = parser.state.startPos;
      spreadStart = parser.state.startPos;
      exprList.push(parseParenItem(
        parser,
        parseSpread(parser),
        spreadNodeStartPos,
      ));

      if (parser.match(tt.comma) && parser.lookaheadState().tokenType ===
      tt.parenR) {
        raiseRestNotLast(parser);
        parser.eat(tt.comma);
      }
    } else {
      exprList.push(parseMaybeAssign<ReturnType<typeof parseParenItem>>(
        parser,
        context,
        false,
        refShorthandDefaultPos,
        parseParenItem,
        refNeedsArrowPos,
      ));
    }
  }

  const innerEnd = parser.getPosition();
  parser.expectClosing(openContext);

  parser.state.maybeInArrowParameters = oldMaybeInArrowParameters;

  const arrowStart = startPos;
  if (canBeArrow && shouldParseArrow(parser)) {
    const {valid, returnType, predicate} = parseArrowHead(parser);

    if (valid) {
      checkYieldAwaitInDefaultParams(parser);
      parser.state.yieldPos = oldYieldPos;
      parser.state.awaitPos = oldAwaitPos;

      for (const param of exprList) {
        if (parser.isParenthesized(param)) {
          parser.addDiagnostic({
            loc: param.loc,
            description: descriptions.JS_PARSER.PARENTHESIZED_FUNCTION_PARAMS,
          });
        }
      }

      const arrow = parseArrowExpression(parser, arrowStart, {
        assignmentList: exprList,
      });
      parser.state.yieldInPossibleArrowParameters = oldYield;
      return {
        ...arrow,
        head: {
          ...arrow.head,
          predicate,
          returnType,
        },
      };
    }
  }

  parser.state.yieldInPossibleArrowParameters = oldYield;

  // We keep the old value if it isn't null, for cases like

  //   (x = (yield)) => {}
  parser.state.yieldPos = oldYieldPos || parser.state.yieldPos;
  parser.state.awaitPos = oldAwaitPos || parser.state.awaitPos;

  if (exprList.length === 0) {
    parser.addDiagnostic({
      start: innerStart,
      end: innerEnd,
      description: descriptions.JS_PARSER.EMPTY_PARENTHESIZED_EXPRESSION,
    });

    exprList.push(toReferenceIdentifier(parser, parser.createUnknownIdentifier(
      'empty parenthesized expression',
      innerStart,
      innerEnd,
    )));
  }

  if (optionalCommaStart !== undefined) {
    parser.unexpectedToken(optionalCommaStart);
  }

  if (spreadStart !== undefined) {
    parser.unexpectedToken(spreadStart);
  }

  if (get0(refShorthandDefaultPos.index) > 0) {
    parser.unexpectedToken(parser.getPositionFromIndex(
      refShorthandDefaultPos.index,
    ));
  }

  if (get0(refNeedsArrowPos.index) > 0) {
    parser.unexpectedToken(parser.getPositionFromIndex(refNeedsArrowPos.index));
  }

  const filterList = filterSpread(parser, toReferencedListDeep(
    parser,
    exprList, /* isParenthesizedExpr */true,
  ));

  let val: AnyExpression = filterList[0];
  if (filterList.length > 1) {
    val = parser.finishNodeAt(innerStart, innerEnd, {
      type: 'SequenceExpression',
      expressions: filterList,
    });
  }

  parser.addParenthesized(val);

  return val;
}

export function shouldParseArrow(parser: JSParser): boolean {
  return parser.match(tt.colon) || !parser.canInsertSemicolon();
}

export function parseArrowHead(
  parser: JSParser,
): {
  valid: boolean;
  predicate: undefined | AnyFlowPredicate;
  returnType: undefined | AnyPrimaryType;
} {
  if (parser.match(tt.colon)) {
    const oldNoAnonFunctionType = parser.state.noAnonFunctionType;
    parser.state.noAnonFunctionType = true;

    const branch = parser.createBranch<undefined | TypeAnnotationAndPredicate>();

    branch.add(() => {
      const res = parseTypeAnnotationAndPredicate(parser);

      if (parser.canInsertSemicolon()) {
        // No semicolon insertion expected
        return;
      }

      if (parser.eat(tt.arrow)) {
        return res;
      }
    });

    if (branch.hasBranch()) {
      const typeInfo = branch.pick();
      parser.state.noAnonFunctionType = oldNoAnonFunctionType;

      if (typeInfo === undefined) {
        throw new Error(
          'hasBranchResult call above should have refined this condition',
        );
      }

      return {
        valid: true,
        predicate: typeInfo[1],
        returnType: typeInfo[0],
      };
    } else {
      parser.state.noAnonFunctionType = oldNoAnonFunctionType;
      return {
        valid: false,
        predicate: undefined,
        returnType: undefined,
      };
    }
  } else {
    return {
      valid: parser.eat(tt.arrow),
      predicate: undefined,
      returnType: undefined,
    };
  }
}

// Parse a possible function param or call argument
export function parseParenItem(
  parser: JSParser,
  node: AnyExpression | SpreadElement,
  startPos: Position,
): ToReferencedItem {
  let optional: undefined | boolean = undefined;
  if (parser.eat(tt.question)) {
    optional = true;
  }

  if (parser.match(tt.colon)) {
    const typeAnnotation = parsePrimaryTypeAnnotation(parser);
    return parser.finishNode(startPos, {
      type: 'AmbiguousFlowTypeCastExpression',
      expression: node,
      typeAnnotation,
      optional,
    });
  }

  if (optional) {
    return parser.finishNode(startPos, {
      type: 'AmbiguousFlowTypeCastExpression',
      expression: node,
      typeAnnotation: undefined,
      optional,
    });
  }

  return node;
}

// New's precedence is slightly tricky. It must allow its argument to

// be a `[]` or dot subscript expression, but not a call — at least,

// not without wrapping it in parentheses. Thus, it uses the noCalls

// argument to parseSubscripts to prevent it from 'consuming the

// argument list.
export function parseNew(parser: JSParser): NewExpression | MetaProperty {
  const start = parser.getPosition();
  const meta = parseIdentifier(parser, true);

  if (parser.eat(tt.dot)) {
    const metaProp = parseMetaProperty(parser, start, meta, 'target');

    if (!parser.inScope('NON_ARROW_FUNCTION') && !parser.inScope(
      'CLASS_PROPERTY',
    )) {
      parser.addDiagnostic({
        loc: metaProp.loc,
        description: descriptions.JS_PARSER.NEW_TARGET_OUTSIDE_CLASS,
      });
    }

    return metaProp;
  }

  const callee = parseNoCallExpr(parser, 'new callee');

  if (callee.type === 'ImportCall') {
    parser.addDiagnostic({
      loc: callee.loc,
      description: descriptions.JS_PARSER.SUPER_OUTSIDE_METHOD,
    });
  }

  const optionalMember = getFirstOptionalChainMember(callee);
  if (optionalMember !== undefined) {
    const memberLoc = parser.getLoc(optionalMember);

    parser.addDiagnostic({
      description: descriptions.JS_PARSER.NEW_IN_OPTIONAL_CHAIN(memberLoc),
    });
  }

  if (parser.eat(tt.questionDot)) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.NEW_IN_OPTIONAL_CHAIN(),
    });
  }

  let optional = undefined;
  if (parser.eat(tt.questionDot)) {
    optional = true;
  }

  let typeArguments = undefined;
  if (isTypeSystemEnabled(parser) && parser.isRelational('<')) {
    typeArguments = parser.tryBranch(parseTypeCallArguments);
  }

  let args: Array<AnyExpression | SpreadElement> = [];
  if (parser.match(tt.parenL)) {
    const openContext = parser.expectOpening(
      tt.parenL,
      tt.parenR,
      'new argument',
    );
    args = parseExpressionListNonEmpty(
      parser,
      'new expression argument',
      openContext,
    );
    args = toReferencedList(parser, args);
  } else if (parser.isSyntaxEnabled('ts') && typeArguments !== undefined) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.NEW_WITH_TYPESCRIPT_TYPE_ARGUMENTS_NO_PARENS,
    });
  }

  return parser.finishNode(start, {
    type: 'NewExpression',
    callee,
    typeArguments,
    arguments: args,
    optional,
  });
}

function getFirstOptionalChainMember(
  node: AnyNode,
): undefined | OptionalCallExpression | MemberExpression {
  if (node.type === 'OptionalCallExpression') {
    return node;
  }

  if (node.type === 'MemberExpression') {
    if (node.property.optional) {
      return node;
    }

    if (node.property.type === 'StaticMemberProperty') {
      return getFirstOptionalChainMember(node.object);
    }
  }
}

// Parse template expression.
export function parseTemplateElement(
  parser: JSParser,
  isTagged: boolean,
): TemplateElement {
  const start = parser.getPosition();
  const tokenValue = parser.state.tokenValue;

  if (tokenValue === undefined) {
    if (isTagged) {
      parser.state.invalidTemplateEscapePosition = undefined;
    } else {
      parser.addDiagnostic({
        index: parser.state.invalidTemplateEscapePosition,
        description: descriptions.JS_PARSER.INVALID_TEMPLATE_ESCAPE,
      });
    }
  }

  const raw = parser.getRawInput(
    parser.state.startPos.index,
    parser.state.endPos.index,
  ).replace(/\r\n?/g, '\n');
  const cooked = tokenValue === undefined ? raw : String(tokenValue);

  parser.next();
  const tail = parser.match(tt.backQuote);
  return parser.finishNode(start, {
    type: 'TemplateElement',
    raw,
    cooked,
    tail,
  });
}

export function parseTemplate(
  parser: JSParser,
  isTagged: boolean,
): TemplateLiteral {
  const start = parser.getPosition();
  const openContext = parser.expectOpening(
    tt.backQuote,
    tt.backQuote,
    'template literal',
  );
  const expressions = [];
  let curElt = parseTemplateElement(parser, isTagged);
  const quasis = [curElt];

  while (true) {
    if (parser.match(tt.eof) || curElt.tail === true) {
      break;
    }

    const exprPpenContext = parser.expectOpening(
      tt.dollarBraceL,
      tt.braceR,
      'template expression value',
    );
    expressions.push(parseExpression(parser, 'template expression value'));
    parser.expectClosing(exprPpenContext);

    curElt = parseTemplateElement(parser, isTagged);
    quasis.push(curElt);
  }

  parser.expectClosing(openContext);

  return parser.finishNode(start, {
    type: 'TemplateLiteral',
    expressions,
    quasis,
  });
}

export function parseObjectExpression(
  parser: JSParser,
  refShorthandDefaultPos?: IndexTracker,
): ObjectExpression {
  const propHash: Set<string> = new Set();
  let first = true;

  const start = parser.getPosition();
  const properties = [];

  const openContext = parser.expectOpening(tt.braceL, tt.braceR, 'object');

  while (true) {
    if (parser.match(tt.braceR) || parser.match(tt.eof)) {
      parser.expectClosing(openContext);
      break;
    }

    if (first) {
      first = false;
    } else {
      if (!parser.expect(tt.comma)) {
        break;
      }

      if (parser.eat(tt.braceR)) {
        break;
      }
    }

    if (parser.match(tt.ellipsis)) {
      const prop: SpreadProperty = {
        ...parseSpread(parser),
        type: 'SpreadProperty',
      };
      properties.push(prop);
      continue;
    }

    const start = parser.getPosition();
    let isGenerator = parser.eat(tt.star);
    let isAsync = false;

    let key: StaticPropertyKey | ComputedPropertyKey;
    let escapePosition;

    if (parser.isContextual('async')) {
      if (isGenerator) {
        parser.unexpectedToken();
      }

      const asyncId = parseIdentifier(parser);
      if (parser.match(tt.colon) || parser.match(tt.parenL) || parser.match(
        tt.braceR,
      ) || parser.match(tt.eq) || parser.match(tt.comma)) {
        key = {
          type: 'StaticPropertyKey',
          loc: asyncId.loc,
          value: asyncId,
        };
      } else {
        if (parser.hasPrecedingLineBreak()) {
          parser.addDiagnostic({
            description: descriptions.JS_PARSER.ASYNC_OBJECT_METHOD_LINE_BREAK,
          });
        }

        isAsync = true;
        if (parser.match(tt.star)) {
          parser.next();
          isGenerator = true;
        }
        escapePosition = parser.state.escapePosition;
        key = parseObjectPropertyKey(parser);
      }
    } else {
      escapePosition = parser.state.escapePosition;
      key = parseObjectPropertyKey(parser);
    }

    const prop = parseObjectPropertyValue(parser, {
      key,
      start,
      isGenerator,
      isAsync,
      isPattern: false,
      refShorthandDefaultPos,
      escapePosition,
    });
    if (prop === undefined) {
      continue;
    }
    if (prop.type === 'BindingObjectPatternProperty') {
      throw new Error('Impossible');
    }

    checkPropClash(parser, prop, propHash);
    properties.push(prop);
  }

  return parser.finishNode(start, {
    type: 'ObjectExpression',
    properties,
  });
}

export function parseObjectPattern(
  parser: JSParser,
  refShorthandDefaultPos?: IndexTracker,
): BindingObjectPattern {
  const propHash: Set<string> = new Set();
  let first = true;

  const start = parser.getPosition();
  const properties: Array<BindingObjectPatternProperty> = [];
  let rest: undefined | BindingIdentifier;

  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'object pattern',
  );

  let firstRestLocation = undefined;

  while (true) {
    if (parser.match(tt.eof) || parser.match(tt.braceR)) {
      break;
    }

    if (first) {
      first = false;
    } else {
      parser.expect(tt.comma);

      if (parser.match(tt.eof) || parser.match(tt.braceR)) {
        break;
      }
    }

    let isGenerator = false;
    let isAsync = false;
    let start = parser.getPosition();

    if (parser.eat(tt.ellipsis)) {
      const argument = parseBindingIdentifier(parser);
      rest = argument;

      if (firstRestLocation !== undefined) {
        parser.addDiagnostic({
          loc: argument.loc,
          description: descriptions.JS_PARSER.MULTIPLE_DESTRUCTURING_RESTS,
        });
      }

      if (parser.match(tt.braceR) || parser.match(tt.eof)) {
        break;
      }

      if (parser.match(tt.comma) && parser.lookaheadState().tokenType ===
      tt.braceR) {
        parser.addDiagnostic({
          description: descriptions.JS_PARSER.TRAILING_COMMA_AFTER_REST,
        });
        parser.eat(tt.comma);
        break;
      } else {
        firstRestLocation = argument.loc;
        continue;
      }
    }

    start = parser.getPosition();

    const key = parseObjectPropertyKey(parser);
    const prop = parseObjectPropertyValue(parser, {
      key,
      start,
      isGenerator,
      isAsync,
      isPattern: true,
      refShorthandDefaultPos,
      escapePosition: undefined,
    });

    if (prop === undefined) {
      continue;
    }

    checkPropClash(parser, prop, propHash);

    if (prop.type !== 'BindingObjectPatternProperty') {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.INVALID_OBJECT_PATTERN_PROP,
        loc: prop.loc,
      });
      continue;
    }

    properties.push(prop);
  }

  parser.expectClosing(openContext);

  if (firstRestLocation !== undefined) {
    raiseRestNotLast(parser, firstRestLocation);
  }

  return parser.finishNode(start, {
    type: 'BindingObjectPattern',
    properties,
    rest,
  });
}

export function isGetterOrSetterMethod(
  parser: JSParser,
  key: StaticPropertyKey | ComputedPropertyKey,
  // `key` is always from `name.key`, we just need it here to refine
  keyVal: Identifier | AnyExpression | PrivateName,
  isPattern: boolean,
): keyVal is Identifier {
  return !isPattern && key.type === 'StaticPropertyKey' && keyVal.type ===
  'Identifier' && (keyVal.name === 'get' || keyVal.name === 'set') &&
    (parser.match(tt.string) || // get "string"() {}
    parser.match(tt.num) || // get 1() {}
    parser.match(tt.bracketL) || // get ["string"]() {}
    parser.match(tt.name) || // get foo() {}
    !!parser.state.tokenType.keyword) // get debugger() {}
  ;
}

// get methods aren't allowed to have any parameters

// set methods must have exactly 1 parameter
export function checkGetterSetterParamCount(
  parser: JSParser,
  method:
    | FlowFunctionTypeAnnotation
    | ObjectMethod
    | ClassMethod
    | ClassPrivateMethod
    | TSDeclareFunction
    | TSDeclareMethod,

  kind: string,
): void {
  const head = method.type === 'FlowFunctionTypeAnnotation'
    ? method : method.head;

  if (kind === 'get') {
    if (head.rest !== undefined || head.params.length !== 0) {
      parser.addDiagnostic({
        loc: method.loc,
        description: descriptions.JS_PARSER.GETTER_WITH_PARAMS,
      });
    }
  } else if (kind === 'set') {
    if (head.rest !== undefined) {
      parser.addDiagnostic({
        loc: head.rest.loc,
        description: descriptions.JS_PARSER.SETTER_WITH_REST,
      });
    } else if (head.params.length !== 1) {
      parser.addDiagnostic({
        loc: method.loc,
        description: descriptions.JS_PARSER.SETTER_NOT_ONE_PARAM,
      });
    }
  }
}

type ParseObjectMethodOpts = {
  key: AnyObjectPropertyKey;
  start: Position;
  isGenerator: boolean;
  isAsync: boolean;
  isPattern: boolean;
  escapePosition: undefined | Number0;
};

export function parseObjectMethod(
  parser: JSParser,
  {
    key,
    start,
    isGenerator,
    isAsync,
    isPattern,
    escapePosition,
  }: ParseObjectMethodOpts,
): undefined | ObjectMethod {
  if (isAsync || isGenerator || parser.match(tt.parenL)) {
    if (isPattern) {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.OBJECT_METHOD_IN_PATTERN,
      });
    }

    const partial = parseMethod(parser, {
      kind: 'method',
      isClass: false,
      isGenerator,
      isAsync,
      isConstructor: false,
    });

    const {body} = partial;
    if (body === undefined || body.type !== 'BlockStatement') {
      throw new Error('Expected body');
    }

    return parser.finishNode(start, {
      ...partial,
      body,
      key,
      type: 'ObjectMethod',
      kind: 'method',
    });
  }

  if (isGetterOrSetterMethod(parser, key, key.value, isPattern)) {
    if (isAsync) {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.ASYNC_GETTER_SETTER,
      });
    }

    if (isGenerator) {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.GENERATOR_GETTER_SETTER,
      });
    }

    const kind = key.value.name;
    if (kind !== 'get' && kind !== 'set') {
      throw new Error(
        'Name should be get or set as we already validated it as such',
      );
    }
    parser.banUnicodeEscape(escapePosition, kind);

    const newKey = parseObjectPropertyKey(parser);

    const partial = parseMethod(parser, {
      kind,
      isClass: false,
      isGenerator: false,
      isAsync: false,
      isConstructor: false,
    });

    const {body, head} = partial;
    if (body === undefined || body.type !== 'BlockStatement') {
      throw new Error('Expected body');
    }

    const method: ObjectMethod = parser.finishNode(start, {
      head,
      body,
      key: newKey,
      type: 'ObjectMethod',
      kind,
    });
    checkGetterSetterParamCount(parser, method, method.kind);
    return method;
  }
}

export function parseObjectProperty(
  parser: JSParser,
  key: AnyObjectPropertyKey,
  start: Position,
  isPattern: boolean,
  refShorthandDefaultPos: undefined | IndexTracker,
): undefined | ObjectProperty | BindingObjectPatternProperty {
  if (parser.eat(tt.colon)) {
    if (isPattern) {
      const value = parseMaybeDefault(parser);
      return parser.finishNode(start, {
        key,
        type: 'BindingObjectPatternProperty',
        value,
      });
    } else {
      const value = parseMaybeAssign(
        parser,
        'object property value',
        false,
        refShorthandDefaultPos,
      );
      return parser.finishNode(start, {
        key,
        type: 'ObjectProperty',
        value,
      });
    }
  }

  if (key.type === 'StaticPropertyKey' && key.value.type === 'Identifier') {
    checkReservedWord(
      parser,
      key.value.name,
      parser.getLoc(key.value),
      true,
      true,
    );

    if (isPattern) {
      let value: AnyBindingPattern = toBindingIdentifier(
        parser,
        parser.cloneNode(key.value),
      );

      if (parser.match(tt.eq) && refShorthandDefaultPos) {
        if (refShorthandDefaultPos.index === number0) {
          refShorthandDefaultPos.index = parser.state.startPos.index;
        }

        value = parseMaybeDefault(parser, start, value);
      }

      return parser.finishNode(start, {
        type: 'BindingObjectPatternProperty',
        key,
        value,
      });
    }

    return parser.finishNode(start, {
      type: 'ObjectProperty',
      key,
      value: toReferenceIdentifier(parser, parser.cloneNode(key.value)),
    });
  }
}

type ParseObjectPropValueOpts = {
  key: ComputedPropertyKey | StaticPropertyKey;
  start: Position;
  isGenerator: boolean;
  isAsync: boolean;
  isPattern: boolean;
  refShorthandDefaultPos: undefined | IndexTracker;
  escapePosition: undefined | Number0;
};

export function parseObjectPropertyValue(
  parser: JSParser,
  {
    key,
    start,
    isGenerator,
    isAsync,
    isPattern,
    refShorthandDefaultPos,
    escapePosition,
  }: ParseObjectPropValueOpts,
): undefined | ObjectMethod | ObjectProperty | BindingObjectPatternProperty {
  if (key.variance !== undefined) {
    parser.addDiagnostic({
      loc: key.variance.loc,
      description: descriptions.JS_PARSER.ILLEGAL_VARIANCE,
    });
  }

  // parse type parameters for object method shorthand
  let typeParameters = maybeParseTypeParameters(parser);
  if (typeParameters !== undefined && !parser.match(tt.parenL)) {
    parser.unexpectedToken();
  }

  let node:
    | undefined
    | ObjectMethod
    | ObjectProperty
    | BindingObjectPatternProperty = parseObjectMethod(parser, {
    key,
    start,
    isGenerator,
    isAsync,
    isPattern,
    escapePosition,
  }
  ) || parseObjectProperty(parser, key, start, isPattern, refShorthandDefaultPos);

  if (node === undefined) {
    parser.unexpectedToken();
    return undefined;
  }

  if (typeParameters === undefined) {
    return node;
  } else {
    if (node.type === 'ObjectProperty' || node.type ===
    'BindingObjectPatternProperty') {
      parser.addDiagnostic({
        loc: typeParameters.loc,
        description: descriptions.JS_PARSER.OBJECT_PROPERTY_WITH_TYPE_PARAMETERS,
      });
      return node;
    }

    return {
      ...node,
      head: {
        ...node.head,
        typeParameters,
      },
    };
  }
}

export function parseObjectPropertyKey(
  parser: JSParser,
): StaticPropertyKey | ComputedPropertyKey {
  const start = parser.getPosition();
  const variance = parseFlowVariance(parser);

  if (parser.match(tt.bracketL)) {
    const openContext = parser.expectOpening(
      tt.bracketL,
      tt.bracketR,
      'property name',
    );

    const value = parseMaybeAssign(parser, 'property name');
    parser.expectClosing(openContext);
    return parser.finishNode(start, {
      type: 'ComputedPropertyKey',
      value,
      variance,
    });
  } else {
    parser.pushScope('PROPERTY_NAME', true);

    // We check if it's valid for it to be a private name when we push it.
    let value;
    if (parser.match(tt.num)) {
      value = parseNumericLiteral(parser);
    } else if (parser.match(tt.string)) {
      value = parseStringLiteral(parser);
    } else {
      value = parseMaybePrivateName(parser);
    }

    parser.popScope('PROPERTY_NAME');

    return parser.finishNode(start, {
      type: 'StaticPropertyKey',
      value,
      variance,
    });
  }
}

// Parse object or class method.
export function parseMethod(
  parser: JSParser,
  opts: {
    kind: ClassMethodKind | ObjectMethodKind;
    isGenerator: boolean;
    isAsync: boolean;
    isConstructor: boolean;
    isClass: boolean;
  },
): {
  head: FunctionHead;
  body: undefined | ParseFunctionBodyReturn['body'];
} {
  const {kind, isClass, isGenerator, isAsync, isConstructor} = opts;

  const oldYieldPos = parser.state.yieldPos;
  const oldAwaitPos = parser.state.awaitPos;
  parser.pushScope('FUNCTION', true);
  parser.pushScope('NON_ARROW_FUNCTION');
  parser.pushScope('METHOD', kind);
  parser.pushScope('GENERATOR', isGenerator);
  parser.state.yieldPos = number0;
  parser.state.awaitPos = number0;

  const allowTSModifiers = isConstructor;
  const {typeParameters, rest, params} = parseFunctionParams(
    parser,
    kind,
    allowTSModifiers,
  );
  const start = parser.getPosition();
  const {body, head} = parseFunctionBodyAndFinish(parser, {
    rest,
    params,
    id: undefined,
    allowBodiless: isClass,
    isArrowFunction: false,
    isAsync,
    isGenerator,
    isMethod: true,
    start,
  });

  parser.popScope('METHOD');
  parser.popScope('GENERATOR');
  parser.popScope('FUNCTION');
  parser.popScope('NON_ARROW_FUNCTION');
  parser.state.yieldPos = oldYieldPos;
  parser.state.awaitPos = oldAwaitPos;

  return {
    head: {
      ...head,
      typeParameters,
    },
    body,
  };
}

function createFunctionHead(
  parser: JSParser,
  params: Array<AnyBindingPattern>,
  rest: undefined | AnyTargetBindingPattern,
  opts: Partial<FunctionHead>,
): FunctionHead {
  const nonRestParams: FunctionHead['params'] = [];

  for (const param of params) {
    switch (param.type) {
      case 'BindingIdentifier':
      case 'BindingAssignmentPattern':
      case 'BindingObjectPattern':
      case 'BindingArrayPattern':
        nonRestParams.push(param);
        break;

      default:
        throw new Error('TODO');
    }
  }

  return {
    type: 'FunctionHead',
    rest,
    ...splitFunctionParams(nonRestParams),
    ...opts,
  };
}

// Parse arrow function expression.

// If the parameters are provided, they will be converted to an assignable list.
export function parseArrowExpression(
  parser: JSParser,
  start: Position,
  opts: {
    bindingList?: Array<AnyBindingPattern>;
    assignmentList?: Array<undefined | ToReferencedItem>;
    rest?: AnyTargetBindingPattern;
  },

  isAsync: boolean = false,
): ArrowFunctionExpression {
  // if we got there, it's no more "yield in possible arrow parameters";

  // it's just "yield in arrow parameters"
  if (parser.state.yieldInPossibleArrowParameters) {
    parser.addDiagnostic({
      start: parser.state.yieldInPossibleArrowParameters,
      description: descriptions.JS_PARSER.YIELD_NAME_IN_GENERATOR,
    });
  }

  parser.pushScope('FUNCTION', true);

  const oldYieldPos = parser.state.yieldPos;
  const oldAwaitPos = parser.state.awaitPos;
  const oldMaybeInArrowParameters = parser.state.maybeInArrowParameters;
  parser.pushScope('GENERATOR', false);
  parser.state.maybeInArrowParameters = false;
  parser.state.yieldPos = number0;
  parser.state.awaitPos = number0;

  const headEnd = parser.getPosition();

  const {body, hasHoistedVars} = parseFunctionBody(parser, {
    id: undefined,
    allowBodiless: false,
    isArrowFunction: true,
    isMethod: false,
    isAsync,
    isGenerator: false,
    start,
  });

  let params: Array<AnyBindingPattern> = [];
  let rest: undefined | AnyTargetBindingPattern = opts.rest;

  if (opts.bindingList !== undefined) {
    params = opts.bindingList;
  }

  if (opts.assignmentList !== undefined) {
    ({params, rest} = toFunctionParamsBindingList(
      parser,
      opts.assignmentList,
      'arrow function parameters',
    ));
  }

  checkFunctionNameAndParams(parser, {
    isArrowFunction: true,
    isMethod: false,
    id: undefined,
    params,
    rest,
    start,
  }, body);

  parser.popScope('GENERATOR');
  parser.popScope('FUNCTION');
  parser.state.maybeInArrowParameters = oldMaybeInArrowParameters;
  parser.state.yieldPos = oldYieldPos;
  parser.state.awaitPos = oldAwaitPos;

  return parser.finishNode(start, {
    type: 'ArrowFunctionExpression',
    body,
    head: createFunctionHead(parser, params, rest, {
      loc: parser.finishLocAt(start, headEnd),
      hasHoistedVars,
      async: isAsync,
    }),
  });
}

export function isStrictBody(parser: JSParser, body: AnyNode): boolean {
  if (body.type === 'BlockStatement' && body.directives !== undefined) {
    for (const directive of body.directives) {
      if (directive.value === 'use strict') {
        return true;
      }
    }
  }

  return false;
}

type FunctionBodyParseOpts = {
  allowBodiless: boolean;
  isArrowFunction: boolean;
  isAsync: boolean;
  isGenerator: boolean;
  isMethod: boolean;
  start: Position;
  id: BindingIdentifier | undefined;
};

export function parseFunctionBodyAndFinish(
  parser: JSParser,
  opts: CheckFunctionNameParamsOpts & FunctionBodyParseOpts,
): {
  head: FunctionHead;
  body: undefined | ParseFunctionBodyReturn['body'];
} {
  let returnType = undefined;
  let predicate;

  // For arrow functions, `parseArrow` handles the return type itself.
  if (!opts.isArrowFunction && parser.match(tt.colon)) {
    [returnType, predicate] = parseTypeAnnotationAndPredicate(parser);
  }

  if (opts.allowBodiless && !parser.match(tt.braceL) && parser.isLineTerminator()) {
    return {
      head: createFunctionHead(parser, opts.params, opts.rest, {
        loc: parser.finishLoc(opts.start),
        hasHoistedVars: false,
        generator: opts.isGenerator,
        async: opts.isAsync,
        returnType,
        predicate,
      }),
      body: undefined,
    };
  }

  const headEnd = parser.getPosition();

  const {body, hasHoistedVars} = parseFunctionBody(parser, opts);

  const head = createFunctionHead(parser, opts.params, opts.rest, {
    loc: parser.finishLocAt(opts.start, headEnd),
    generator: opts.isGenerator,
    async: opts.isAsync,
    hasHoistedVars: false,
    returnType,
    predicate,
  });

  checkFunctionNameAndParams(parser, {
    isArrowFunction: opts.isArrowFunction,
    isMethod: opts.isMethod,
    id: opts.id,
    start: opts.start,
    params: opts.params,
    rest: opts.rest,
  }, body);

  head.hasHoistedVars = hasHoistedVars;

  return {
    head,
    body,
  };
}

type ParseFunctionBodyReturn = {
  body: BlockStatement | AnyExpression;
  hasHoistedVars: boolean;
};

export function parseFunctionBody(
  parser: JSParser,
  opts: FunctionBodyParseOpts,
): ParseFunctionBodyReturn {
  if (opts.isArrowFunction) {
    return forwardNoArrowParamsConversionAt(parser, opts.start, () =>
      _parseFunctionBody(parser, opts)
    );
  } else {
    return _parseFunctionBody(parser, opts);
  }
}

// Parse function body and check parameters.
function _parseFunctionBody(
  parser: JSParser,
  opts: FunctionBodyParseOpts,
): ParseFunctionBodyReturn {
  const {isArrowFunction, isAsync, isGenerator} = opts;

  const isExpression = isArrowFunction && !parser.match(tt.braceL);

  parser.pushScope('PARAMETERS', false);
  parser.pushScope('ASYNC', isAsync);

  let hasHoistedVars = false;
  let body: AnyExpression | BlockStatement;
  if (isExpression) {
    body = parseMaybeAssign(parser, 'function body');
  } else {
    // Start a new scope with regard to labels and the `inGenerator`

    // flag (restore them to their old value afterwards).
    const oldLabels = parser.state.labels;
    parser.pushScope('GENERATOR', isGenerator);
    parser.state.labels = [];

    const oldhasHoistedVars = parser.state.hasHoistedVars;
    parser.state.hasHoistedVars = false;

    body = parseBlock(parser, true);
    hasHoistedVars = parser.state.hasHoistedVars;

    parser.popScope('GENERATOR');

    parser.state.hasHoistedVars = oldhasHoistedVars;
    parser.state.labels = oldLabels;
  }

  parser.popScope('ASYNC');
  parser.popScope('PARAMETERS');

  return {body, hasHoistedVars};
}

type CheckFunctionNameParamsOpts = {
  isArrowFunction: boolean;
  isMethod: boolean;
  id: undefined | BindingIdentifier;
  params: Array<AnyBindingPattern>;
  rest: undefined | AnyTargetBindingPattern;
  start: Position;
};

export function checkFunctionNameAndParams(
  parser: JSParser,
  opts: CheckFunctionNameParamsOpts,
  body: AnyExpression | BlockStatement,
  force?: boolean,
): void {
  const {isArrowFunction, isMethod, id, rest, params, start} = opts;

  if (!isSimpleParamList(params, rest) && body.type === 'BlockStatement' &&
    body.directives !== undefined) {
    const firstDirective = body.directives[0];
    if (firstDirective !== undefined && firstDirective.value === 'use strict') {
      parser.addDiagnostic({
        loc: firstDirective.loc,
        description: descriptions.JS_PARSER.STRICT_DIRECTIVE_IN_NON_SIMPLE_PARAMS,
      });
    }
  }

  if (isArrowFunction && force !== true &&
    parser.state.noArrowParamsConversionAt.includes(start.index)) {
    return undefined;
  }

  // If this is a strict mode function, verify that argument names

  // are not repeated, and it does not try to bind the words `eval`
  const _isStrictBody = isStrictBody(parser, body);
  const isStrict = parser.inScope('STRICT') || _isStrictBody;

  const isSimpleParams = isSimpleParamList(params, rest);
  const shouldCheckLVal: boolean = isStrict || isArrowFunction || isMethod ||
  !isSimpleParams;

  parser.pushScope('STRICT', isStrict);

  if (shouldCheckLVal) {
    const clashes: Map<string, AnyNode> = new Map();

    if (id !== undefined) {
      checkLVal(parser, id, true, undefined, 'function name');
    }

    for (const param of params) {
      if (_isStrictBody && param.type !== 'BindingIdentifier') {
        parser.addDiagnostic({
          loc: param.loc,
          description: descriptions.JS_PARSER.NON_SIMPLE_PARAM_IN_EXPLICIT_STRICT_FUNCTION,
        });
      }
      checkLVal(parser, param, true, clashes, 'function parameter list');
    }
  }

  parser.popScope('STRICT');
}

function isSimpleParamList(
  params: Array<AnyBindingPattern>,
  rest: undefined | AnyTargetBindingPattern,
): boolean {
  if (rest !== undefined) {
    return false;
  }

  for (const param of params) {
    if (param.type !== 'BindingIdentifier') {
      return false;
    }
  }

  return true;
}

// Parses a comma-separated list of expressions, and returns them as

// an array. `close` is the token type that ends the list, and

// `allowEmpty` can be turned on to allow subsequent commas with

// nothing in between them to be parsed as `null` (which is needed

// for array literals).
export function parseExpressionList(
  parser: JSParser,
  context: ExpressionContext,
  openContext: OpeningContext,
  allowEmpty?: boolean,
  refShorthandDefaultPos?: IndexTracker,
): Array<ReturnType<typeof parseCallArgument>> {
  const elts = [];
  let first = true;

  while (true) {
    if (parser.match(openContext.close) || parser.match(tt.eof)) {
      break;
    }

    if (first) {
      first = false;
    } else {
      parser.expect(tt.comma);

      if (parser.match(openContext.close)) {
        break;
      }
    }

    elts.push(parseCallArgument(
      parser,
      context,
      allowEmpty,
      refShorthandDefaultPos,
    ));
  }

  parser.expectClosing(openContext);

  return elts;
}

export function parseExpressionListNonEmpty(
  parser: JSParser,
  context: ExpressionContext,
  openContext: OpeningContext,
  refShorthandDefaultPos?: IndexTracker,
): Array<AnyExpression> {
  const val = parseExpressionList(
    parser,
    context,
    openContext,
    false,
    refShorthandDefaultPos,
  );
  // @ts-ignore: Passed allowEmpty: false above
  return val;
}

export function parseCallArgument(
  parser: JSParser,
  context: ExpressionContext,
  maybeAllowEmpty?: boolean,
  refShorthandDefaultPos?: IndexTracker,
  refNeedsArrowPos?: IndexTracker,
  refTrailingCommaPos?: IndexTracker,
): undefined | AnyExpression | SpreadElement | AmbiguousFlowTypeCastExpression {
  const allowEmpty = Boolean(maybeAllowEmpty);

  let elt: undefined | ReturnType<typeof parseParenItem>;
  if (allowEmpty && parser.match(tt.comma)) {
    elt = undefined;
  } else if (parser.match(tt.ellipsis)) {
    const spreadNodeStart = parser.state.startPos;

    elt = parseParenItem(parser, parseSpread(
      parser,
      refShorthandDefaultPos,
      refNeedsArrowPos,
    ), spreadNodeStart);

    if (refTrailingCommaPos && parser.match(tt.comma)) {
      refTrailingCommaPos.index = parser.state.startPos.index;
    }
  } else {
    elt = parseMaybeAssign<ReturnType<typeof parseParenItem>>(
      parser,
      context,
      false,
      refShorthandDefaultPos,
      parseParenItem,
      refNeedsArrowPos,
    );
  }

  return elt;
}

// Parse the next token as an identifier. If `liberal` is true (used

// when parsing properties), it will also convert keywords into

// identifiers.
export function parseIdentifier(parser: JSParser, liberal?: boolean): Identifier {
  const start = parser.getPosition();
  const name = parseIdentifierName(parser, liberal);
  return createIdentifier(parser, start, name);
}

export function parseBindingIdentifier(
  parser: JSParser,
  liberal?: boolean,
): BindingIdentifier {
  return toBindingIdentifier(parser, parseIdentifier(parser, liberal));
}

export function parseReferenceIdentifier(
  parser: JSParser,
  liberal?: boolean,
): ReferenceIdentifier {
  return toReferenceIdentifier(parser, parseIdentifier(parser, liberal));
}

export function toBindingIdentifier(
  parser: JSParser,
  node: ReferenceIdentifier | Identifier | AssignmentIdentifier,
): BindingIdentifier {
  return parser.finalizeNode({
    ...node,
    type: 'BindingIdentifier',
  });
}

export function toAssignmentIdentifier(
  parser: JSParser,
  node: ReferenceIdentifier | Identifier | BindingIdentifier,
): AssignmentIdentifier {
  return parser.finalizeNode({
    ...node,
    type: 'AssignmentIdentifier',
  });
}

export function toReferenceIdentifier(
  parser: JSParser,
  node: BindingIdentifier | Identifier | AssignmentIdentifier,
): ReferenceIdentifier {
  return parser.finalizeNode({
    ...node,
    type: 'ReferenceIdentifier',
  });
}

export function toIdentifier(
  parser: JSParser,
  node: BindingIdentifier | ReferenceIdentifier | AssignmentIdentifier,
): Identifier {
  return {
    ...node,
    type: 'Identifier',
  };
}

export function createIdentifier(
  parser: JSParser,
  start: Position,
  name: string,
): Identifier {
  return parser.finishNode(start, {
    type: 'Identifier',
    name,
  });
}

export function parseIdentifierName(
  parser: JSParser,
  liberal: boolean = false,
): string {
  const loc = parser.finishLocAt(parser.state.startPos, parser.state.endPos);

  if (!liberal) {
    checkReservedWord(
      parser,
      String(parser.state.tokenValue),
      loc,
      !!parser.state.tokenType.keyword,
      false,
    );
  }

  let name: string;

  if (parser.match(tt.name)) {
    name = String(parser.state.tokenValue);
  } else if (parser.state.tokenType.keyword !== undefined) {
    name = parser.state.tokenType.keyword;

    // `class` and `function` keywords push new context into this.context.

    // But there is no chance to pop the context if the keyword is consumed

    // as an identifier such as a property name.

    // If the previous token is a dot, this does not apply because the

    // context-managing code already ignored the keyword
    if ((name === 'class' || name === 'function') &&
      (parser.state.lastEndPos.index !== inc(parser.state.lastStartPos.index) ||
      parser.input.charCodeAt(get0(parser.state.lastStartPos.index)) !==
      charCodes.dot)) {
      parser.state.context.pop();
    }
  } else {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.EXPECTED_IDENTIFIER,
    });
    name = '';
  }

  if (!liberal) {
    checkReservedWord(parser, name, loc, parser.state.tokenType.keyword !==
    undefined, false);
  }

  parser.next();
  return name;
}

export function checkReservedWord(
  parser: JSParser,
  word: string,
  loc: SourceLocation,
  checkKeywords: boolean,
  isBinding: boolean,
): void {
  if (parser.isSyntaxEnabled('ts')) {
    // TypeScript support in Babel disables reserved word checking...

    // This is mostly because TS allows reserved words in certain scenarios

    // TODO we should just allow those rather than relying on this hack
    return undefined;
  }

  if (parser.inScope('GENERATOR') && word === 'yield') {
    parser.addDiagnostic({
      loc,
      description: descriptions.JS_PARSER.YIELD_NAME_IN_GENERATOR,
    });
  }

  if (parser.inScope('ASYNC') && word === 'await') {
    parser.addDiagnostic({
      loc,
      description: descriptions.JS_PARSER.AWAIT_NAME_IN_ASYNC,
    });
  }

  if (parser.inScope('CLASS_PROPERTY') && word === 'arguments') {
    parser.addDiagnostic({
      loc,
      description: descriptions.JS_PARSER.ARGUMENTS_IN_CLASS_FIELD,
    });
  }

  if (checkKeywords && isKeyword(word)) {
    parser.addDiagnostic({
      loc,
      description: descriptions.JS_PARSER.UNEXPECTED_KEYWORD(word),
    });
  }

  let isReserved = false;
  if (parser.inScope('STRICT')) {
    if (isBinding) {
      isReserved = isStrictBindReservedWord(word, parser.inModule);
    } else {
      isReserved = isStrictReservedWord(word, parser.inModule);
    }
  } else {
    isReserved = isReservedWord(word, parser.inModule);
  }

  if (isReserved) {
    if (!parser.inScope('ASYNC') && word === 'await') {
      parser.addDiagnostic({
        loc,
        description: descriptions.JS_PARSER.AWAIT_OUTSIDE_ASYNC,
      });
    } else {
      parser.addDiagnostic({
        loc,
        description: descriptions.JS_PARSER.RESERVED_WORD(word),
      });
    }
  }
}

// Parses await expression inside async function.
export function parseAwait(parser: JSParser): AwaitExpression {
  if (!parser.state.awaitPos) {
    parser.state.awaitPos = parser.state.index;
  }

  if (!parser.inScope('ASYNC')) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.AWAIT_OUTSIDE_ASYNC,
    });
  }

  const start = parser.getPosition();
  parser.next();

  if (parser.inScope('PARAMETERS')) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.AWAIT_IN_ASYNC_PARAMS,
    });
  }

  if (parser.eat(tt.star)) {
    parser.addDiagnostic({
      start,
      description: descriptions.JS_PARSER.AWAIT_STAR,
    });
  }

  const argument = parseMaybeUnary(parser, 'await argument');
  return parser.finishNode(start, {type: 'AwaitExpression', argument});
}

// Parses yield expression inside generator.
export function parseYield(parser: JSParser, noIn?: boolean): YieldExpression {
  if (!parser.state.yieldPos) {
    parser.state.yieldPos = parser.state.index;
  }

  const start = parser.getPosition();

  if (parser.inScope('PARAMETERS')) {
    parser.addDiagnostic({
      start,
      description: descriptions.JS_PARSER.YIELD_IN_GENERATOR_PARAMS,
    });
  }

  if (parser.state.maybeInArrowParameters &&
    // We only set yieldInPossibleArrowParameters if we haven't already

    // found a possible invalid YieldExpression.
    parser.state.yieldInPossibleArrowParameters === undefined) {
    parser.state.yieldInPossibleArrowParameters = start;
  }

  parser.next();

  let delegate: undefined | boolean;
  let argument: undefined | AnyExpression;
  if (parser.match(tt.semi) || !parser.match(tt.star) &&
    !parser.state.tokenType.startsExpr || parser.canInsertSemicolon()) {
    delegate = false;
  } else {
    delegate = parser.eat(tt.star);
    argument = parseMaybeAssign<AnyExpression>(parser, 'yield argument', noIn);
  }

  return parser.finishNode(start, {
    type: 'YieldExpression',
    delegate,
    argument,
  });
}

function parseNullLiteral(parser: JSParser): NullLiteral {
  const start = parser.getPosition();
  parser.next();
  return parser.finishNode(start, {type: 'NullLiteral'});
}

export function parseStringLiteral(parser: JSParser): StringLiteral {
  const start = parser.getPosition();
  const value = String(parser.state.tokenValue);
  parser.next();
  return parser.finishNode(start, {
    type: 'StringLiteral',
    value,
  });
}

function parseBigIntLiteral(parser: JSParser): BigIntLiteral {
  const start = parser.getPosition();
  const value = String(parser.state.tokenValue);
  parser.next();
  return parser.finishNode(start, {
    type: 'BigIntLiteral',
    value,
  });
}

export function parseNumericLiteral(parser: JSParser): NumericLiteral {
  const start = parser.getPosition();
  const value = Number(parser.state.tokenValue);
  parser.next();
  return parser.finishNode(start, {
    type: 'NumericLiteral',
    value,
  });
}

function parseRegExpLiteral(parser: JSParser): RegExpLiteral {
  const start = parser.getPosition();
  const value = parser.state.tokenValue;
  if (!(value instanceof RegExpTokenValue)) {
    throw new Error('Expected regex token value');
  }
  parser.next();

  const {flags, pattern} = value;

  const regexParser = createRegExpParser({
    offsetPosition: {
      // Advance passed first slash
      ...start,
      column: inc(start.column),
      index: inc(start.index),
    },
    path: parser.filename,
    input: pattern,
    unicode: flags.has('u'),
  });

  const {diagnostics, expression} = regexParser.parse();

  for (const diagnostic of diagnostics) {
    parser.addDiagnostic(diagnostic);
  }

  return parser.finishNode(start, {
    type: 'RegExpLiteral',
    expression,
    global: flags.has('g'),
    multiline: flags.has('m'),
    sticky: flags.has('y'),
    insensitive: flags.has('i'),
    noDotNewline: flags.has('s'),
    unicode: flags.has('u'),
  });
}

function parseImportOrMetaProperty(parser: JSParser): ImportCall | MetaProperty {
  if (parser.lookaheadState().tokenType === tt.dot) {
    return parseImportMetaProperty(parser);
  } else {
    return parseImportCall(parser);
  }
}

function parseImportCall(parser: JSParser): ImportCall {
  parser.expect(tt._import);

  const start = parser.getPosition();
  const openContext = parser.expectOpening(tt.parenL, tt.parenR, 'array');

  let argument: ReturnType<typeof parseCallArgument>;

  if (parser.match(tt.parenR)) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.IMPORT_EXACT_ARGUMENTS,
    });

    argument = toReferenceIdentifier(parser, parser.createUnknownIdentifier(
      'import call argument',
    ));
  } else {
    const callArg = parseCallArgument(parser, 'call expression argument', false);
    if (callArg === undefined) {
      throw new Error(
        'Expected argument, parseExpressionListItem was passed maybeAllowEmpty: false',
      );
    } else {
      argument = callArg;
    }
  }

  // TODO warn on multiple arguments
  if (parser.eat(tt.comma)) {
    parser.addDiagnostic({
      start: parser.state.lastStartPos,
      end: parser.state.lastEndPos,
      description: descriptions.JS_PARSER.IMPORT_TRAILING_COMMA,
    });
  }

  if (argument.type === 'SpreadElement') {
    parser.addDiagnostic({
      loc: argument.loc,
      description: descriptions.JS_PARSER.IMPORT_SPREAD,
    });
  }

  parser.expectClosing(openContext);

  const spreadOrExpression: AnyExpression | SpreadElement = argument.type ===
  'AmbiguousFlowTypeCastExpression' ? argument.expression : argument;

  const expression: AnyExpression = spreadOrExpression.type === 'SpreadElement'
    ? spreadOrExpression.argument : spreadOrExpression;

  return parser.finishNode(start, {type: 'ImportCall', argument: expression});
}

function parseSuper(parser: JSParser): Super {
  if (!parser.inScope('METHOD') && !parser.inScope('CLASS_PROPERTY') &&
    parser.sourceType !== 'template') {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.SUPER_OUTSIDE_METHOD,
    });
  }

  const start = parser.getPosition();
  parser.next();

  if (!parser.match(tt.parenL) && !parser.match(tt.bracketL) && !parser.match(
    tt.dot,
  )) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.INVALID_SUPER_SUFFIX,
    });
  }

  const loc = parser.finishLoc(start);

  if (parser.match(tt.parenL) &&
    (parser.getLastScope('METHOD') !== 'constructor' || parser.getLastScope(
      'CLASS',
    ) !== 'derived') && parser.sourceType !== 'template') {
    parser.addDiagnostic({
      loc,
      description: descriptions.JS_PARSER.SUPER_CALL_OUTSIDE_CONSTRUCTOR,
    });
  }

  return parser.finalizeNode({
    type: 'Super',
    loc,
  });
}

function parseDoExpression(parser: JSParser): DoExpression {
  const start = parser.getPosition();
  parser.next();
  const oldLabels = parser.state.labels;
  parser.state.labels = [];
  parser.pushScope('FUNCTION', false);
  const body = parseBlock(parser, false);
  parser.popScope('FUNCTION');
  parser.state.labels = oldLabels;
  return parser.finishNode(start, {
    type: 'DoExpression',
    body,
  });
}

function parseArrayExpression(
  parser: JSParser,
  refShorthandDefaultPos?: IndexTracker,
): ArrayExpression {
  const start = parser.getPosition();
  const openContext = parser.expectOpening(tt.bracketL, tt.bracketR, 'array');

  const elements = toReferencedListOptional(parser, parseExpressionList(
    parser,
    'array element',
    openContext,
    true,
    refShorthandDefaultPos,
  ));

  return parser.finishNode(start, {
    type: 'ArrayExpression',
    elements,
  });
}
