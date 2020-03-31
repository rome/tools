/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSParser} from '../parser';
import {Position} from '@romejs/parser-core';
import {
  FlowClassImplements,
  AnyPrimaryType,
  TSExpressionWithTypeArguments,
  TSInterfaceDeclaration,
  FlowInterfaceDeclaration,
  AnyFlowDeclare,
  AnyFlowPredicate,
  TypeAliasTypeAnnotation,
  AnyTypeParameter,
  AnyTypeArguments,
  FlowOpaqueType,
  AnyExpression,
  AmbiguousFlowTypeCastExpression,
  AnyLiteralTypeAnnotation,
  AnyTargetAssignmentPattern,
  PatternMeta,
} from '@romejs/js-ast';
import {types as tt} from '../tokenizer/types';
import {
  TSDeclareNode,
  parseFlowTypeAnnotation,
  parseTSTypeAnnotation,
  parseFlowTypeParameterDeclaration,
  parseTSTypeParameters,
  parseFlowTypeParameterInstantiation,
  parseTSTypeArguments,
  parseFlowInterface,
  parseFlowDeclare,
  parseTSTypeAliasTypeAnnotation,
  parseTSInterfaceDeclaration,
  parseTSDeclare,
  parseFlowClassImplemented,
  parseTSHeritageClause,
  parseFlowOpaqueType,
  parseTSEnumDeclaration,
  parseTSAmbientExternalModuleDeclaration,
  parseTSModuleOrNamespaceDeclaration,
  parseTSModuleBlock,
  parseFlowTypeAndPredicateInitialiser,
  parseTSTypeOrTypePredicateAnnotation,
  parseTSAbstractClass,
  parseFlowTypeParameterInstantiationCallOrNew,
  toBindingIdentifier,
  parseFlowTypeAliasTypeAnnotation,
  toTargetAssignmentPattern,
} from './index';
import {descriptions} from '@romejs/diagnostics';

export function isTypeSystemEnabled(parser: JSParser): boolean {
  return parser.isSyntaxEnabled('flow') || parser.isSyntaxEnabled('ts');
}

export function parseTypeLiteralAnnotation(
  parser: JSParser,
): AnyLiteralTypeAnnotation {
  const start = parser.getPosition();

  switch (parser.state.tokenType) {
    case tt.string:
      {
        const value = String(parser.state.tokenValue);
        parser.next();
        return parser.finishNode(start, {
          type: 'StringLiteralTypeAnnotation',
          value,
        });
      }

    case tt.num:
      {
        const value = Number(parser.state.tokenValue);
        parser.next();
        return parser.finishNode(start, {
          type: 'NumericLiteralTypeAnnotation',
          value,
        });
      }

    case tt._true:
    case tt._false:
      {
        const value = parser.match(tt._true);
        parser.next();
        return parser.finishNode(start, {
          type: 'BooleanLiteralTypeAnnotation',
          value,
        });
      }

    case tt.plusMin:
      {
        if (parser.state.tokenValue === '-') {
          parser.next();

          if (!parser.match(tt.num)) {
            parser.addDiagnostic({
              description: descriptions.JS_PARSER.TYPE_NUMERIC_LITERAL_EXPECTED,
            });
            parser.next();
            return parser.finishNode(start, {
              type: 'NumericLiteralTypeAnnotation',
              value: 0,
            });
          }

          const value = Number(parser.state.tokenValue);
          parser.next();
          return parser.finishNode(start, {
            type: 'NumericLiteralTypeAnnotation',
            value: -value,
          });
        } else {
          parser.addDiagnostic({
            description: descriptions.JS_PARSER.TYPE_NUMERIC_LITERAL_PLUS,
          });
          parser.next();

          if (!parser.match(tt.num)) {
            parser.addDiagnostic({
              description: descriptions.JS_PARSER.TYPE_NUMERIC_LITERAL_EXPECTED,
            });
            parser.next();
            return parser.finishNode(start, {
              type: 'NumericLiteralTypeAnnotation',
              value: 0,
            });
          }

          return parseTypeLiteralAnnotation(parser);
        }
      }

    default:
      throw new Error(
        'Caller should have already validated the range of token types',
      );
  }
}

