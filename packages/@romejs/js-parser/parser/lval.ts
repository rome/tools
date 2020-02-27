/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {IndexTracker, createIndexTracker} from '@romejs/js-parser-utils';
import {Position, SourceLocation} from '@romejs/parser-core';
import {types as tt} from '../tokenizer/types';
import {isStrictBindReservedWord} from '@romejs/js-parser-utils';
import {
  AnyExpression,
  AnyNode,
  AnyBindingPattern,
  AnyAssignmentPattern,
  SpreadElement,
  BindingAssignmentPattern,
  SpreadProperty,
  BindingArrayPattern,
  ConstTSAccessibility,
  FlowTypeCastExpression,
  BindingObjectPattern,
  BindingIdentifier,
  AnyTargetBindingPattern,
  AssignmentObjectPatternProperty,
  AnyAuxiliary,
  AnyTargetAssignmentPattern,
  BindingObjectPatternProperty,
  AssignmentIdentifier,
  AnyParamBindingPattern,
  ReferenceIdentifier,
  AmbiguousFlowTypeCastExpression,
} from '@romejs/js-ast';
import {JSParser, OpeningContext} from '../parser';
import {
  parseMaybeAssign,
  parseObjectPattern,
  ambiguousTypeCastToParameter,
  parsePrimaryTypeAnnotation,
  parseTSAccessModifier,
  hasTSModifier,
} from './index';
import {PartialDiagnosticAdvice} from '@romejs/diagnostics';
import {get0} from '@romejs/ob1';
import {
  parseBindingIdentifier,
  toAssignmentIdentifier,
  toBindingIdentifier,
  toReferenceIdentifier,
} from './expression';

const VALID_REST_ARGUMENT_TYPES = ['Identifier', 'MemberExpression'];

type ToAssignmentPatternNode =
  | AnyExpression
  | AnyAssignmentPattern
  | AnyTargetBindingPattern
  | AnyAuxiliary;

// Convert existing expression atom to assignable pattern if possible.
export function toAssignmentPattern(
  parser: JSParser,
  node: AnyNode,
  contextDescription: string,
): AnyAssignmentPattern {
  switch (node.type) {
    case 'AssignmentObjectPattern':
    case 'AssignmentArrayPattern':
    case 'AssignmentAssignmentPattern':
    case 'AssignmentObjectPatternProperty':
    case 'AssignmentIdentifier':
    case 'MemberExpression':
      return node;

    case 'AmbiguousFlowTypeCastExpression':
      return toAssignmentPattern(
        parser,
        ambiguousTypeCastToParameter(parser, node),
        contextDescription,
      );

    case 'BindingIdentifier':
    case 'ReferenceIdentifier':
      return toAssignmentIdentifier(node);

    case 'TSAsExpression':
      return {
        ...node,
        type: 'TSAssignmentAsExpression',
        expression: toTargetAssignmentPattern(
          parser,
          node.expression,
          contextDescription,
        ),
      };

    case 'TSNonNullExpression':
      return {
        ...node,
        type: 'TSAssignmentNonNullExpression',
        expression: toTargetAssignmentPattern(
          parser,
          node.expression,
          contextDescription,
        ),
      };

    case 'TSTypeAssertion':
      return {
        ...node,
        type: 'TSAssignmentTypeAssertion',
        expression: toTargetAssignmentPattern(
          parser,
          node.expression,
          contextDescription,
        ),
      };

    case 'ObjectExpression': {
      const props = [];
      let rest: undefined | AssignmentIdentifier;
      for (let index = 0; index < node.properties.length; index++) {
        const prop = node.properties[index];
        if (prop.type === 'SpreadProperty') {
          const arg = toTargetAssignmentPattern(
            parser,
            prop.argument,
            contextDescription,
          );
          if (arg.type === 'AssignmentIdentifier') {
            rest = arg;
          } else {
            parser.addDiagnostic({
              loc: arg.loc,
              message: "Invalid rest operator's argument",
            });
          }
          continue;
        }

        const isLast = index === node.properties.length - 1;
        props.push(toAssignmentObjectProperty(parser, prop, isLast));
      }
      return {
        type: 'AssignmentObjectPattern',
        loc: node.loc,
        properties: props,
        rest,
      };
    }

    case 'ArrayExpression': {
      const {list: elements, rest} = toAssignableList(
        parser,
        node.elements,
        contextDescription,
      );
      return {
        type: 'AssignmentArrayPattern',
        loc: node.loc,
        elements,
        rest,
      };
    }

    case 'AssignmentExpression': {
      if (node.operator !== '=') {
        parser.addDiagnostic({
          loc: parser.getLoc(node.left),
          message:
            "Only '=' operator can be used for specifying default value.",
        });
      }

      return {
        type: 'AssignmentAssignmentPattern',
        left: toTargetAssignmentPattern(parser, node.left, contextDescription),
        right: node.right,
        loc: node.loc,
      };
    }

    default: {
      const message = `Invalid left-hand side in ${contextDescription} ${node.type}`;
      parser.addDiagnostic({
        loc: node.loc,
        message,
      });
      return toAssignmentIdentifier(
        parser.createUnknownIdentifier(contextDescription),
      );
    }
  }
}

