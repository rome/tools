/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {types as tt, TokenType} from '../tokenizer/types';
import {Position, SourceLocation} from '@romejs/parser-core';
import {State} from '../tokenizer/state';
import {JSParser} from '../parser';
import {
  Identifier,
  AnyNode,
  ConstImportModuleKind,
  AnyFlowPredicate,
  FlowTypeParameterInstantiation,
  FlowDeclaredPredicate,
  FlowInferredPredicate,
  FlowDeclareClass,
  FlowDeclareFunction,
  AnyFlowDeclare,
  StringLiteral,
  FlowDeclareVariable,
  FlowDeclareModule,
  BindingIdentifier,
  AnyStatement,
  BlockStatement,
  FlowDeclareModuleExports,
  FlowDeclareInterface,
  FlowDeclareOpaqueType,
  FlowObjectTypeAnnotation,
  FlowInterfaceExtends,
  FlowTypeParameterDeclaration,
  FlowInterfaceDeclaration,
  FlowInterfaceTypeAnnotation,
  TypeAliasTypeAnnotation,
  FlowOpaqueType,
  FlowTypeParameter,
  FlowObjectTypeProperty,
  FlowObjectTypeCallProperty,
  FlowObjectTypeIndexer,
  FlowObjectTypeInternalSlot,
  FlowObjectTypePropertyKey,
  FlowObjectTypePropertyKind,
  ReferenceIdentifier,
  FlowObjectTypeSpreadProperty,
  FlowFunctionTypeAnnotation,
  FlowFunctionTypeParam,
  FlowQualifiedTypeIdentifier,
  AnyFlowPrimary,
  FlowClassImplements,
  AnyFlowKeywordTypeAnnotation,
  FlowGenericTypeAnnotation,
  FlowVariance,
  FlowTypeofTypeAnnotation,
  FlowTupleTypeAnnotation,
  FlowVarianceKind,
  ArrowFunctionExpression,
  FlowDeclareExportDefault,
  FlowDeclareExportNamed,
  FlowDeclareExportAll,
} from '@romejs/js-ast';
import {
  isLetStart,
  parseImport,
  parseIdentifier,
  parseExpression,
  parseExport,
  parseFunctionParams,
  parseArrowHead,
  parseArrowExpression,
  createIdentifier,
  checkGetterSetterParamCount,
  parseNumericLiteral,
  parseStringLiteral,
  addFlowDiagnostic,
  parseBindingIdentifier,
  toReferenceIdentifier,
  toBindingIdentifier,
  parseTypeLiteralAnnotation,
} from './index';
import {get0} from '@romejs/ob1';
import {descriptions} from '@romejs/diagnostics';
import {parseReferenceIdentifier} from './expression';

const primitiveTypes = [
  'any',
  'bool',
  'boolean',
  'empty',
  'false',
  'mixed',
  'null',
  'number',
  'bigint',
  'static',
  'string',
  'true',
  'typeof',
  'void',
  'interface',
  'extends',
  '_',
];

const exportSuggestions: Map<string, string> = new Map([
  ['const', 'declare export var'],
  ['let', 'declare export var'],
  ['type', 'export type'],
  ['interface', 'export interface'],
]);

function checkNotUnderscore(parser: JSParser, id: Identifier) {
  if (id.name === '_') {
    parser.addDiagnostic({
      loc: id.loc,
      description: descriptions.JS_PARSER.FLOW_BAD_UNDERSCORE_NAME,
    });
  }
}

function isEsModuleType(bodyElement: AnyNode): boolean {
  return bodyElement.type === 'ExportAllDeclaration' || bodyElement.type ===
  'ExportLocalDeclaration' && (!bodyElement.declaration ||
  bodyElement.declaration.type !== 'TypeAliasTypeAnnotation' &&
    bodyElement.declaration.type !== 'FlowInterfaceDeclaration');
}

export function hasTypeImportKind(
  kind: undefined | ConstImportModuleKind,
): boolean {
  return kind === 'type' || kind === 'typeof';
}

export function isMaybeDefaultImport(state: State): boolean {
  return (state.tokenType === tt.name || !!state.tokenType.keyword) &&
    state.tokenValue !== 'from';
}

export function parseFlowTypeParameterInstantiationCallOrNew(
  parser: JSParser,
): FlowTypeParameterInstantiation {
  const start = parser.getPosition();
  const params = [];

  parser.pushScope('TYPE', true);

  if (parser.expectRelational('<')) {
    while (!parser.isRelational('>')) {
      params.push(parseFlowTypeOrImplicitInstantiation(parser));

      if (!parser.isRelational('>') && !parser.expect(tt.comma)) {
        break;
      }
    }
    parser.expectRelational('>');
  }

  parser.popScope('TYPE');

  return parser.finishNode(start, {
    type: 'FlowTypeParameterInstantiation',
    params,
  });
}

function parseFlowTypeOrImplicitInstantiation(parser: JSParser): AnyFlowPrimary {
  if (parser.state.tokenType === tt.name && parser.state.tokenValue === '_') {
    const startPos = parser.state.startPos;
    const node = parseReferenceIdentifier(parser);
    return parseFlowGenericType(parser, startPos, node);
  } else {
    return parseFlowType(parser);
  }
}

function parseFlowTypeInitialiser(
  parser: JSParser,
  tok?: TokenType,
): AnyFlowPrimary {
  parser.pushScope('TYPE', true);
  parser.expect(tok || tt.colon);
  const type = parseFlowType(parser);
  parser.popScope('TYPE');
  return type;
}

function parseFlowPredicate(
  parser: JSParser,
): FlowDeclaredPredicate | FlowInferredPredicate {
  const start = parser.getPosition();
  const moduloPos = parser.state.startPos;
  parser.expect(tt.modulo);
  const checksPos = parser.state.startPos;
  parser.expectContextual('checks');

  // Force '%' and 'checks' to be adjacent
  if (moduloPos.line !== checksPos.line || get0(moduloPos.column) !== get0(
    checksPos.column,
  ) - 1) {
    parser.addDiagnostic({
      start: moduloPos,
      description: descriptions.JS_PARSER.FLOW_SPACE_BETWEEN_PERCENT_CHECKS,
    });
  }

  if (parser.match(tt.parenL)) {
    const openContext = parser.expectOpening(
      tt.parenL,
      tt.parenR,
      'flow declared predicate',
    );
    const value = parseExpression(parser, 'flow declared predicate');
    parser.expectClosing(openContext);
    return parser.finishNode(start, {
      type: 'FlowDeclaredPredicate',
      value,
    });
  } else {
    return parser.finishNode(start, {
      type: 'FlowInferredPredicate',
    });
  }
}