function parseFlowOrTS<F, TS>(
  parser: JSParser,
  label: string,
  flow: (parser: JSParser) => F,
  ts: (parser: JSParser) => TS,
): F | TS {
  if (parser.isSyntaxEnabled('flow')) {
    return flow(parser);
  } else if (parser.isSyntaxEnabled('ts')) {
    return ts(parser);
  } else {
    const branches = parser.createBranch<F | TS>();

    // Suppress disabled syntax errors
    parser.syntax.add('flow');
    branches.add(flow);
    parser.syntax.delete('flow');

    // If we parsed this as Flow syntax, then it's definitely valid Flow syntax, but could also be valid TS syntax since sometimes they intersect
    let isFlowOrTS = branches.hasOptimalBranch();

    // Suppress disabled syntax errors
    parser.syntax.add('ts');
    branches.add(ts);
    parser.syntax.delete('ts');

    // If we didn't pick the Flow branch but picked the TS one, then this could only ever be TS syntax
    let isOnlyTS = !isFlowOrTS && branches.hasOptimalBranch();

    const start = parser.getPosition();
    const node = branches.pick();

    if (isOnlyTS) {
      addTSDiagnostic(parser, label, start);
    } else {
      addFlowOrTSDiagnostic(parser, label, start);
    }

    return node;
  }
}

export function addFlowOrTSDiagnostic(
  parser: JSParser,
  label: string,
  start: Position,
) {
  if (parser.isSyntaxEnabled('ts') || parser.isSyntaxEnabled('flow')) {
    return;
  }

  parser.addDiagnostic({
    start,
    description: descriptions.JS_PARSER.FLOW_OR_TEST_REQUIRED(label),
  });
}

export function addFlowDiagnostic(
  parser: JSParser,
  label: string,
  start: Position,
) {
  if (parser.isSyntaxEnabled('flow')) {
    return;
  }

  parser.addDiagnostic({
    start,
    description: descriptions.JS_PARSER.FLOW_REQUIRED(label),
  });
}

export function addTSDiagnostic(parser: JSParser, label: string, start: Position) {
  if (parser.isSyntaxEnabled('ts')) {
    return;
  }

  parser.addDiagnostic({
    start,
    description: descriptions.JS_PARSER.TS_REQUIRED(label),
  });
}

export function parseClassImplements(
  parser: JSParser,
): Array<FlowClassImplements | TSExpressionWithTypeArguments> {
  return parseFlowOrTS(
    parser,
    'class implements',
    parseFlowClassImplemented,
    () => parseTSHeritageClause(parser, 'implements'),
  );
}

export function parsePrimaryTypeAnnotation(parser: JSParser): AnyPrimaryType {
  return parseFlowOrTS(parser, 'type annotation', parseFlowTypeAnnotation, () =>
    parseTSTypeAnnotation(parser, true)
  );
}

export function parseInterface(
  parser: JSParser,
  start: Position,
): TSInterfaceDeclaration | FlowInterfaceDeclaration {
  parser.addDiagnosticFilter({
    message: 'interface is a reserved word',
    start,
  });

  return parseFlowOrTS(parser, 'interface', () =>
    parseFlowInterface(parser, start), () =>
    parseTSInterfaceDeclaration(parser, start)
  );
}

export function parseDeclare(
  parser: JSParser,
  start: Position,
): TSDeclareNode | AnyFlowDeclare {
  return parseFlowOrTS(parser, 'type declaration', () =>
    parseFlowDeclare(parser, start), () => parseTSDeclare(parser, start)
  );
}

export type TypeAnnotationAndPredicate = [undefined | AnyPrimaryType,
  | undefined
  | AnyFlowPredicate];

export function parseTypeAnnotationAndPredicate(
  parser: JSParser,
): TypeAnnotationAndPredicate {
  return parseFlowOrTS(parser, 'type annotation and a predicate', () => {
    return parseFlowTypeAndPredicateInitialiser(parser);
  }, () => {
    return [
      parseTSTypeOrTypePredicateAnnotation(parser, tt.colon),
      undefined,
    ];
  });
}

export function parseTypeAlias(
  parser: JSParser,
  start: Position,
): TypeAliasTypeAnnotation | TypeAliasTypeAnnotation {
  return parseFlowOrTS(parser, 'type alias', () =>
    parseFlowTypeAliasTypeAnnotation(parser, start), () =>
    parseTSTypeAliasTypeAnnotation(parser, start)
  );
}

export function parseTypeParameters(
  parser: JSParser,
  allowDefault: boolean = false,
): AnyTypeParameter {
  return parseFlowOrTS(parser, 'type parameters', () =>
    parseFlowTypeParameterDeclaration(parser, allowDefault), parseTSTypeParameters
  );
}

export function maybeParseTypeParameters(
  parser: JSParser,
  allowDefault?: boolean,
): undefined | AnyTypeParameter {
  if (parser.isRelational('<')) {
    return parseTypeParameters(parser, allowDefault);
  } else {
    return undefined;
  }
}