export function toTargetAssignmentPattern(
  parser: JSParser,
  node: ToAssignmentPatternNode,
  contextDescription: string,
): AnyTargetAssignmentPattern {
  const binding = toAssignmentPattern(parser, node, contextDescription);

  switch (binding.type) {
    case 'AssignmentIdentifier':
    case 'AssignmentArrayPattern':
    case 'AssignmentObjectPattern':
    case 'MemberExpression':
    case 'TSAssignmentAsExpression':
    case 'TSAssignmentNonNullExpression':
    case 'TSAssignmentTypeAssertion':
      return binding;

    default:
      parser.addDiagnostic({
        loc: node.loc,
        message: 'Not a valid assignment target',
      });
      return {
        type: 'AssignmentIdentifier',
        loc: node.loc,
        name: 'X',
      };
  }
}

export function toTargetBindingPattern(
  parser: JSParser,
  node: ToAssignmentPatternNode,
  contextDescription: string,
): AnyTargetBindingPattern {
  const binding = toBindingPattern(parser, node, contextDescription);

  switch (binding.type) {
    case 'BindingIdentifier':
    case 'BindingArrayPattern':
    case 'BindingObjectPattern':
      return binding;

    default:
      // TODO return Unknown
      throw new Error('TODO ' + binding.type);
  }
}

export function toParamBindingPattern(
  parser: JSParser,
  node: ToAssignmentPatternNode,
  contextDescription: string,
): AnyParamBindingPattern {
  const binding = toBindingPattern(parser, node, contextDescription);

  switch (binding.type) {
    case 'BindingIdentifier':
    case 'BindingArrayPattern':
    case 'BindingObjectPattern':
    case 'BindingAssignmentPattern':
      return binding;

    default:
      // TODO return Unknown
      throw new Error('TODO ' + binding.type);
  }
}