export function parseFlowTypeAndPredicateInitialiser(
  parser: JSParser,
): [undefined | AnyFlowPrimary, undefined | AnyFlowPredicate] {
  addFlowDiagnostic(
    parser,
    ' flow type and predicate initializer',
    parser.getPosition(),
  );

  parser.pushScope('TYPE', true);
  parser.expect(tt.colon);
  let type = undefined;
  let predicate = undefined;
  if (parser.match(tt.modulo)) {
    parser.popScope('TYPE');
    predicate = parseFlowPredicate(parser);
  } else {
    type = parseFlowType(parser);
    parser.popScope('TYPE');
    if (parser.match(tt.modulo)) {
      predicate = parseFlowPredicate(parser);
    }
  }
  return [type, predicate];
}

function parseFlowDeclareClass(
  parser: JSParser,
  start: Position,
): FlowDeclareClass {
  parser.next();
  return parser.finishNode(start, {
    ...parseFlowInterfaceish(parser, true),
    type: 'FlowDeclareClass',
  });
}

function parseFlowDeclareFunction(
  parser: JSParser,
  start: Position,
): FlowDeclareFunction {
  parser.next();

  const id = parseIdentifier(parser);

  let typeParameters = undefined;
  if (parser.isRelational('<')) {
    typeParameters = parseFlowTypeParameterDeclaration(parser, true);
  }

  const openContext = parser.expectOpening(
    tt.parenL,
    tt.parenR,
    'flow function type params',
  );
  const {params, rest} = parseFlowFunctionTypeParams(parser);
  parser.expectClosing(openContext);

  const [returnType, predicate] = parseFlowTypeAndPredicateInitialiser(parser);

  parser.semicolon();

  if (predicate !== undefined && predicate.type === 'FlowInferredPredicate') {
    parser.addDiagnostic({
      loc: predicate.loc,
      description: descriptions.JS_PARSER.FLOW_UNINFERRABLE_PREDICATE_ON_FUNCTION,
    });
  }

  return parser.finishNode(start, {
    type: 'FlowDeclareFunction',
    id: parser.finishNode(start, {
      type: 'BindingIdentifier',
      name: id.name,

      meta: parser.finishNode(start, {
        type: 'PatternMeta',
        typeAnnotation: parser.finishNode(start, {
          type: 'FlowFunctionTypeAnnotation',
          params,
          rest,
          returnType,
          typeParameters,
        }),
      }),
    }),
    predicate,
  });
}

export function parseFlowDeclare(
  parser: JSParser,
  start: Position,
  insideModule: boolean = false,
): AnyFlowDeclare {
  addFlowDiagnostic(parser, 'type declaration', start);

  if (parser.match(tt._class)) {
    return parseFlowDeclareClass(parser, start);
  }

  if (parser.match(tt._function)) {
    return parseFlowDeclareFunction(parser, start);
  }

  if (parser.match(tt._var)) {
    return parseFlowDeclareVariable(parser, start);
  }

  if (parser.isContextual('module')) {
    if (parser.lookaheadState().tokenType === tt.dot) {
      return parseFlowDeclareModuleExports(parser, start);
    } else {
      if (insideModule) {
        parser.addDiagnostic({
          description: descriptions.JS_PARSER.FLOW_DECLARE_MODULE_IN_DECLARE_MODULE,
        });
      }
      return parseFlowDeclareModule(parser, start);
    }
  }

  if (parser.isContextual('type')) {
    return parseFlowDeclareTypeAlias(parser, start);
  }

  if (parser.isContextual('opaque')) {
    return parseFlowDeclareOpaqueType(parser, start);
  }

  if (parser.isContextual('interface')) {
    return parseFlowDeclareInterface(parser, start);
  }

  if (parser.match(tt._export)) {
    return parseExportLocalDeclaration(parser, start, insideModule);
  }

  parser.addDiagnostic({
    description: descriptions.JS_PARSER.FLOW_UNKNOWN_DECLARATION_START,
  });

  // Fake node
  return parser.finishNode(start, {
    type: 'FlowDeclareVariable',
    id: toBindingIdentifier(parser, parser.createUnknownIdentifier(
      'flow declaration',
      start,
    )),
  });
}

function parseFlowDeclareVariable(
  parser: JSParser,
  start: Position,
): FlowDeclareVariable {
  parser.next();
  const id = parseFlowTypeAnnotatableIdentifier(
    parser,
    /*allowPrimitiveOverride*/true,
  );
  parser.semicolon();
  return parser.finishNode(start, {
    type: 'FlowDeclareVariable',
    id,
  });
}

function parseFlowDeclareModule(
  parser: JSParser,
  start: Position,
): FlowDeclareModule {
  // Eat `module` token
  parser.next();

  let id: StringLiteral | BindingIdentifier;
  if (parser.match(tt.string)) {
    id = parseStringLiteral(parser);
  } else {
    id = parseBindingIdentifier(parser);
  }

  const bodyStart = parser.getPosition();
  const body: Array<AnyStatement> = [];
  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'flow declare module body',
  );

  while (!parser.match(tt.braceR)) {
    let bodyNodeStart = parser.getPosition();

    let bodyNode;
    if (parser.match(tt._import)) {
      const lookahead = parser.lookaheadState();
      if (lookahead.tokenValue !== 'type' && lookahead.tokenValue !== 'typeof') {
        parser.addDiagnostic({
          description: descriptions.JS_PARSER.FLOW_IMPORT_KINDLESS_IN_DECLARE_MODULE,
        });
      }
      parser.next();
      bodyNode = parseImport(parser, bodyNodeStart);
    } else {
      if (!parser.expectContextual(
        'declare',
        descriptions.JS_PARSER.FLOW_DECLARE_MODULE_INVALID_CHILD,
      )) {
        break;
      }

      bodyNode = parseFlowDeclare(parser, bodyNodeStart, true);
    }

    body.push(bodyNode);
  }

  parser.expectClosing(openContext);

  const bodyNode: BlockStatement = parser.finishNode(bodyStart, {
    type: 'BlockStatement',
    body,
  });

  let kind: undefined | 'commonjs' | 'es';
  let hasModuleExport = false;

  for (const bodyElement of body) {
    if (isEsModuleType(bodyElement)) {
      if (kind === 'commonjs') {
        parser.addDiagnostic({
          loc: bodyElement.loc,
          description: descriptions.JS_PARSER.FLOW_MIXED_DECLARE_EXPORTS,
        });
      }
      kind = 'es';
    } else if (bodyElement.type === 'FlowDeclareModuleExports') {
      if (hasModuleExport) {
        parser.addDiagnostic({
          loc: bodyElement.loc,
          description: descriptions.JS_PARSER.FLOW_DUPLICATE_DECLARE_MODULE_EXPORTS,
        });
      }

      if (kind === 'es') {
        parser.addDiagnostic({
          loc: bodyElement.loc,
          description: descriptions.JS_PARSER.FLOW_MIXED_DECLARE_EXPORTS,
        });
      }

      kind = 'commonjs';
      hasModuleExport = true;
    }
  }

  return parser.finishNode(start, {
    type: 'FlowDeclareModule',
    id,
    kind: kind === undefined ? 'commonjs' : kind,
    body: bodyNode,
  });
}