export function parseTypeArguments(parser: JSParser): AnyTypeArguments {
  return parseFlowOrTS(
    parser,
    'type arguments',
    parseFlowTypeParameterInstantiation,
    parseTSTypeArguments,
  );
}

export function parseTypeCallArguments(parser: JSParser): AnyTypeArguments {
  return parseFlowOrTS(
    parser,
    'type call arguments',
    parseFlowTypeParameterInstantiationCallOrNew,
    parseTSTypeArguments,
  );
}

export function maybeParseTypeArguments(
  parser: JSParser,
): undefined | AnyTypeArguments {
  if (parser.isRelational('<')) {
    return parseTypeArguments(parser);
  } else {
    return undefined;
  }
}

type TypeExpressionStatement =
  | TSDeclareNode
  | TypeAliasTypeAnnotation
  | AnyFlowDeclare
  | FlowOpaqueType
  | TypeAliasTypeAnnotation
  | TSInterfaceDeclaration
  | FlowInterfaceDeclaration;

export function parseTypeExpressionStatement(
  parser: JSParser,
  start: Position,
  expr: AnyExpression,
): undefined | TypeExpressionStatement {
  // TODO TypeScript does not like parser.isLineTerminator()
  if (expr.type !== 'ReferenceIdentifier') {
    return undefined;
  }

  // In TS, line breaks aren't allowed between the keyword and the rest of the statement

  // In Flow, they are allowed
  if (parser.isSyntaxEnabled('ts') && parser.hasPrecedingLineBreak()) {
    return undefined;
  }

  switch (expr.name) {
    case 'declare':
      if (parser.match(tt._class) || parser.match(tt.name) || parser.match(
        tt._function,
      ) || parser.match(tt._const) || parser.match(tt._var) || parser.match(
        tt._export,
      )) {
        return parseDeclare(parser, start);
      } else {
        break;
      }

    case 'interface':
      // TODO perform some lookahead to make sure we want to do this
      return parseInterface(parser, start);

    case 'type':
      // TODO perform some lookahead to make sure we want to do this
      return parseTypeAlias(parser, start);

    case 'opaque':
      // TODO perform some lookahead to make sure we want to do this
      addFlowDiagnostic(parser, 'opaque type', start);
      return parseFlowOpaqueType(parser, start, false);

    case 'abstract':
      if (parser.match(tt._class)) {
        addTSDiagnostic(parser, 'abstract class', start);
        return parseTSAbstractClass(parser, start);
      } else {
        break;
      }

    case 'enum':
      {
        if (parser.match(tt.name)) {
          addTSDiagnostic(parser, 'enum declaration', start);
          return parseTSEnumDeclaration(parser, start, /* isConst */false);
        } else {
          break;
        }
      }

    case 'module':
      if (parser.match(tt.string)) {
        addTSDiagnostic(parser, 'ambient external module declaration', start);
        return parseTSAmbientExternalModuleDeclaration(parser, start);
      } else if (parser.match(tt.name) && !parser.isLineTerminator()) {
        addTSDiagnostic(parser, 'module or namespace declaration', start);
        return parseTSModuleOrNamespaceDeclaration(parser, start);
      } else {
        break;
      }

    case 'namespace':
      if (!parser.match(tt.name)) {
        return undefined;
      }

      addTSDiagnostic(parser, 'module or namespace declaration', start);
      return parseTSModuleOrNamespaceDeclaration(parser, start);

    // TODO abstract this into typescript.js
    case 'global':
      // `global { }` (with no `declare`) may appear inside an ambient module declaration.

      // Would like to use parseTSAmbientExternalModuleDeclaration here, but already ran past 'global'.
      if (parser.match(tt.braceL)) {
        addTSDiagnostic(parser, 'module declaration', start);
        const global = true;
        const id = toBindingIdentifier(parser, expr);
        const body = parseTSModuleBlock(parser);
        return parser.finishNode(start, {
          type: 'TSModuleDeclaration',
          global,
          id,
          body,
        });
      }
  }

  return undefined;
}

export function ambiguousTypeCastToParameter(
  parser: JSParser,
  node: AmbiguousFlowTypeCastExpression,
): AnyTargetAssignmentPattern {
  const start = parser.getPosition();
  const expr = toTargetAssignmentPattern(parser, node.expression, 'parameter');

  const meta: PatternMeta = parser.finishNode(start, {
    type: 'PatternMeta',
    optional: node.optional,
    typeAnnotation: node.typeAnnotation,
  });

  return parser.finishNode(start, {
    ...expr,
    // @ts-ignore
    meta,
  });
}