export function toBindingPattern(
  parser: JSParser,
  node: ToAssignmentPatternNode,
  contextDescription: string,
): AnyBindingPattern {
  const binding = toAssignmentPattern(parser, node, contextDescription);

  if (binding.type === 'MemberExpression') {
    parser.addDiagnostic({
      loc: node.loc,
      message: 'Binding member expression',
    });

    return {
      type: 'BindingIdentifier',
      name: 'X',
      loc: node.loc,
    };
  }

  switch (binding.type) {
    case 'AssignmentObjectPattern': {
      const newNode: BindingObjectPattern = {
        ...binding,
        type: 'BindingObjectPattern',
        rest:
          binding.rest === undefined
            ? undefined
            : toBindingIdentifier(binding.rest),
        properties: binding.properties.map(prop => {
          const bindingProp = toBindingPattern(
            parser,
            prop,
            contextDescription,
          );

          if (bindingProp.type !== 'BindingObjectPatternProperty') {
            throw new Error('impossible condition');
          }

          return bindingProp;
        }),
      };
      return newNode;
    }

    case 'AssignmentAssignmentPattern': {
      const newNode: BindingAssignmentPattern = {
        ...binding,
        type: 'BindingAssignmentPattern',
        left: toTargetBindingPattern(parser, binding.left, contextDescription),
      };
      return newNode;
    }

    case 'AssignmentArrayPattern': {
      const newNode: BindingArrayPattern = {
        ...binding,
        type: 'BindingArrayPattern',
        elements: binding.elements.map(elem =>
          elem === undefined
            ? elem
            : toParamBindingPattern(parser, elem, contextDescription),
        ),
        rest:
          binding.rest === undefined
            ? undefined
            : toTargetBindingPattern(parser, binding.rest, contextDescription),
      };
      return newNode;
    }

    case 'AssignmentIdentifier': {
      const newNode: BindingIdentifier = {
        ...binding,
        type: 'BindingIdentifier',
      };
      return newNode;
    }

    case 'AssignmentObjectPatternProperty': {
      const newNode: BindingObjectPatternProperty = {
        ...binding,
        type: 'BindingObjectPatternProperty',
        value: toBindingPattern(parser, binding.value, contextDescription),
      };
      return newNode;
    }

    default:
      throw new Error(`Unknown node ${node.type}`);
  }
}

export function toAssignmentObjectProperty(
  parser: JSParser,
  prop: AnyNode,
  isLast: boolean,
): AssignmentObjectPatternProperty {
  switch (prop.type) {
    case 'ObjectMethod': {
      const error =
        prop.kind === 'get' || prop.kind === 'set'
          ? "Object pattern can't contain getter or setter"
          : "Object pattern can't contain methods";

      parser.addDiagnostic({
        loc: prop.key.loc,
        message: error,
      });

      const fakeProp: AssignmentObjectPatternProperty = {
        type: 'AssignmentObjectPatternProperty',
        loc: prop.loc,
        key: {
          type: 'StaticPropertyKey',
          value: {
            type: 'Identifier',
            name: 'X',
            loc: prop.loc,
          },
          loc: prop.loc,
        },
        value: {
          type: 'AssignmentIdentifier',
          name: 'X',
          loc: prop.loc,
        },
      };

      return fakeProp;
    }

    case 'ObjectProperty':
      return {
        ...prop,
        type: 'AssignmentObjectPatternProperty',
        value: toAssignmentPattern(
          parser,
          prop.value,
          'assignment object property value',
        ),
      };

    default:
      parser.addDiagnostic({
        loc: prop.loc,
        message: 'Not a valid assignment object pattern property',
      });
      return {
        type: 'AssignmentObjectPatternProperty',
        loc: prop.loc,
        key: {
          type: 'StaticPropertyKey',
          loc: prop.loc,
          value: {
            type: 'Identifier',
            loc: prop.loc,
            name: 'X',
          },
        },
        value: {
          type: 'AssignmentIdentifier',
          loc: prop.loc,
          name: 'X',
        },
      };
  }
}