function parseExportLocalDeclaration(
  parser: JSParser,
  start: Position,
  insideModule: undefined | boolean,
): FlowDeclareExportDefault | FlowDeclareExportNamed | FlowDeclareExportAll {
  parser.expect(tt._export);

  if (parser.eat(tt._default)) {
    let declaration;
    if (parser.match(tt._function) || parser.match(tt._class)) {
      // declare export default class ...

      // declare export default function ...
      declaration = parseFlowDeclare(parser, parser.getPosition());
    } else {
      // declare export default [type];
      declaration = parseFlowType(parser);
      parser.semicolon();
    }

    return parser.finishNode(start, {
      type: 'FlowDeclareExportDefault',
      declaration,
    });
  } else {
    if (parser.match(tt._const) || isLetStart(parser) || (parser.isContextual(
      'type',
    ) || parser.isContextual('interface')) && !insideModule) {
      const label = String(parser.state.tokenValue);
      const suggestion = String(exportSuggestions.get(label));
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.FLOW_DECLARE_EXPORT_UNSUPPORTED(
          label,
          suggestion,
        ),
      });
    }

    if ( // declare export var ...
    parser.match(tt._var) || // declare function ...
    parser.match(tt._function) || // declare export class ...
    parser.match(tt._class) || // declare export opaque ..
    parser.isContextual('opaque')) {
      const declaration = parseFlowDeclare(parser, parser.getPosition());

      return parser.finishNode(start, {
        type: 'FlowDeclareExportNamed',
        declaration,
      });
    }

    if ( // declare export * from '';
    parser.match(tt.star) || // declare export {} ...
    parser.match(tt.braceL) || // declare export interface ...
    parser.isContextual('interface') || // declare export type ...
    parser.isContextual('type') || // declare export opaque type ...
    parser.isContextual('opaque')) {
      const node = parseExport(parser, start);

      if (node !== undefined) {
        if (node.type === 'ExportLocalDeclaration' || node.type ===
        'ExportExternalDeclaration') {
          return {
            ...node,
            type: 'FlowDeclareExportNamed',
          };
        } else if (node.type === 'ExportAllDeclaration') {
          return {
            ...node,
            type: 'FlowDeclareExportAll',
          };
        }
      }
    }
  }

  parser.addDiagnostic({
    start,
    description: descriptions.JS_PARSER.FLOW_UNKNOWN_DECLARE_EXPORT_START,
  });

  // Fake node
  return parser.finishNode(start, {
    type: 'FlowDeclareExportDefault',
    declaration: {
      ...parser.createUnknownStringLiteral(
        'flow declare export declaration',
        start,
      ),
      type: 'StringLiteralTypeAnnotation',
    },
  });
}

function parseFlowDeclareModuleExports(
  parser: JSParser,
  start: Position,
): FlowDeclareModuleExports {
  parser.expectContextual('module');
  parser.expect(tt.dot);
  parser.expectContextual('exports');
  const typeAnnotation = parseFlowTypeAnnotation(parser);
  parser.semicolon();

  return parser.finishNode(start, {
    type: 'FlowDeclareModuleExports',
    typeAnnotation,
  });
}

function parseFlowDeclareTypeAlias(
  parser: JSParser,
  start: Position,
): TypeAliasTypeAnnotation {
  parser.next();
  return {
    ...parseFlowTypeAliasTypeAnnotation(parser, start),
    declare: true,
  };
}

function parseFlowDeclareOpaqueType(
  parser: JSParser,
  start: Position,
): FlowDeclareOpaqueType {
  parser.next();
  const opaque = parseFlowOpaqueType(parser, start, true);
  return parser.finishNode(start, {
    ...opaque,
    type: 'FlowDeclareOpaqueType',
  });
}

function parseFlowDeclareInterface(
  parser: JSParser,
  start: Position,
): FlowDeclareInterface {
  parser.next();
  return parser.finishNode(start, {
    ...parseFlowInterfaceish(parser),
    type: 'FlowDeclareInterface',
  });
}

// Interfaces
function parseFlowInterfaceish(
  parser: JSParser,
  isClass: boolean = false,
): {
  body: FlowObjectTypeAnnotation;
  extends: Array<FlowInterfaceExtends>;
  mixins: Array<FlowInterfaceExtends>;
  implements: Array<FlowInterfaceExtends>;
  id: BindingIdentifier;
  typeParameters: undefined | FlowTypeParameterDeclaration;
} {
  const id = parseFlowRestrictedIdentifier(parser, /*liberal*/!isClass);

  let typeParameters = undefined;
  if (parser.isRelational('<')) {
    typeParameters = parseFlowTypeParameterDeclaration(parser, true);
  }

  const _extends = [];
  const mixins = [];
  const _implements = [];

  if (parser.eat(tt._extends)) {
    do {
      _extends.push(parseFlowInterfaceExtends(parser));
    } while (!isClass && parser.eat(tt.comma));
  }

  if (parser.isContextual('mixins')) {
    parser.next();
    do {
      mixins.push(parseFlowInterfaceExtends(parser));
    } while (parser.eat(tt.comma));
  }

  if (parser.isContextual('implements')) {
    parser.next();
    do {
      _implements.push(parseFlowInterfaceExtends(parser));
    } while (parser.eat(tt.comma));
  }

  const body = parseFlowObjectType(parser, {
    allowStatic: isClass,
    allowExact: false,
    allowSpread: false,
    allowProto: isClass,
    allowInexact: false,
  });
  return {
    body,
    extends: _extends,
    mixins,
    id,
    typeParameters,
    implements: _implements,
  };
}