export function toAssignableList(
  parser: JSParser,
  exprList: Array<
    | undefined
    | AnyAssignmentPattern
    | AmbiguousFlowTypeCastExpression
    | SpreadElement
    | AnyExpression
  >,
  contextDescription: string,
): {
  list: Array<undefined | AnyAssignmentPattern>;
  rest: undefined | AnyTargetAssignmentPattern;
} {
  const newList: Array<AnyAssignmentPattern> = [];
  let rest: undefined | AnyTargetAssignmentPattern;

  let end = exprList.length;

  // Validate last element
  if (end > 0) {
    let last = exprList[end - 1];

    if (last !== undefined && last.type === 'SpreadElement') {
      const arg = toTargetAssignmentPattern(
        parser,
        last.argument,
        contextDescription,
      );
      rest = arg;
      end--;
    }

    if (
      last !== undefined &&
      last.type === 'AmbiguousFlowTypeCastExpression' &&
      last.expression.type === 'SpreadElement'
    ) {
      rest = ambiguousTypeCastToParameter(parser, {
        ...last,
        expression: last.expression.argument,
      });
      end--;
    }
  }

  // Turn type casts that we found in function parameter head into type annotated params
  for (let i = 0; i < end; i++) {
    const expr = exprList[i];
    if (expr === undefined) {
      continue;
    }

    if (expr.type === 'AmbiguousFlowTypeCastExpression') {
      exprList[i] = ambiguousTypeCastToParameter(parser, expr);
    }

    if (expr.type === 'TSAsExpression' || expr.type === 'TSTypeAssertion') {
      parser.addDiagnostic({
        loc: expr.loc,
        message: 'Unexpected type cast in parameter position',
      });
    }
  }

  for (let i = 0; i < end; i++) {
    const elt = exprList[i];
    if (elt === undefined) {
      continue;
    }

    if (elt.type === 'SpreadElement') {
      raiseRestNotLast(parser, parser.getLoc(elt));
    }

    const assign = toAssignmentPattern(parser, elt, contextDescription);
    newList.push(assign);
  }

  return {list: newList, rest};
}

export function toFunctionParamsBindingList(
  parser: JSParser,
  exprList: Array<undefined | ToReferencedItem>,
  contextDescription: string,
): {
  params: Array<BindingAssignmentPattern | AnyTargetBindingPattern>;
  rest: undefined | AnyTargetBindingPattern;
} {
  const bindingList: Array<
    BindingAssignmentPattern | AnyTargetBindingPattern
  > = [];

  const {list: assignmentList, rest: assignmentRest} = toAssignableList(
    parser,
    exprList,
    contextDescription,
  );

  const bindingRest =
    assignmentRest === undefined
      ? assignmentRest
      : toTargetBindingPattern(parser, assignmentRest, contextDescription);

  for (const item of assignmentList) {
    if (item === undefined) {
      // TODO should never happen?
      continue;
    }

    if (item.type === 'AssignmentAssignmentPattern') {
      const binding = toBindingPattern(parser, item, contextDescription);
      if (binding.type !== 'BindingAssignmentPattern') {
        throw new Error('TODO');
      }

      bindingList.push(binding);
      continue;
    }

    const binding = toTargetBindingPattern(parser, item, contextDescription);
    bindingList.push(binding);
  }

  return {params: bindingList, rest: bindingRest};
}

// this is a list of nodes, from 'something like a call expression, we need to filter the
// type casts that we've found that are illegal in this context
export function toReferencedList(
  parser: JSParser,
  exprList: Array<ToReferencedItem>,
  isParenthesizedExpr?: boolean,
): Array<SpreadElement | AnyExpression> {
  for (let i = 0; i < exprList.length; i++) {
    const expr = exprList[i];
    exprList[i] = toReferencedItem(
      parser,
      expr,
      exprList.length > 1,
      isParenthesizedExpr,
    );
  }

  // @ts-ignore: We actually filtered them out
  return exprList;
}

export function toReferencedListOptional(
  parser: JSParser,
  exprList: Array<undefined | ToReferencedItem>,
  isParenthesizedExpr?: boolean,
): Array<undefined | SpreadElement | AnyExpression> {
  for (let i = 0; i < exprList.length; i++) {
    const expr = exprList[i];
    if (expr !== undefined) {
      exprList[i] = toReferencedItem(
        parser,
        expr,
        exprList.length > 1,
        isParenthesizedExpr,
      );
    }
  }

  // @ts-ignore: We actually filtered them out
  return exprList;
}

export type ToReferencedItem =
  | AmbiguousFlowTypeCastExpression
  | SpreadElement
  | AnyExpression;

export function toReferencedItem(
  parser: JSParser,
  expr: ToReferencedItem,
  multiple?: boolean,
  isParenthesizedExpr?: boolean,
): AnyExpression | SpreadElement {
  if (expr.type !== 'AmbiguousFlowTypeCastExpression') {
    return expr;
  }

  if (parser.isSyntaxEnabled('ts')) {
    parser.addDiagnostic({
      loc: expr.loc,
      message: "Flow type cast expressions aren't allowed in TypeScript",
    });
  }

  if (!parser.isParenthesized(expr) && (multiple || !isParenthesizedExpr)) {
    parser.addDiagnostic({
      loc: expr.loc,
      message:
        'The type cast expression is expected to be wrapped with parentheses',
    });
  }

  if (expr.optional) {
    parser.addDiagnostic({
      loc: expr.loc,
      message:
        'Type cast expressions cannot be optional. Did you mean for this to be a function parameter?',
    });
  }

  const {typeAnnotation, expression} = expr;

  if (typeAnnotation === undefined) {
    parser.addDiagnostic({
      loc: expr.loc,
      message:
        'Type cast expression has no type annotation. Did you mean for this to be a function parameter?',
    });
    return expression;
  }

  if (expression.type === 'SpreadElement') {
    throw new Error(
      "I don't think a SpreadElement is ever allowed to hit this path?",
    );
  }

  const node: FlowTypeCastExpression = {
    type: 'FlowTypeCastExpression',
    loc: expr.loc,
    typeAnnotation,
    expression,
  };
  return node;
}

export function filterSpread<T extends AnyNode>(
  parser: JSParser,
  elems: Array<T | ReferenceIdentifier | SpreadElement>,
): Array<ReferenceIdentifier | T> {
  for (let i = 0; i < elems.length; i++) {
    const elem = elems[i];
    if (elem.type === 'SpreadElement') {
      parser.addDiagnostic({
        message: 'Is this even ever possible?',
        loc: elem.loc,
      });

      elems[i] = toReferenceIdentifier(
        parser.createUnknownIdentifier('spread substitute'),
      );
    }
  }
  // @ts-ignore Technically wrong but we removed all SpreadElement
  return elems;
}

export function toReferencedListDeep(
  parser: JSParser,
  exprList: Array<ToReferencedItem>,
  isParenthesizedExpr?: boolean,
): Array<AnyExpression | SpreadElement> {
  const refList = toReferencedList(parser, exprList, isParenthesizedExpr);
  toReferencedListDeepItems(parser, refList);
  return refList;
}

export function toReferencedListDeepOptional(
  parser: JSParser,
  exprList: Array<undefined | ToReferencedItem>,
  isParenthesizedExpr?: boolean,
): Array<undefined | AnyExpression | SpreadElement> {
  const refList = toReferencedListOptional(
    parser,
    exprList,
    isParenthesizedExpr,
  );
  toReferencedListDeepItems(parser, refList);
  return refList;
}

function toReferencedListDeepItems(
  parser: JSParser,
  exprList: Array<undefined | ToReferencedItem>,
) {
  for (let i = 0; i < exprList.length; i++) {
    const expr = exprList[i];
    if (expr !== undefined && expr.type === 'ArrayExpression') {
      toReferencedListDeepOptional(parser, expr.elements);
    }
  }
}

export function parseSpread(
  parser: JSParser,
  refShorthandDefaultPos?: IndexTracker,
  refNeedsArrowPos?: IndexTracker,
): SpreadElement {
  const start = parser.getPosition();
  parser.next();

  const argument = parseMaybeAssign<AnyExpression>(
    parser,
    'spread argument',
    false,
    refShorthandDefaultPos,
    undefined,
    refNeedsArrowPos,
  );

  if (get0(parser.state.commaAfterSpreadAt) === -1 && parser.match(tt.comma)) {
    parser.state.commaAfterSpreadAt = parser.state.index;
  }

  return {
    loc: parser.finishLoc(start),
    type: 'SpreadElement',
    argument,
  };
}