function parseFlowInterfaceType(parser: JSParser): FlowInterfaceTypeAnnotation {
  const start = parser.getPosition();
  parser.expectContextual('interface');

  const _extends = [];
  if (parser.eat(tt._extends)) {
    do {
      _extends.push(parseFlowInterfaceExtends(parser));
    } while (parser.eat(tt.comma));
  }

  const body = parseFlowObjectType(parser, {
    allowStatic: false,
    allowExact: false,
    allowSpread: false,
    allowProto: false,
    allowInexact: false,
  });

  return parser.finishNode(start, {
    type: 'FlowInterfaceTypeAnnotation',
    extends: _extends,
    body,
  });
}

function parseFlowInterfaceExtends(parser: JSParser): FlowInterfaceExtends {
  const start = parser.getPosition();

  const id = parseFlowQualifiedTypeIdentifier(parser);
  let typeParameters = undefined;
  if (parser.isRelational('<')) {
    typeParameters = parseFlowTypeParameterInstantiation(parser);
  }

  return parser.finishNode(start, {
    type: 'FlowInterfaceExtends',
    id,
    typeParameters,
  });
}

export function parseFlowInterface(
  parser: JSParser,
  start: Position,
): FlowInterfaceDeclaration {
  addFlowDiagnostic(parser, 'interface declaration', start);
  return {
    ...parseFlowInterfaceish(parser),
    loc: parser.finishLoc(start),
    type: 'FlowInterfaceDeclaration',
  };
}

export function checkReservedType(
  parser: JSParser,
  word: string,
  loc: SourceLocation,
) {
  if (primitiveTypes.includes(word)) {
    parser.addDiagnostic({
      loc,
      description: descriptions.JS_PARSER.FLOW_RESERVED_TYPE(word),
    });
  }
}

export function parseFlowRestrictedIdentifier(
  parser: JSParser,
  liberal?: boolean,
): BindingIdentifier {
  checkReservedType(parser, String(parser.state.tokenValue), parser.finishLocAt(
    parser.state.startPos,
    parser.state.endPos,
  ));
  return parseBindingIdentifier(parser, liberal);
}

// Type aliases
export function parseFlowTypeAliasTypeAnnotation(
  parser: JSParser,
  start: Position,
): TypeAliasTypeAnnotation {
  addFlowDiagnostic(parser, 'type alias', start);

  const id = parseFlowRestrictedIdentifier(parser);
  let typeParameters;

  if (parser.isRelational('<')) {
    typeParameters = parseFlowTypeParameterDeclaration(parser, true);
  } else {
    typeParameters = undefined;
  }

  const right = parseFlowTypeInitialiser(parser, tt.eq);
  parser.semicolon();

  return parser.finishNode(start, {
    type: 'TypeAliasTypeAnnotation',
    id,
    typeParameters,
    right,
  });
}

export function parseFlowOpaqueType(
  parser: JSParser,
  start: Position,
  declare: boolean,
): FlowOpaqueType {
  addFlowDiagnostic(parser, 'opaque type', start);
  parser.expectContextual('type');
  const id = parseFlowRestrictedIdentifier(parser, /*liberal*/true);

  let typeParameters;
  if (parser.isRelational('<')) {
    typeParameters = parseFlowTypeParameterDeclaration(parser, true);
  } else {
    typeParameters = undefined;
  }

  // Parse the supertype
  let supertype = undefined;
  if (parser.match(tt.colon)) {
    supertype = parseFlowTypeInitialiser(parser, tt.colon);
  }

  let impltype = undefined;
  if (!declare) {
    impltype = parseFlowTypeInitialiser(parser, tt.eq);
  }
  parser.semicolon();

  return parser.finishNode(start, {
    type: 'FlowOpaqueType',
    id,
    typeParameters,
    supertype,
    impltype,
  });
}

function parseFlowTypeParameter(
  parser: JSParser,
  allowDefault: boolean = true,
  requireDefault: boolean = false,
): FlowTypeParameter {
  const start = parser.getPosition();

  const variance = parseFlowVariance(parser);

  const ident = parseFlowTypeAnnotatableIdentifier(parser);
  const name = ident.name;
  const bound = ident.meta !== undefined ? ident.meta.typeAnnotation : undefined;

  let def;
  if (parser.match(tt.eq)) {
    if (!allowDefault) {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.FLOW_DISALLOW_DEFAULT_TYPE_PARAMETER,
      });
    }

    parser.eat(tt.eq);
    def = parseFlowType(parser);
  } else if (requireDefault) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.FLOW_DEFAULT_TYPE_PARAMETER_REQUIRED,
    });
  }

  return parser.finishNode(start, {
    type: 'FlowTypeParameter',
    default: def,
    name,
    variance,
    bound,
  });
}

export function parseFlowTypeParameterDeclaration(
  parser: JSParser,
  allowDefault: boolean,
): FlowTypeParameterDeclaration {
  const start = parser.getPosition();
  addFlowDiagnostic(parser, 'type parameter declaration', start);

  const params = [];

  parser.pushScope('TYPE', true);

  parser.expectRelational('<');

  let defaultRequired = false;

  do {
    const param = parseFlowTypeParameter(parser, allowDefault, defaultRequired);

    if (param.default) {
      defaultRequired = true;
    }

    params.push(param);

    if (!parser.isRelational('>') && !parser.expect(tt.comma)) {
      break;
    }
  } while (!parser.isRelational('>'));

  parser.expectRelational('>');

  parser.popScope('TYPE');

  return parser.finishNode(start, {
    type: 'FlowTypeParameterDeclaration',
    params,
  });
}

export function parseFlowTypeParameterInstantiation(
  parser: JSParser,
): FlowTypeParameterInstantiation {
  const start = parser.getPosition();
  addFlowDiagnostic(parser, 'type parameter instantiation', start);

  const params = [];

  parser.pushScope('TYPE', true);

  parser.expectRelational('<');
  const oldNoAnonFunctionType = parser.state.noAnonFunctionType;
  parser.state.noAnonFunctionType = false;

  while (!parser.isRelational('>')) {
    params.push(parseFlowType(parser));

    if (!parser.isRelational('>') && !parser.expect(tt.comma)) {
      break;
    }
  }

  parser.state.noAnonFunctionType = oldNoAnonFunctionType;
  parser.expectRelational('>');

  parser.popScope('TYPE');

  return parser.finishNode(start, {
    type: 'FlowTypeParameterInstantiation',
    params,
  });
}