// Parses lvalue (assignable) atom.
export function parseTargetBindingPattern(
  parser: JSParser,
): AnyTargetBindingPattern {
  switch (parser.state.tokenType) {
    case tt.bracketL:
      return parseArrayPattern(parser);

    case tt.braceL:
      return parseObjectPattern(parser, createIndexTracker());
  }

  return parseBindingIdentifier(parser);
}

function parseArrayPattern(parser: JSParser): BindingArrayPattern {
  const start = parser.getPosition();
  const openContext = parser.expectOpening(
    tt.bracketL,
    tt.bracketR,
    'array pattern',
  );
  const {list: elements, rest} = parseBindingList(parser, openContext, true);
  return {
    loc: parser.finishLoc(start),
    type: 'BindingArrayPattern',
    elements,
    rest,
  };
}

export function parseBindingList(
  parser: JSParser,
  openContext: OpeningContext,
  allowEmpty: boolean = false,
  allowTSModifiers: boolean = false,
): {
  list: Array<undefined | AnyParamBindingPattern>;
  rest: undefined | AnyTargetBindingPattern;
} {
  const elts: Array<undefined | AnyParamBindingPattern> = [];
  let rest: undefined | AnyTargetBindingPattern;

  let first = true;
  while (true) {
    if (parser.match(openContext.close) || parser.match(tt.eof)) {
      parser.expectClosing(openContext);
      break;
    }

    if (first) {
      first = false;
    } else {
      if (!parser.eat(tt.comma)) {
        parser.addDiagnostic({
          message: `Expected a comma to separate items in ${openContext.name}`,
        });
        break;
      }
    }

    if (allowEmpty && parser.match(tt.comma)) {
      elts.push(undefined);
    } else if (parser.match(openContext.close)) {
      parser.expectClosing(openContext);
      break;
    } else if (parser.match(tt.ellipsis)) {
      parser.next();

      rest = parseBindingListItemTypes(
        parser,
        parser.getPosition(),
        parseTargetBindingPattern(parser),
      );

      if (!hasCommaAfterRest(parser)) {
        parser.expectClosing(openContext);
        break;
      }
    } else {
      elts.push(parseBindingListItem(parser, allowTSModifiers));
    }
  }
  return {list: elts, rest};
}

export function parseBindingListNonEmpty(
  parser: JSParser,
  openContext: OpeningContext,
  allowTSModifiers?: boolean,
): {list: Array<AnyBindingPattern>; rest: undefined | AnyTargetBindingPattern} {
  const list = parseBindingList(parser, openContext, false, allowTSModifiers);
  // @ts-ignore: Need to make this more explicit we set `allowEmpty: false` above
  return list;
}

export function parseBindingListItem(
  parser: JSParser,
  allowTSModifiers: boolean,
): AnyParamBindingPattern {
  const start = parser.getPosition();

  let accessibility: undefined | ConstTSAccessibility;
  let readonly = false;
  if (allowTSModifiers) {
    accessibility = parseTSAccessModifier(parser);
    readonly = hasTSModifier(parser, ['readonly']);
  }

  const left = parseBindingListItemTypes(
    parser,
    start,
    parseTargetBindingPattern(parser),
  );
  const elt = parseMaybeDefault(parser, start, left);

  if (accessibility !== undefined || readonly) {
    if (!parser.isSyntaxEnabled('ts')) {
      parser.addDiagnostic({
        message:
          'Accessibility and readonly syntax found but TS is not enabled',
      });
    }

    if (
      elt.type !== 'BindingIdentifier' &&
      elt.type !== 'BindingAssignmentPattern'
    ) {
      parser.addDiagnostic({
        start,
        message:
          'A parameter property may not be declared using a binding pattern.',
      });
    }

    return {
      ...elt,
      loc: parser.finishLoc(start),
      meta: {
        type: 'PatternMeta',
        loc: parser.finishLoc(start),
        accessibility,
        readonly,
      },
    };
  }

  return elt;
}