function parseFlowObjectPropertyKey(parser: JSParser): FlowObjectTypePropertyKey {
  if (parser.match(tt.num)) {
    return parseNumericLiteral(parser);
  } else if (parser.match(tt.string)) {
    return parseStringLiteral(parser);
  } else {
    return parseIdentifier(parser, true);
  }
}

function parseFlowObjectTypeIndexer(
  parser: JSParser,
  start: Position,
  isStatic: boolean,
  variance: undefined | FlowVariance,
): FlowObjectTypeIndexer {
  let id;
  let key;

  // Note: bracketL has already been consumed
  if (parser.lookaheadState().tokenType === tt.colon) {
    id = parseFlowObjectPropertyKey(parser);
    key = parseFlowTypeInitialiser(parser);
  } else {
    id = undefined;
    key = parseFlowType(parser);
  }
  parser.expect(tt.bracketR);
  const value = parseFlowTypeInitialiser(parser);

  return parser.finishNode(start, {
    type: 'FlowObjectTypeIndexer',
    static: isStatic,
    key,
    id,
    value,
    variance,
  });
}

function parseFlowObjectTypeMethodish(
  parser: JSParser,
  start: Position,
): FlowFunctionTypeAnnotation {
  let typeParameters = undefined;

  if (parser.isRelational('<')) {
    typeParameters = parseFlowTypeParameterDeclaration(parser, false);
  }

  const openContext = parser.expectOpening(
    tt.parenL,
    tt.parenR,
    'function object method params',
  );
  const {params, rest} = parseFlowFunctionTypeParams(parser);
  parser.expectClosing(openContext);
  const returnType = parseFlowTypeInitialiser(parser);

  return parser.finishNode(start, {
    type: 'FlowFunctionTypeAnnotation',
    params,
    rest,
    typeParameters,
    returnType,
  });
}

function parseFlowObjectTypeCallProperty(
  parser: JSParser,
  start: Position,
  isStatic: boolean,
): FlowObjectTypeCallProperty {
  const valueNode = parser.getPosition();
  const value = parseFlowObjectTypeMethodish(parser, valueNode);
  return parser.finishNode(start, {
    type: 'FlowObjectTypeCallProperty',
    static: isStatic,
    value,
  });
}

function parseFlowObjectType(
  parser: JSParser,
  opts: {
    allowStatic: boolean;
    allowExact: boolean;
    allowSpread: boolean;
    allowProto: boolean;
    allowInexact: boolean;
  },
): FlowObjectTypeAnnotation {
  const {allowExact, allowSpread, allowProto, allowInexact} = opts;
  let {allowStatic} = opts;

  parser.pushScope('TYPE', true);

  const start = parser.getPosition();

  const properties = [];

  let openContext;
  let exact;
  let inexact;
  if (allowExact && parser.match(tt.braceBarL)) {
    openContext = parser.expectOpening(
      tt.braceBarL,
      tt.braceBarR,
      'flow exact object',
    );
    exact = true;
  } else {
    openContext = parser.expectOpening(tt.braceL, tt.braceR, 'flow object');
    exact = false;
  }

  while (true) {
    if (parser.match(tt.eof) || parser.match(openContext.close)) {
      break;
    }

    const start = parser.getPosition();
    let isStatic = false;
    let protoStart: undefined | Position = undefined;

    if (allowProto && parser.isContextual('proto')) {
      const lookahead = parser.lookaheadState();

      if (lookahead.tokenType !== tt.colon && lookahead.tokenType !== tt.question) {
        parser.next();
        protoStart = parser.state.startPos;
        allowStatic = false;
      }
    }

    if (allowStatic && parser.isContextual('static')) {
      const lookahead = parser.lookaheadState();

      // static is a valid identifier name
      if (lookahead.tokenType !== tt.colon && lookahead.tokenType !== tt.question) {
        parser.next();
        isStatic = true;
      }
    }

    const variance = parseFlowVariance(parser);

    if (parser.eat(tt.bracketL)) {
      if (protoStart !== undefined) {
        parser.unexpectedToken(protoStart);
      }

      if (parser.eat(tt.bracketL)) {
        if (variance) {
          parser.addDiagnostic({
            loc: variance.loc,
            description: descriptions.JS_PARSER.ILLEGAL_VARIANCE,
          });
        }

        properties.push(parseFlowObjectTypeInternalSlot(parser, start, isStatic));
      } else {
        properties.push(parseFlowObjectTypeIndexer(
          parser,
          start,
          isStatic,
          variance,
        ));
      }
    } else if (parser.match(tt.parenL) || parser.isRelational('<')) {
      if (protoStart !== undefined) {
        parser.unexpectedToken(protoStart);
      }

      if (variance) {
        parser.addDiagnostic({
          loc: variance.loc,
          description: descriptions.JS_PARSER.ILLEGAL_VARIANCE,
        });
      }

      properties.push(parseFlowObjectTypeCallProperty(parser, start, isStatic));
    } else {
      let kind: FlowObjectTypePropertyKind = 'init';

      if (parser.isContextual('get') || parser.isContextual('set')) {
        const lookahead = parser.lookaheadState();
        if (
          lookahead.tokenType === tt.name || lookahead.tokenType === tt.string ||
          lookahead.tokenType === tt.num
        ) {
          const value = String(parser.state.tokenValue);
          if (value !== 'get' && value !== 'set') {
            throw new Error(
              'Expected get or set as we already validated it above',
            );
          }
          kind = value;
          parser.next();
        }
      }

      const propOrInexact = parseFlowObjectTypeProperty(parser, {
        start,
        isStatic,
        protoStart,
        variance,
        kind,
        allowSpread,
        allowInexact,
      });

      if (propOrInexact === undefined) {
        inexact = true;
      } else {
        properties.push(propOrInexact);
      }
    }

    flowObjectTypeSemicolon(parser);
  }

  parser.expectClosing(openContext);
  parser.popScope('TYPE');

  return parser.finishNode(start, {
    type: 'FlowObjectTypeAnnotation',
    properties,
    exact,
    inexact,
  });
}