export function parseBindingListItemTypes(
  parser: JSParser,
  start: Position,
  param: AnyTargetBindingPattern,
): AnyTargetBindingPattern {
  let typeAnnotation;
  let optional;

  if (parser.eat(tt.question)) {
    if (param.type !== 'BindingIdentifier') {
      parser.addDiagnostic({
        loc: param.loc,
        message:
          'A binding pattern parameter cannot be optional in an implementation signature.',
      });
    }

    optional = true;
  }

  if (parser.match(tt.colon)) {
    typeAnnotation = parsePrimaryTypeAnnotation(parser);
  }

  return {
    ...param,
    meta: {
      type: 'PatternMeta',
      loc: parser.finishLoc(start),
      optional,
      typeAnnotation,
    },
  };
}

// Parses assignment pattern around given atom if possible.

export function parseMaybeDefault(
  parser: JSParser,
  start: Position = parser.getPosition(),
  left: AnyTargetBindingPattern = parseTargetBindingPattern(parser),
): AnyTargetBindingPattern | BindingAssignmentPattern {
  let target: AnyBindingPattern;

  if (parser.eat(tt.eq)) {
    const right = parseMaybeAssign<AnyExpression>(
      parser,
      'assignment pattern right',
    );
    const assign: BindingAssignmentPattern = {
      loc: parser.finishLoc(start),
      type: 'BindingAssignmentPattern',
      left,
      right,
    };
    target = assign;
  } else {
    target = left;
  }

  if (
    target.type === 'BindingAssignmentPattern' &&
    target.meta !== undefined &&
    target.meta.typeAnnotation !== undefined &&
    parser.getLoc(target.right).start.index <
      parser.getLoc(target.meta.typeAnnotation).start.index
  ) {
    parser.addDiagnostic({
      loc: target.meta.typeAnnotation.loc,
      message:
        'Type annotations must come before default assignments, e.g. instead of `age = 25: number` use `age: number = 25`',
    });
  }

  return target;
}

const ALLOWED_PARENTHESIZED_LVAL_TYPES = [
  'Identifier',
  'MemberExpression',
  'TSAsExpression',
  'TSTypeAssertion',
  'TSAssignmentTypeAssertion',
  'TSAssignmentAsExpression',
  'TSAssignmentNonNullExpression',
];