function parseFlowObjectTypeProperty(
  parser: JSParser,
  opts: {
    start: Position;
    protoStart: undefined | Position;
    isStatic: boolean;
    variance: undefined | FlowVariance;
    kind: FlowObjectTypePropertyKind;
    allowSpread: boolean;
    allowInexact: boolean;
  },
): undefined | FlowObjectTypeProperty | FlowObjectTypeSpreadProperty {
  const {
    start,
    isStatic,
    protoStart,
    variance,
    kind,
    allowSpread,
    allowInexact,
  } = opts;

  if (parser.match(tt.ellipsis)) {
    if (!allowSpread) {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.FLOW_DISALLOWED_SPREAD,
      });
    }

    if (protoStart !== undefined) {
      parser.unexpectedToken(protoStart);
    }

    if (variance) {
      parser.addDiagnostic({
        loc: variance.loc,
        description: descriptions.JS_PARSER.ILLEGAL_VARIANCE,
      });
    }

    parser.expect(tt.ellipsis);

    const isInexactToken = parser.eat(tt.comma) || parser.eat(tt.semi);

    if (parser.match(tt.braceR)) {
      if (allowInexact) {
        return undefined;
      }

      parser.addDiagnostic({
        description: descriptions.JS_PARSER.FLOW_INEXACT_SYNTAX_NOT_ALLOWED,
      });
    }

    if (parser.match(tt.braceBarR)) {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.FLOW_INEXACT_CANNOT_APPEAR_IN_EXPLICIT_EXACT,
      });
    }

    if (isInexactToken) {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.FLOW_INEXACT_MUST_BE_AT_END,
      });
    }

    const argument = parseFlowType(parser);
    return parser.finishNode(start, {
      type: 'FlowObjectTypeSpreadProperty',
      argument,
    });
  } else {
    const proto = protoStart !== undefined;
    const key = parseFlowObjectPropertyKey(parser);
    let value = undefined;
    let optional = false;

    if (parser.isRelational('<') || parser.match(tt.parenL)) {
      if (protoStart !== undefined) {
        parser.unexpectedToken(protoStart);
      }

      if (variance) {
        parser.addDiagnostic({
          loc: variance.loc,
          description: descriptions.JS_PARSER.ILLEGAL_VARIANCE,
        });
      }

      value = parseFlowObjectTypeMethodish(parser, start);

      if (kind === 'get' || kind === 'set') {
        checkGetterSetterParamCount(parser, value, kind);
      }
    } else {
      if (kind !== 'init') {
        parser.unexpectedToken();
      }

      if (parser.eat(tt.question)) {
        optional = true;
      }
      value = parseFlowTypeInitialiser(parser);
    }

    return parser.finishNode(start, {
      type: 'FlowObjectTypeProperty',
      key,
      static: isStatic,
      kind,
      value,
      variance,
      optional,
      proto,
    });
  }
}

function flowObjectTypeSemicolon(parser: JSParser): void {
  if (
    !parser.eat(tt.semi) && !parser.eat(tt.comma) && !parser.match(tt.braceR) &&
      !parser.match(tt.braceBarR)
  ) {
    parser.unexpectedToken();
  }
}

function parseFlowQualifiedTypeIdentifier(
  parser: JSParser,
  start: Position = parser.getPosition(),
  id?: ReferenceIdentifier,
): ReferenceIdentifier | FlowQualifiedTypeIdentifier {
  let node: ReferenceIdentifier | FlowQualifiedTypeIdentifier = id === undefined
    ? parseReferenceIdentifier(parser) : id;

  while (parser.eat(tt.dot)) {
    const id = parseIdentifier(parser);
    node = parser.finishNode(start, {
      type: 'FlowQualifiedTypeIdentifier',
      id,
      qualification: node,
    });
  }

  return node;
}

function parseFlowGenericType(
  parser: JSParser,
  start: Position,
  _id: ReferenceIdentifier,
): FlowGenericTypeAnnotation | FlowGenericTypeAnnotation {
  let typeParameters = undefined;
  const id = parseFlowQualifiedTypeIdentifier(parser, start, _id);

  if (parser.isRelational('<')) {
    typeParameters = parseFlowTypeParameterInstantiation(parser);
  }

  return parser.finishNode(start, {
    type: 'FlowGenericTypeAnnotation',
    id,
    typeParameters,
  });
}

function parseFlowTypeofType(parser: JSParser): FlowTypeofTypeAnnotation {
  const start = parser.getPosition();
  parser.expect(tt._typeof);
  const argument = parseFlowPrimaryType(parser);
  return parser.finishNode(start, {
    type: 'FlowTypeofTypeAnnotation',
    argument,
  });
}

function parseFlowTupleType(parser: JSParser): FlowTupleTypeAnnotation {
  const start = parser.getPosition();
  const types = [];
  const openContext = parser.expectOpening(
    tt.bracketL,
    tt.bracketR,
    'flow tuple type',
  );
  // We allow trailing commas

  while (parser.state.index < parser.length && !parser.match(tt.bracketR)) {
    types.push(parseFlowType(parser));
    if (parser.match(tt.bracketR)) {
      break;
    }

    if (!parser.expect(tt.comma)) {
      break;
    }
  }
  parser.expectClosing(openContext);
  return parser.finishNode(start, {
    type: 'FlowTupleTypeAnnotation',
    types,
  });
}

function parseFlowFunctionTypeParam(parser: JSParser): FlowFunctionTypeParam {
  let name = undefined;
  let optional = false;
  let typeAnnotation = undefined;
  const start = parser.getPosition();
  const lh = parser.lookaheadState();
  if (lh.tokenType === tt.colon || lh.tokenType === tt.question) {
    name = parseIdentifier(parser);
    if (parser.eat(tt.question)) {
      optional = true;
    }
    typeAnnotation = parseFlowTypeInitialiser(parser);
  } else {
    typeAnnotation = parseFlowType(parser);
  }
  return parser.finishNode(start, {
    type: 'FlowFunctionTypeParam',
    name,
    meta: parser.finishNode(start, {
      type: 'PatternMeta',
      optional,
      typeAnnotation,
    }),
  });
}

function reinterpretTypeAsFlowFunctionTypeParam(
  parser: JSParser,
  type: AnyFlowPrimary,
): FlowFunctionTypeParam {
  const loc = parser.finishLoc(parser.getLoc(type).start);
  return {
    type: 'FlowFunctionTypeParam',
    loc,
    name: undefined,
    meta: {
      type: 'PatternMeta',
      loc,
      optional: false,
      typeAnnotation: type,
    },
  };
}

function parseFlowFunctionTypeParams(
  parser: JSParser,
): {
  params: Array<FlowFunctionTypeParam>;
  rest: undefined | FlowFunctionTypeParam;
} {
  const params: Array<FlowFunctionTypeParam> = [];
  let rest: undefined | FlowFunctionTypeParam;

  while (!parser.match(tt.parenR) && !parser.match(tt.ellipsis)) {
    params.push(parseFlowFunctionTypeParam(parser));

    if (!parser.match(tt.parenR) && !parser.expect(tt.comma)) {
      break;
    }
  }

  if (parser.eat(tt.ellipsis)) {
    const param = parseFlowFunctionTypeParam(parser);
    rest = param;

    // TODO warn on additional elements?
  }

  return {params, rest};
}

function flowIdentToTypeAnnotation(
  parser: JSParser,
  start: Position,
  id: Identifier,
): AnyFlowKeywordTypeAnnotation | FlowGenericTypeAnnotation {
  switch (id.name) {
    case 'any':
      return parser.finishNode(start, {
        type: 'AnyKeywordTypeAnnotation',
      });

    case 'bool':
    case 'boolean':
      return parser.finishNode(start, {
        type: 'BooleanKeywordTypeAnnotation',
      });

    case 'mixed':
      return parser.finishNode(start, {
        type: 'MixedKeywordTypeAnnotation',
      });

    case 'empty':
      return parser.finishNode(start, {
        type: 'EmptyKeywordTypeAnnotation',
      });

    case 'number':
      return parser.finishNode(start, {
        type: 'NumberKeywordTypeAnnotation',
      });

    case 'string':
      return parser.finishNode(start, {
        type: 'StringKeywordTypeAnnotation',
      });

    case 'bigint':
      return parser.finishNode(start, {
        type: 'BigIntKeywordTypeAnnotation',
      });

    default:
      checkNotUnderscore(parser, id);
      return parseFlowGenericType(parser, start, toReferenceIdentifier(
        parser,
        id,
      ));
  }
}

// The parsing of types roughly parallels the parsing of expressions, and
// primary types are kind of like primary expressions...they're the
// primitives with which other types are constructed.
function parseFlowPrimaryType(parser: JSParser): AnyFlowPrimary {
  const start = parser.getPosition();
  let type;
  let isGroupedType = false;
  const oldNoAnonFunctionType = parser.state.noAnonFunctionType;

  switch (parser.state.tokenType) {
    case tt.name:
      if (parser.isContextual('interface')) {
        return parseFlowInterfaceType(parser);
      }

      return flowIdentToTypeAnnotation(parser, start, parseIdentifier(parser));

    case tt.braceL:
      return parseFlowObjectType(parser, {
        allowStatic: false,
        allowExact: false,
        allowSpread: true,
        allowProto: false,
        allowInexact: true,
      });

    case tt.braceBarL:
      return parseFlowObjectType(parser, {
        allowStatic: false,
        allowExact: true,
        allowSpread: true,
        allowProto: false,
        allowInexact: false,
      });

    case tt.bracketL:
      return parseFlowTupleType(parser);

    case tt.relational:
      if (parser.state.tokenValue === '<') {
        const typeParameters = parseFlowTypeParameterDeclaration(parser, false);
        const openContext = parser.expectOpening(
          tt.parenL,
          tt.parenR,
          'flow function params',
        );
        const {params, rest} = parseFlowFunctionTypeParams(parser);
        parser.expectClosing(openContext);

        parser.expect(tt.arrow);

        const returnType = parseFlowType(parser);

        return parser.finishNode(start, {
          type: 'FlowFunctionTypeAnnotation',
          typeParameters,
          params,
          rest,
          returnType,
        });
      }
      break;

    case tt.parenL:
      {
        const openContext = parser.expectOpening(
          tt.parenL,
          tt.parenR,
          'flow function params',
        );

        // Check to see if this is actually a grouped type
        if (!parser.match(tt.parenR) && !parser.match(tt.ellipsis)) {
          if (parser.match(tt.name)) {
            const token = parser.lookaheadState().tokenType;
            isGroupedType = token !== tt.question && token !== tt.colon;
          } else {
            isGroupedType = true;
          }
        }

        if (isGroupedType) {
          parser.state.noAnonFunctionType = false;
          type = parseFlowType(parser);
          parser.state.noAnonFunctionType = oldNoAnonFunctionType;

          // A `,` or a `) =>` means this is an anonymous function type
          if (parser.state.noAnonFunctionType || !(parser.match(tt.comma) ||
          parser.match(tt.parenR) && parser.lookaheadState().tokenType ===
          tt.arrow)) {
            parser.expectClosing(openContext);
            return type;
          } else {
            // Eat a comma if there is one
            parser.eat(tt.comma);
          }
        }

        let params;
        let rest;
        if (type) {
          const firstParam = reinterpretTypeAsFlowFunctionTypeParam(parser, type);
          ({params, rest} = parseFlowFunctionTypeParams(parser));
          params = [firstParam, ...params];
        } else {
          ({params, rest} = parseFlowFunctionTypeParams(parser));
        }

        parser.expectClosing(openContext);
        parser.expect(tt.arrow);

        const returnType = parseFlowType(parser);

        return parser.finishNode(start, {
          type: 'FlowFunctionTypeAnnotation',
          typeParameters: undefined,
          params,
          rest,
          returnType,
        });
      }

    case tt.num:
    case tt.string:
    case tt._true:
    case tt._false:
    case tt.plusMin:
      return parseTypeLiteralAnnotation(parser);

    case tt._void:
      parser.next();
      return parser.finishNode(start, {type: 'VoidKeywordTypeAnnotation'});

    case tt._null:
      parser.next();
      return parser.finishNode(start, {type: 'NullKeywordTypeAnnotation'});

    case tt._this:
      parser.next();
      return parser.finishNode(start, {type: 'FlowThisTypeAnnotation'});

    case tt.star:
      parser.next();
      return parser.finishNode(start, {type: 'FlowExistsTypeAnnotation'});

    default:
      if (parser.state.tokenType.keyword === 'typeof') {
        return parseFlowTypeofType(parser);
      } else if (parser.state.tokenType.keyword !== undefined) {
        const label = parser.state.tokenType.label;
        parser.next();
        const id = createIdentifier(parser, start, label);
        return flowIdentToTypeAnnotation(parser, start, id);
      }

  }

  parser.addDiagnostic({
    description: descriptions.JS_PARSER.FLOW_UNKNOWN_PRIMARY_START,
  });

  // Fake node
  return parser.finishNode(start, {type: 'MixedKeywordTypeAnnotation'});
}

function parseFlowPostfixType(parser: JSParser): AnyFlowPrimary {
  const startPos = parser.state.startPos;
  let type = parseFlowPrimaryType(parser);
  while (!parser.canInsertSemicolon() && parser.match(tt.bracketL)) {
    const elementType = type;
    parser.expect(tt.bracketL);
    parser.expect(tt.bracketR);
    type = parser.finishNode(startPos, {
      type: 'FlowArrayTypeAnnotation',
      elementType,
    });
  }
  return type;
}