// Verify that a node is an lval — something that can be assigned
// to.
export function checkLVal(
  parser: JSParser,
  expr: AnyAssignmentPattern | AnyBindingPattern | AnyExpression,
  maybeIsBinding: undefined | boolean,
  checkClashes: undefined | Map<string, AnyNode>,
  contextDescription: string,
): void {
  const isBinding: boolean =
    maybeIsBinding === undefined ? false : maybeIsBinding;

  // Verify that nodes aren't parenthesized
  if (
    parser.isParenthesized(expr) &&
    !ALLOWED_PARENTHESIZED_LVAL_TYPES.includes(expr.type)
  ) {
    let adviceMsg;
    if (expr.type === 'BindingObjectPattern') {
      adviceMsg = 'Did you use `({a}) = 0` instead of `({a} = 0)`?';
    } else if (expr.type === 'BindingArrayPattern') {
      adviceMsg = 'Did you use `([a]) = 0` instead of `([a] = 0)`?';
    }
    const advice: PartialDiagnosticAdvice = [];
    if (adviceMsg !== undefined) {
      advice.push({
        type: 'log',
        category: 'info',
        message: adviceMsg,
      });
    }

    parser.addDiagnostic({
      message: 'Invalid parenthesized binding',
      advice,
      loc: expr.loc,
    });
  }

  switch (expr.type) {
    case 'FlowTypeCastExpression':
      // Allow 'typecasts' to appear on the left of assignment expressions,
      // because it may be in an arrow function.
      // e.g. `const f = (foo: number = 0) => foo;`
      // This will be validated later
      return undefined;

    case 'TSAsExpression':
    case 'TSNonNullExpression':
    case 'TSTypeAssertion':
      checkLVal(
        parser,
        expr.expression,
        isBinding,
        checkClashes,
        contextDescription,
      );
      return undefined;

    case 'BindingIdentifier':
    case 'ReferenceIdentifier':
    case 'AssignmentIdentifier':
      if (
        parser.inScope('STRICT') &&
        isStrictBindReservedWord(expr.name, parser.inModule)
      ) {
        parser.addDiagnostic({
          loc: expr.loc,
          message: `${expr.name} is a reserved word`,
        });
      }

      if (checkClashes !== undefined) {
        const clash = checkClashes.get(expr.name);

        if (clash === undefined) {
          checkClashes.set(expr.name, expr);
        } else {
          const loc = parser.getLoc(clash);
          parser.addDiagnostic({
            loc: expr.loc,
            message: 'Argument name clash in strict mode',
            advice: [
              {
                type: 'log',
                category: 'info',
                message: 'Collides with this existing definition',
              },
              {
                type: 'frame',
                filename: parser.filename,
                start: loc.start,
                end: loc.end,
              },
            ],
          });
        }
      }
      break;

    case 'AssignmentObjectPattern':
    case 'BindingObjectPattern':
      if (expr.rest !== undefined) {
        checkLVal(parser, expr.rest, isBinding, checkClashes, 'rest property');
      }

      for (let prop of expr.properties) {
        if (prop.type === 'BindingObjectPatternProperty') {
          checkLVal(
            parser,
            prop.value,
            isBinding,
            checkClashes,
            'object destructuring pattern',
          );
        } else {
          checkLVal(
            parser,
            prop,
            isBinding,
            checkClashes,
            'object destructuring pattern',
          );
        }
      }
      break;

    case 'AssignmentObjectPatternProperty':
    case 'BindingObjectPatternProperty':
      break;

    case 'AssignmentArrayPattern':
    case 'BindingArrayPattern':
      if (expr.rest !== undefined) {
        checkLVal(parser, expr.rest, isBinding, checkClashes, 'rest element');
      }

      for (const elem of expr.elements) {
        if (elem) {
          checkLVal(
            parser,
            elem,
            isBinding,
            checkClashes,
            'array destructuring pattern',
          );
        }
      }
      break;

    case 'BindingAssignmentPattern':
      checkLVal(
        parser,
        expr.left,
        isBinding,
        checkClashes,
        'assignment pattern',
      );
      break;
  }
}

export function checkToRestConversion(
  parser: JSParser,
  node: SpreadProperty | SpreadElement,
): void {
  if (VALID_REST_ARGUMENT_TYPES.includes(node.argument.type) === false) {
    parser.addDiagnostic({
      loc: node.argument.loc,
      message: "Invalid rest operator's argument",
    });
  }
}

export function hasCommaAfterRest(parser: JSParser): boolean {
  if (parser.match(tt.comma)) {
    raiseRestNotLast(parser);
    return true;
  }

  return false;
}

export function raiseRestNotLast(
  parser: JSParser,
  loc?: SourceLocation,
  start?: Position,
) {
  parser.addDiagnostic({
    start,
    loc,
    message: `The rest element has to be the last element when destructuring`,
  });
}

export function checkCommaAfterRestFromSpread(parser: JSParser): void {
  if (get0(parser.state.commaAfterSpreadAt) > -1) {
    raiseRestNotLast(
      parser,
      undefined,
      parser.getPositionFromIndex(parser.state.commaAfterSpreadAt),
    );
  }
}