function parseFlowPrefixType(parser: JSParser): AnyFlowPrimary {
  const start = parser.getPosition();
  if (parser.eat(tt.question)) {
    return parser.finishNode(start, {
      type: 'FlowNullableTypeAnnotation',
      typeAnnotation: parseFlowPrefixType(parser),
    });
  } else {
    return parseFlowPostfixType(parser);
  }
}

function parseFlowAnonFunctionWithoutParens(parser: JSParser): AnyFlowPrimary {
  const param = parseFlowPrefixType(parser);

  if (!parser.state.noAnonFunctionType && parser.eat(tt.arrow)) {
    const start = parser.getLoc(param).start;
    const params = [reinterpretTypeAsFlowFunctionTypeParam(parser, param)];
    const returnType = parseFlowType(parser);
    return parser.finishNode(start, {
      type: 'FlowFunctionTypeAnnotation',
      params,
      returnType,
    });
  }

  return param;
}

function parseFlowIntersectionType(parser: JSParser): AnyFlowPrimary {
  const start = parser.getPosition();
  parser.eat(tt.bitwiseAND);

  const type = parseFlowAnonFunctionWithoutParens(parser);
  const types = [type];
  while (parser.eat(tt.bitwiseAND)) {
    types.push(parseFlowAnonFunctionWithoutParens(parser));
  }

  if (types.length === 1) {
    return type;
  } else {
    return parser.finishNode(start, {
      type: 'IntersectionTypeAnnotation',
      types,
    });
  }
}

function eatUnionBitwise(parser: JSParser) {
  if (parser.match(tt.logicalOR)) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.CONFUSED_OR,
    });
    parser.next();
  } else {
    parser.eat(tt.bitwiseOR);
  }
}

function parseFlowUnionType(parser: JSParser): AnyFlowPrimary {
  const start = parser.getPosition();
  eatUnionBitwise(parser);

  const type = parseFlowIntersectionType(parser);
  const types = [type];

  while (parser.match(tt.logicalOR) || parser.match(tt.bitwiseOR)) {
    eatUnionBitwise(parser);
    types.push(parseFlowIntersectionType(parser));
  }

  if (types.length === 1) {
    return type;
  } else {
    return parser.finishNode(start, {
      type: 'UnionTypeAnnotation',
      types,
    });
  }
}

function parseFlowType(parser: JSParser): AnyFlowPrimary {
  parser.pushScope('TYPE', true);
  const type = parseFlowUnionType(parser);
  parser.popScope('TYPE');
  // Ensure that a brace after a function generic type annotation is a

  // statement, except in arrow functions (noAnonFunctionType)
  parser.state.exprAllowed = parser.state.exprAllowed ||
  parser.state.noAnonFunctionType;
  return type;
}

export function parseFlowTypeAnnotation(parser: JSParser): AnyFlowPrimary {
  const start = parser.getPosition();
  addFlowDiagnostic(parser, 'type annotation', start);
  return parseFlowTypeInitialiser(parser);
}

function parseFlowTypeAnnotatableIdentifier(
  parser: JSParser,
  allowPrimitiveOverride: boolean = false,
): BindingIdentifier {
  const start = parser.getPosition();
  const ident = allowPrimitiveOverride
    ? parseBindingIdentifier(parser) : parseFlowRestrictedIdentifier(parser);

  let typeAnnotation = undefined;
  if (parser.match(tt.colon)) {
    typeAnnotation = parseFlowTypeAnnotation(parser);
  }

  if (typeAnnotation === undefined) {
    return ident;
  } else {
    return parser.finishNode(start, {
      ...ident,
      meta: parser.finishNode(start, {type: 'PatternMeta', typeAnnotation}),
    });
  }
}

export function parseFlowClassImplemented(
  parser: JSParser,
): Array<FlowClassImplements> {
  const implemented: Array<FlowClassImplements> = [];

  do {
    const start = parser.getPosition();
    const id = parseFlowRestrictedIdentifier(parser, /*liberal*/true);

    let typeParameters;
    if (parser.isRelational('<')) {
      typeParameters = parseFlowTypeParameterInstantiation(parser);
    }

    implemented.push(parser.finishNode(start, {
      type: 'FlowClassImplements',
      id: toReferenceIdentifier(parser, id),
      typeParameters,
    }));
  } while (parser.eat(tt.comma));

  return implemented;
}

export function parseFlowVariance(parser: JSParser): undefined | FlowVariance {
  if (parser.match(tt.plusMin)) {
    const start = parser.getPosition();
    addFlowDiagnostic(parser, 'variance', start);
    let kind: undefined | FlowVarianceKind;
    if (parser.state.tokenValue === '+') {
      kind = 'plus';
    } else {
      kind = 'minus';
    }
    parser.next();
    return parser.finishNode(start, {
      type: 'FlowVariance',
      kind,
    });
  }
}

export function parseAsyncArrowWithFlowTypeParameters(
  parser: JSParser,
  startPos: Position,
): undefined | ArrowFunctionExpression {
  const {params, rest, typeParameters} = parseFunctionParams(parser);

  const {returnType, valid, predicate} = parseArrowHead(parser);
  if (!valid) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.FLOW_INVALID_ASYNC_ARROW_WITH_TYPE_PARAMS,
    });
    return undefined;
  }

  const func = parseArrowExpression(parser, startPos, {
    bindingList: params,
    rest,
  }, /* isAsync */true);

  return {
    ...func,
    head: {
      ...func.head,
      returnType,
      predicate,
      typeParameters,
    },
  };
}

export function parseFlowObjectTypeInternalSlot(
  parser: JSParser,
  start: Position,
  isStatic: boolean,
): FlowObjectTypeInternalSlot {
  // Note: both bracketL have already been consumed
  const id = parseFlowObjectPropertyKey(parser);
  parser.expect(tt.bracketR);
  parser.expect(tt.bracketR);

  let optional = false;
  let value;
  if (parser.isRelational('<') || parser.match(tt.parenL)) {
    value = parseFlowObjectTypeMethodish(parser, start);
  } else {
    optional = parser.eat(tt.question);
    value = parseFlowTypeInitialiser(parser);
  }

  return parser.finishNode(start, {
    type: 'FlowObjectTypeInternalSlot',
    optional,
    value,
    id,
    static: isStatic,
  });
}
