/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position} from '@romejs/parser-core';
import {TokenType} from '../tokenizer/types';
import {JSParser} from '../parser';
import {types as tt} from '../tokenizer/types';
import {
  parseExpressionAtom,
  parseIdentifierName,
  parseMaybeUnary,
  parseIdentifier,
  parseTemplate,
  parseMaybeAssign,
  parseVarStatement,
  parseObjectPropertyKey,
  parseBindingListNonEmpty,
  hasCommaAfterRest,
  parseBlockOrModuleBlockBody,
  parseExpression,
  parseTypeExpressionStatement,
  parseStringLiteral,
  assertVarKind,
  parseBindingIdentifier,
  parseReferenceIdentifier,
  toBindingIdentifier,
  toReferenceIdentifier,
  parseClassDeclaration,
  parseExportDefaultClassDeclaration,
  parseFunctionDeclaration,
  parseTypeLiteralAnnotation,
} from './index';
import {
  VariableDeclarationKind,
  TSMappedTypeBoolean,
  Identifier,
  StringLiteral,
  AnyExpression,
  ConstTSModifier,
  TSImportType,
  AnyTSEntityName,
  TSTypeReference,
  TSThisType,
  TSTypePredicate,
  TSTypeQuery,
  TSTypeParameter,
  TSTypeParameterDeclaration,
  AnyNode,
  AnyTargetBindingPattern,
  TSConstructSignatureDeclaration,
  TSCallSignatureDeclaration,
  TSIndexSignature,
  TSPropertySignature,
  TSMethodSignature,
  AnyTSTypeElement,
  TSTypeLiteral,
  TSMappedType,
  TSTupleType,
  TSParenthesizedType,
  TSFunctionType,
  TSConstructorType,
  TSTypeOperator,
  TSSignatureDeclarationMeta,
  TSInferType,
  TSTypeAssertion,
  TSExpressionWithTypeArguments,
  TSInterfaceDeclaration,
  TSInterfaceBody,
  TypeAliasTypeAnnotation,
  TSEnumMember,
  TSEnumDeclaration,
  TSModuleBlock,
  TSModuleDeclaration,
  TSImportEqualsDeclaration,
  AnyTSModuleReference,
  TSExternalModuleReference,
  FunctionDeclaration,
  ClassDeclaration,
  VariableDeclarationStatement,
  TSDeclareFunction,
  TSTypeParameterInstantiation,
  ConstTSAccessibility,
  TSExportAssignment,
  TSNamespaceExportDeclaration,
  AnyTSPrimary,
  AnyTSKeywordTypeAnnotation,
  TemplateLiteralTypeAnnotation,
  TSOptionalType,
} from '@romejs/js-ast';

type ParsingContext =
  | 'EnumMembers'
  | 'HeritageClauseElement'
  | 'TupleElementTypes'
  | 'TypeMembers'
  | 'TypeParametersOrArguments';

// Doesn't handle 'void' or 'null' because those are keywords, not identifiers.
function keywordTypeFromName(
  value: string,
): AnyTSKeywordTypeAnnotation['type'] | undefined {
  switch (value) {
    case 'any':
      return 'AnyKeywordTypeAnnotation';

    case 'boolean':
      return 'BooleanKeywordTypeAnnotation';

    case 'bigint':
      return 'BigIntKeywordTypeAnnotation';

    case 'never':
      return 'NeverKeywordTypeAnnotation';

    case 'number':
      return 'NumberKeywordTypeAnnotation';

    case 'object':
      return 'ObjectKeywordTypeAnnotation';

    case 'string':
      return 'StringKeywordTypeAnnotation';

    case 'symbol':
      return 'SymbolKeywordTypeAnnotation';

    case 'undefined':
      return 'UndefinedKeywordTypeAnnotation';

    case 'unknown':
      return 'UnknownKeywordTypeAnnotation';

    default:
      return undefined;
  }
}

function tsIsIdentifier(parser: JSParser): boolean {
  // TODO: actually a bit more complex in TypeScript, but shouldn't matter.
  // See https://github.com/Microsoft/TypeScript/issues/15008
  return parser.match(tt.name);
}

function tsNextTokenCanFollowModifier(parser: JSParser) {
  // Note: TypeScript's implementation is much more complicated because
  // more things are considered modifiers there.
  // This implementation only handles modifiers not handled by @babel/parser itself. And 'static'.
  // TODO: Would be nice to avoid lookahead. Want a hasLineBreakUpNext() method...
  parser.next();
  return (
    !parser.hasPrecedingLineBreak() &&
    !parser.match(tt.parenL) &&
    !parser.match(tt.parenR) &&
    !parser.match(tt.colon) &&
    !parser.match(tt.eq) &&
    !parser.match(tt.question) &&
    !parser.match(tt.bang)
  );
}

/** Parses a modifier matching one the given modifier names. */
export function parseTSModifier<T extends ConstTSModifier>(
  parser: JSParser,
  allowedModifiers: Array<T>,
): undefined | T {
  if (!parser.match(tt.name)) {
    return undefined;
  }

  // @ts-ignore: We are lying here but we validate it in all the correct places
  const modifier: T = String(parser.state.tokenValue);
  if (
    allowedModifiers.includes(modifier) &&
    tryTSParse(parser, tsNextTokenCanFollowModifier)
  ) {
    return modifier;
  }
}

export function hasTSModifier(
  parser: JSParser,
  allowedModifiers: Array<ConstTSModifier>,
): boolean {
  return parseTSModifier(parser, allowedModifiers) !== undefined;
}

function tsIsListTerminator(parser: JSParser, kind: ParsingContext): boolean {
  switch (kind) {
    case 'EnumMembers':
    case 'TypeMembers':
      return parser.match(tt.braceR);

    case 'HeritageClauseElement':
      return parser.match(tt.braceL);

    case 'TupleElementTypes':
      return parser.match(tt.bracketR);

    case 'TypeParametersOrArguments':
      return parser.isRelational('>');
  }

  throw new Error('Unreachable');
}

function parseTSList<T>(
  parser: JSParser,
  kind: ParsingContext,
  parseElement: ParserCallback<T>,
): Array<T> {
  const result: Array<T> = [];
  while (!tsIsListTerminator(parser, kind)) {
    // Skipping 'parseListElement' from the TS source since that's just for error handling.
    result.push(parseElement(parser));
  }
  return result;
}

/**
 * If !expectSuccess, returns undefined instead of failing to parse.
 * If expectSuccess, parseElement should always return a defined value.
 */
function parseTSDelimitedList<T>(
  parser: JSParser,
  kind: ParsingContext,
  parseElement: ParserCallback<undefined | T>,
): Array<T> {
  const result = [];

  while (true) {
    if (tsIsListTerminator(parser, kind)) {
      break;
    }

    const element = parseElement(parser);
    if (element == undefined) {
      break;
    }

    result.push(element);

    if (parser.eat(tt.comma)) {
      continue;
    }

    if (tsIsListTerminator(parser, kind)) {
      break;
    }

    // This will fail with an error about a missing comma
    if (parser.expect(tt.comma)) {
      break;
    }
  }

  return result;
}

function parseTSBracketedList<T>(
  parser: JSParser,
  kind: ParsingContext,
  parseElement: ParserCallback<undefined | T>,
  bracket: boolean,
  skipFirstToken: boolean,
): Array<T> {
  if (!skipFirstToken) {
    if (bracket) {
      parser.expect(tt.bracketL);
    } else {
      parser.expectRelational('<');
    }
  }

  const result = parseTSDelimitedList(parser, kind, parseElement);

  if (bracket) {
    parser.expect(tt.bracketR);
  } else {
    parser.expectRelational('>');
  }

  return result;
}

function parseTSImportType(parser: JSParser): TSImportType {
  const start = parser.getPosition();
  parser.expect(tt._import);
  const openContext = parser.expectOpening(
    tt.parenL,
    tt.parenR,
    'ts import type',
  );

  if (!parser.match(tt.string)) {
    parser.addDiagnostic({
      message: 'Argument in a type import must be a string literal',
    });
  }

  const argument = parseExpressionAtom(parser, 'ts import argument');
  parser.expectClosing(openContext);

  let qualifier;
  if (parser.eat(tt.dot)) {
    qualifier = parseTSEntityName(parser, /* allowReservedWords */ true);
  }

  let typeParameters;
  if (parser.isRelational('<')) {
    typeParameters = parseTSTypeArguments(parser);
  }

  return {
    loc: parser.finishLoc(start),
    type: 'TSImportType',
    argument,
    qualifier,
    typeParameters,
  };
}

function parseTSEntityName(
  parser: JSParser,
  allowReservedWords: boolean,
): AnyTSEntityName {
  let entity: AnyTSEntityName = parseReferenceIdentifier(parser);
  while (parser.eat(tt.dot)) {
    const start: Position = parser.getLoc(entity).start;
    const right = parseIdentifier(parser, allowReservedWords);
    entity = {
      loc: parser.finishLoc(start),
      type: 'TSQualifiedName',
      left: entity,
      right,
    };
  }
  return entity;
}

function parseTSTypeReference(parser: JSParser): TSTypeReference {
  const start = parser.getPosition();
  const typeName = parseTSEntityName(parser, /* allowReservedWords */ false);
  let typeParameters;
  if (!parser.hasPrecedingLineBreak() && parser.isRelational('<')) {
    typeParameters = parseTSTypeArguments(parser);
  }
  return {
    loc: parser.finishLoc(start),
    type: 'TSTypeReference',
    typeName,
    typeParameters,
  };
}

function parseTSThisTypePredicate(
  parser: JSParser,
  lhs: TSThisType,
): TSTypePredicate {
  parser.next();
  const start = parser.getLoc(lhs).start;
  const parameterName = lhs;
  const typeAnnotation = parseTSTypeAnnotation(parser, /* eatColon */ false);

  return {
    loc: parser.finishLoc(start),
    type: 'TSTypePredicate',
    parameterName,
    typeAnnotation,
  };
}

function parseTSThisTypeNode(parser: JSParser): TSThisType {
  const start = parser.getPosition();
  parser.next();
  return {
    loc: parser.finishLoc(start),
    type: 'TSThisType',
  };
}

function parseTSTypeQuery(parser: JSParser): TSTypeQuery {
  const start = parser.getPosition();
  parser.expect(tt._typeof);
  let exprName;
  if (parser.match(tt._import)) {
    exprName = parseTSImportType(parser);
  } else {
    exprName = parseTSEntityName(parser, /* allowReservedWords */ true);
  }
  return {
    loc: parser.finishLoc(start),
    type: 'TSTypeQuery',
    exprName,
  };
}

function parseTSTypeParameter(parser: JSParser): TSTypeParameter {
  const start = parser.getPosition();
  const name = parseIdentifierName(parser);
  const constraint = tsEatThenParseType(parser, tt._extends);
  const _default = tsEatThenParseType(parser, tt.eq);
  return {
    loc: parser.finishLoc(start),
    type: 'TSTypeParameter',
    name,
    constraint,
    default: _default,
  };
}

function tryParseTSTypeParameters(
  parser: JSParser,
): undefined | TSTypeParameterDeclaration {
  if (parser.isRelational('<')) {
    return parseTSTypeParameters(parser);
  }
}

export function parseTSTypeParameters(
  parser: JSParser,
): TSTypeParameterDeclaration {
  const start = parser.getPosition();

  parser.expectRelational('<');

  const params = parseTSBracketedList(
    parser,
    'TypeParametersOrArguments',
    parseTSTypeParameter,
    /* bracket */ false,
    /* skipFirstToken */ true,
  );

  return {
    loc: parser.finishLoc(start),
    type: 'TSTypeParameterDeclaration',
    params,
  };
}

export function tryTSNextParseConstantContext(
  parser: JSParser,
): undefined | TSTypeReference {
  if (parser.lookaheadState().tokenType === tt._const) {
    parser.next();
    return parseTSTypeReference(parser);
  }
}

export function tsCheckLiteralForConstantContext(
  parser: JSParser,
  node: AnyNode,
) {
  switch (node.type) {
    case 'StringLiteral':
    case 'TemplateLiteral':
    case 'NumericLiteral':
    case 'BooleanLiteral':
    case 'SpreadElement':
    case 'ObjectMethod':
    case 'ObjectExpression':
      break;

    case 'ArrayExpression':
      for (const elem of node.elements) {
        if (elem) {
          tsCheckLiteralForConstantContext(parser, elem);
        }
      }
      break;

    case 'ObjectProperty':
      tsCheckLiteralForConstantContext(parser, node.value);
      break;

    case 'UnaryExpression':
      tsCheckLiteralForConstantContext(parser, node.argument);
      break;

    default:
      parser.addDiagnostic({
        loc: node.loc,
        message: 'Only literal values are allowed in constant contexts',
      });
  }
}

// Note: In TypeScript implementation we must provide `yieldContext` and `awaitContext`,
// but here it's always false, because parser.is only used for types.
function parseTSSignatureDeclarationMeta(
  parser: JSParser,
  returnToken: TokenType,
): {
  typeAnnotation: undefined | AnyTSPrimary;
  meta: TSSignatureDeclarationMeta;
} {
  const start = parser.getPosition();

  // Arrow fns *must* have return token (`=>`). Normal functions can omit it.
  const returnTokenRequired = returnToken === tt.arrow;
  const typeParameters = tryParseTSTypeParameters(parser);
  const {list: parameters, rest} = parseTSBindingListForSignature(parser);

  let typeAnnotation;
  if (returnTokenRequired) {
    typeAnnotation = parseTSTypeOrTypePredicateAnnotation(parser, returnToken);
  } else if (parser.match(returnToken)) {
    typeAnnotation = parseTSTypeOrTypePredicateAnnotation(parser, returnToken);
  }

  return {
    typeAnnotation,
    meta: {
      type: 'TSSignatureDeclarationMeta',
      loc: parser.finishLoc(start),
      typeParameters,
      parameters,
      rest,
    },
  };
}

function parseTSBindingListForSignature(
  parser: JSParser,
): {
  list: Array<AnyTargetBindingPattern>;
  rest: undefined | AnyTargetBindingPattern;
} {
  const openContext = parser.expectOpening(
    tt.parenL,
    tt.parenR,
    'ts signature parameters',
  );
  const {list: patterns, rest} = parseBindingListNonEmpty(parser, openContext);
  const validPatterns: Array<AnyTargetBindingPattern> = [];

  for (const pattern of patterns) {
    if (
      pattern.type === 'BindingIdentifier' ||
      pattern.type === 'BindingObjectPattern' ||
      pattern.type === 'BindingArrayPattern'
    ) {
      validPatterns.push(pattern);
    } else {
      parser.addDiagnostic({
        loc: pattern.loc,
        message: `Name in a signature must be an Identifier, ObjectPattern or ArrayPattern, instead got ${pattern.type}`,
      });
    }
  }

  return {list: validPatterns, rest};
}

function parseTSTypeMemberSemicolon(parser: JSParser): void {
  if (!parser.eat(tt.comma)) {
    parser.semicolon();
  }
}

function parseTSConstructSignatureDeclaration(
  parser: JSParser,
): TSConstructSignatureDeclaration {
  const start = parser.getPosition();
  parser.expect(tt._new);
  const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
    parser,
    tt.colon,
  );
  parser.semicolon();
  return {
    type: 'TSConstructSignatureDeclaration',
    loc: parser.finishLoc(start),
    meta,
    typeAnnotation,
  };
}

function parseTSCallSignatureDeclaration(
  parser: JSParser,
): TSCallSignatureDeclaration {
  const start = parser.getPosition();
  const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
    parser,
    tt.colon,
  );
  parser.semicolon();
  return {
    type: 'TSCallSignatureDeclaration',
    loc: parser.finishLoc(start),
    meta,
    typeAnnotation,
  };
}

function tsIsUnambiguouslyIndexSignature(parser: JSParser) {
  parser.next(); // Skip '{'
  return parser.eat(tt.name) && parser.match(tt.colon);
}

export function tryTSParseIndexSignature(
  parser: JSParser,
  start: Position,
): undefined | TSIndexSignature {
  if (
    !(
      parser.match(tt.bracketL) &&
      lookaheadTS(parser, tsIsUnambiguouslyIndexSignature)
    )
  ) {
    return undefined;
  }

  parser.expect(tt.bracketL);

  const idStart = parser.getPosition();
  const id = {
    ...parseBindingIdentifier(parser),
    typeAnnotation: parseTSTypeAnnotation(parser),
    loc: parser.finishLoc(idStart),
  };

  parser.expect(tt.bracketR);
  const parameters = [id];

  const typeAnnotation = tryTSParseTypeAnnotation(parser);

  parser.semicolon();
  return {
    loc: parser.finishLoc(start),
    type: 'TSIndexSignature',
    typeAnnotation,
    parameters,
    rest: undefined,
  };
}

function parseTSPropertyOrMethodSignature(
  parser: JSParser,
  start: Position,
  readonly: boolean,
): TSPropertySignature | TSMethodSignature {
  const key = parseObjectPropertyKey(parser);
  const optional = parser.eat(tt.question);

  if (!readonly && (parser.match(tt.parenL) || parser.isRelational('<'))) {
    const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
      parser,
      tt.colon,
    );
    parseTSTypeMemberSemicolon(parser);
    return {
      loc: parser.finishLoc(start),
      type: 'TSMethodSignature',
      optional,
      meta,
      key,
      typeAnnotation,
    };
  } else {
    const typeAnnotation = tryTSParseTypeAnnotation(parser);
    parseTSTypeMemberSemicolon(parser);
    return {
      loc: parser.finishLoc(start),
      type: 'TSPropertySignature',
      optional,
      readonly,
      typeAnnotation,
      key,
    };
  }
}

function parseTSTypeMember(parser: JSParser): AnyTSTypeElement {
  if (parser.match(tt.parenL) || parser.isRelational('<')) {
    return parseTSCallSignatureDeclaration(parser);
  }

  if (
    parser.match(tt._new) &&
    lookaheadTS(parser, tsIsStartOfConstructSignature)
  ) {
    return parseTSConstructSignatureDeclaration(parser);
  }

  const start = parser.getPosition();
  const readonly = hasTSModifier(parser, ['readonly']);

  const idx = tryTSParseIndexSignature(parser, start);
  if (idx) {
    return {
      ...idx,
      readonly,
    };
  }

  return parseTSPropertyOrMethodSignature(parser, start, readonly);
}

function tsIsStartOfConstructSignature(parser: JSParser) {
  parser.next();
  return parser.match(tt.parenL) || parser.isRelational('<');
}

function parseTSTypeLiteral(parser: JSParser): TSTypeLiteral {
  const start = parser.getPosition();
  const members = parseTSObjectTypeMembers(parser);
  return {
    loc: parser.finishLoc(start),
    type: 'TSTypeLiteral',
    members,
  };
}

function parseTSObjectTypeMembers(parser: JSParser): Array<AnyTSTypeElement> {
  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'ts object type members',
  );
  const members = parseTSList(parser, 'TypeMembers', parseTSTypeMember);
  parser.expectClosing(openContext);
  return members;
}

function tsIsStartOfMappedType(parser: JSParser): boolean {
  parser.next();

  if (parser.eat(tt.plusMin)) {
    return parser.isContextual('readonly');
  }

  if (parser.isContextual('readonly')) {
    parser.next();
  }

  if (!parser.match(tt.bracketL)) {
    return false;
  }

  parser.next();

  if (!tsIsIdentifier(parser)) {
    return false;
  }

  parser.next();

  return parser.match(tt._in);
}

function parseTSMappedTypeParameter(parser: JSParser): TSTypeParameter {
  const start = parser.getPosition();
  const name = parseIdentifierName(parser);
  const constraint = tsExpectThenParseType(parser, tt._in);
  return {
    loc: parser.finishLoc(start),
    type: 'TSTypeParameter',
    name,
    constraint,
  };
}

function toPlusMin(val: unknown): '+' | '-' {
  const str = String(val);
  if (str === '+' || str === '-') {
    return str;
  } else {
    throw new Error('Expected +/-');
  }
}

function parseTSMappedType(parser: JSParser): TSMappedType {
  const start = parser.getPosition();

  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'ts mapped type',
  );

  let readonly: TSMappedTypeBoolean;
  if (parser.match(tt.plusMin)) {
    readonly = toPlusMin(parser.state.tokenValue);
    parser.next();
    parser.expectContextual('readonly');
  } else if (parser.eatContextual('readonly')) {
    readonly = true;
  }

  const paramOpenContext = parser.expectOpening(
    tt.bracketL,
    tt.bracketR,
    'ts mapped type parameter',
  );
  const typeParameter = parseTSMappedTypeParameter(parser);
  parser.expectClosing(paramOpenContext);

  let optional: TSMappedTypeBoolean;
  if (parser.match(tt.plusMin)) {
    optional = toPlusMin(parser.state.tokenValue);
    parser.next();
    parser.expect(tt.question);
  } else if (parser.eat(tt.question)) {
    optional = true;
  }

  const typeAnnotation = tryTSParseType(parser);
  parser.semicolon();
  parser.expectClosing(openContext);

  return {
    loc: parser.finishLoc(start),
    type: 'TSMappedType',
    typeParameter,
    typeAnnotation,
    optional,
    readonly,
  };
}

function parseTSTupleType(parser: JSParser): TSTupleType {
  const start = parser.getPosition();
  const elementDefs = parseTSBracketedList(
    parser,
    'TupleElementTypes',
    parseTSTupleElementType,
    /* bracket */ true,
    /* skipFirstToken */ false,
  );

  // Validate the elementTypes to ensure:
  //   No mandatory elements may follow optional elements
  //   If there's a rest element, it must be at the end of the tuple
  let seenOptionalElement = false;
  const elementTypes: TSTupleType['elementTypes'] = [];
  let rest: undefined | AnyTSPrimary;
  for (const {type, isRest} of elementDefs) {
    if (rest !== undefined) {
      // No elements should come after a rest, we should have already produced an error
      continue;
    }

    if (type.type === 'TSOptionalType') {
      seenOptionalElement = true;
    } else if (seenOptionalElement && !isRest) {
      parser.addDiagnostic({
        loc: type.loc,
        message: 'A required element cannot follow an optional element.',
      });
    }

    if (isRest) {
      rest = type;
    } else {
      elementTypes.push(type);
    }
  }

  return {
    loc: parser.finishLoc(start),
    type: 'TSTupleType',
    elementTypes,
    rest,
  };
}

function parseTSTupleElementType(
  parser: JSParser,
): {
  type: AnyTSPrimary | TSOptionalType;
  isRest: boolean;
} {
  // parses `...TsType[]`
  if (parser.match(tt.ellipsis)) {
    parser.next(); // skips ellipsis

    const typeAnnotation = parseTSType(parser);
    hasCommaAfterRest(parser);

    return {
      isRest: true,
      type: typeAnnotation,
    };
  }

  const typeAnnotation = parseTSType(parser);

  // Parses `TsType?`
  if (parser.eat(tt.question)) {
    const start = parser.getLoc(typeAnnotation).start;
    return {
      isRest: false,
      type: {
        loc: parser.finishLoc(start),
        type: 'TSOptionalType',
        typeAnnotation,
      },
    };
  }

  return {
    isRest: false,
    type: typeAnnotation,
  };
}

function parseTSParenthesizedType(parser: JSParser): TSParenthesizedType {
  const start = parser.getPosition();
  const openContext = parser.expectOpening(
    tt.parenL,
    tt.parenR,
    'ts parenthesized type',
  );
  const typeAnnotation = parseTSType(parser);
  parser.expectClosing(openContext);
  return {
    loc: parser.finishLoc(start),
    type: 'TSParenthesizedType',
    typeAnnotation,
  };
}

function parseTSFunctionType(parser: JSParser): TSFunctionType {
  const start = parser.getPosition();
  const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
    parser,
    tt.arrow,
  );

  if (typeAnnotation === undefined) {
    throw new Error(
      'Type annotation return type required as we passed tt.arrow above',
    );
  }

  return {
    type: 'TSFunctionType',
    loc: parser.finishLoc(start),
    meta,
    typeAnnotation,
  };
}

function parseTSConstructorType(parser: JSParser): TSConstructorType {
  const start = parser.getPosition();
  parser.expect(tt._new);

  const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
    parser,
    tt.arrow,
  );

  if (typeAnnotation === undefined) {
    throw new Error(
      'Type annotation return type required as we passed tt.arrow above',
    );
  }

  return {
    type: 'TSConstructorType',
    loc: parser.finishLoc(start),
    meta,
    typeAnnotation,
  };
}

function parseTSTemplateLiteralType(
  parser: JSParser,
): TemplateLiteralTypeAnnotation {
  const templateNode = parseTemplate(parser, false);

  if (templateNode.expressions.length > 0) {
    parser.addDiagnostic({
      loc: parser.getLoc(templateNode.expressions[0]),
      message: 'Template literal types cannot have any substitution',
    });
  }

  return {
    type: 'TemplateLiteralTypeAnnotation',
    value: templateNode.quasis[0].raw,
    loc: templateNode.loc,
  };
}

function parseTSNonArrayType(parser: JSParser): AnyTSPrimary {
  switch (parser.state.tokenType) {
    case tt.name:
    case tt._void:
    case tt._null: {
      let type:
        | undefined
        | AnyTSKeywordTypeAnnotation['type']
        | 'VoidKeywordTypeAnnotation'
        | 'NullKeywordTypeAnnotation';
      if (parser.match(tt._void)) {
        type = 'VoidKeywordTypeAnnotation';
      } else if (parser.match(tt._null)) {
        type = 'NullKeywordTypeAnnotation';
      } else {
        type = keywordTypeFromName(String(parser.state.tokenValue));
      }
      if (type !== undefined && parser.lookaheadState().tokenType !== tt.dot) {
        const start = parser.getPosition();
        parser.next();
        return {
          type,
          loc: parser.finishLoc(start),
        } as AnyTSPrimary;
      }
      return parseTSTypeReference(parser);
    }

    case tt.string:
    case tt.num:
    case tt._true:
    case tt._false:
    case tt.plusMin:
      return parseTypeLiteralAnnotation(parser);

    case tt._this: {
      const thisKeyword = parseTSThisTypeNode(parser);
      if (parser.isContextual('is') && !parser.hasPrecedingLineBreak()) {
        return parseTSThisTypePredicate(parser, thisKeyword);
      } else {
        return thisKeyword;
      }
    }

    case tt._typeof:
      return parseTSTypeQuery(parser);

    case tt._import:
      return parseTSImportType(parser);

    case tt.braceL:
      if (lookaheadTS(parser, tsIsStartOfMappedType)) {
        return parseTSMappedType(parser);
      } else {
        return parseTSTypeLiteral(parser);
      }

    case tt.bracketL:
      return parseTSTupleType(parser);

    case tt.parenL:
      return parseTSParenthesizedType(parser);

    case tt.backQuote:
      return parseTSTemplateLiteralType(parser);
  }

  parser.addDiagnostic({
    message: 'Unknown TS non array type start',
  });
  parser.next();

  return {
    type: 'TSTypeReference',
    loc: parser.finishLoc(parser.getPosition()),
    typeName: toReferenceIdentifier(
      parser.createUnknownIdentifier('ts non array type start'),
    ),
  };
}

function parseTSArrayTypeOrHigher(parser: JSParser): AnyTSPrimary {
  let type = parseTSNonArrayType(parser);

  while (!parser.hasPrecedingLineBreak() && parser.eat(tt.bracketL)) {
    if (parser.match(tt.bracketR)) {
      const start = parser.getLoc(type).start;
      const elementType = type;
      parser.expect(tt.bracketR);
      type = {
        loc: parser.finishLoc(start),
        type: 'TSArrayType',
        elementType,
      };
    } else {
      const start = parser.getLoc(type).start;
      const objectType = type;
      const indexType = parseTSType(parser);
      parser.expect(tt.bracketR);
      type = {
        loc: parser.finishLoc(start),
        type: 'TSIndexedAccessType',
        objectType,
        indexType,
      };
    }
  }
  return type;
}

function parseTSTypeOperator(
  parser: JSParser,
  operator: TSTypeOperator['operator'],
): TSTypeOperator {
  const start = parser.getPosition();
  parser.expectContextual(operator);

  const typeAnnotation = parseTSTypeOperatorOrHigher(parser);

  const node: TSTypeOperator = {
    loc: parser.finishLoc(start),
    type: 'TSTypeOperator',
    typeAnnotation,
    operator,
  };

  if (operator === 'readonly') {
    tsCheckTypeAnnotationForReadOnly(parser, typeAnnotation);
  }

  return node;
}

function tsCheckTypeAnnotationForReadOnly(
  parser: JSParser,
  node: AnyTSPrimary,
) {
  switch (node.type) {
    case 'TSTupleType':
    case 'TSArrayType':
      return undefined;

    default:
      parser.addDiagnostic({
        loc: node.loc,
        message:
          "'readonly' type modifier is only permitted on array and tuple literal types.",
      });
      break;
  }
}

function parseTSInferType(parser: JSParser): TSInferType {
  const inferStart = parser.getPosition();
  parser.expectContextual('infer');

  const start = parser.getPosition();
  const typeParameter: TSTypeParameter = {
    type: 'TSTypeParameter',
    name: parseIdentifierName(parser),
    loc: parser.finishLoc(start),
  };

  return {
    loc: parser.finishLoc(inferStart),
    type: 'TSInferType',
    typeParameter,
  };
}

const TS_TYPE_OPERATORS: Array<TSTypeOperator['operator']> = [
  'keyof',
  'unique',
  'readonly',
];

function parseTSTypeOperatorOrHigher(parser: JSParser): AnyTSPrimary {
  let operator: undefined | TSTypeOperator['operator'];

  for (const op of TS_TYPE_OPERATORS) {
    if (parser.isContextual(op)) {
      operator = op;
      break;
    }
  }

  if (operator !== undefined) {
    return parseTSTypeOperator(parser, operator);
  } else if (parser.isContextual('infer')) {
    return parseTSInferType(parser);
  } else {
    return parseTSArrayTypeOrHigher(parser);
  }
}

function parseTSUnionOrIntersectionType(
  parser: JSParser,
  kind: 'UnionTypeAnnotation' | 'IntersectionTypeAnnotation',
  parseConstituentType: ParserCallback<AnyTSPrimary>,
  operator: TokenType,
): AnyTSPrimary {
  parser.eat(operator);
  let type = parseConstituentType(parser);

  if (parser.match(operator)) {
    const types = [type];
    while (parser.eat(operator)) {
      types.push(parseConstituentType(parser));
    }

    const start = parser.getLoc(type).start;
    if (kind === 'UnionTypeAnnotation') {
      type = {
        loc: parser.finishLoc(start),
        type: 'UnionTypeAnnotation',
        types,
      };
    } else if (kind === 'IntersectionTypeAnnotation') {
      type = {
        loc: parser.finishLoc(start),
        type: 'IntersectionTypeAnnotation',
        types,
      };
    }
  }

  return type;
}

function parseIntersectionTypeAnnotationOrHigher(
  parser: JSParser,
): AnyTSPrimary {
  return parseTSUnionOrIntersectionType(
    parser,
    'IntersectionTypeAnnotation',
    parseTSTypeOperatorOrHigher,
    tt.bitwiseAND,
  );
}

function parseUnionTypeAnnotationOrHigher(parser: JSParser) {
  return parseTSUnionOrIntersectionType(
    parser,
    'UnionTypeAnnotation',
    parseIntersectionTypeAnnotationOrHigher,
    tt.bitwiseOR,
  );
}

function tsIsStartOfFunctionType(parser: JSParser) {
  if (parser.isRelational('<')) {
    return true;
  }
  return (
    parser.match(tt.parenL) &&
    lookaheadTS(parser, tsIsUnambiguouslyStartOfFunctionType)
  );
}

function tsSkipParameterStart(parser: JSParser): boolean {
  if (parser.match(tt.name) || parser.match(tt._this)) {
    parser.next();
    return true;
  }

  if (parser.match(tt.braceL)) {
    let braceStackCounter = 1;
    parser.next();

    while (braceStackCounter > 0) {
      if (parser.match(tt.braceL)) {
        braceStackCounter++;
      } else if (parser.match(tt.braceR)) {
        braceStackCounter--;
      }
      parser.next();
    }
    return true;
  }

  if (parser.match(tt.bracketL)) {
    let braceStackCounter = 1;
    parser.next();

    while (braceStackCounter > 0) {
      if (parser.match(tt.bracketL)) {
        braceStackCounter++;
      } else if (parser.match(tt.bracketR)) {
        braceStackCounter--;
      }
      parser.next();
    }
    return true;
  }

  return false;
}

function tsIsUnambiguouslyStartOfFunctionType(parser: JSParser): boolean {
  parser.next();
  if (parser.match(tt.parenR) || parser.match(tt.ellipsis)) {
    // ()
    // (...
    return true;
  }
  if (tsSkipParameterStart(parser)) {
    if (
      parser.match(tt.colon) ||
      parser.match(tt.comma) ||
      parser.match(tt.question) ||
      parser.match(tt.eq)
    ) {
      // (xxx :
      // (xxx ,
      // (xxx ?
      // (xxx =
      return true;
    }
    if (parser.match(tt.parenR)) {
      parser.next();
      if (parser.match(tt.arrow)) {
        // (xxx ) =>
        return true;
      }
    }
  }
  return false;
}

export function parseTSTypeOrTypePredicateAnnotation(
  parser: JSParser,
  returnToken: TokenType,
): AnyTSPrimary {
  const start = parser.getPosition();
  parser.pushScope('TYPE', true);
  parser.expect(returnToken);

  let typePredicateVariable;
  if (tsIsIdentifier(parser)) {
    typePredicateVariable = tryTSParse(parser, parseTSTypePredicatePrefix);
  }
  if (typePredicateVariable === undefined) {
    parser.popScope('TYPE');
    return parseTSTypeAnnotation(parser, /* eatColon */ false, start);
  }

  const type = parseTSTypeAnnotation(parser, /* eatColon */ false);

  const typePredicateStart = parser.getLoc(typePredicateVariable).start;
  parser.popScope('TYPE');

  return {
    loc: parser.finishLoc(typePredicateStart),
    type: 'TSTypePredicate',
    parameterName: typePredicateVariable,
    typeAnnotation: type,
  };
}

function tryTSParseTypeAnnotation(parser: JSParser): undefined | AnyTSPrimary {
  return parser.match(tt.colon) ? parseTSTypeAnnotation(parser) : undefined;
}

function tryTSParseType(parser: JSParser): undefined | AnyTSPrimary {
  return tsEatThenParseType(parser, tt.colon);
}

function parseTSTypePredicatePrefix(parser: JSParser): undefined | Identifier {
  const id = parseIdentifier(parser);
  if (parser.isContextual('is') && !parser.hasPrecedingLineBreak()) {
    parser.next();
    return id;
  }
}

export function parseTSTypeAnnotation(
  parser: JSParser,
  eatColon: boolean = true,
  start: Position = parser.getPosition(),
): AnyTSPrimary {
  parser.pushScope('TYPE', true);

  if (eatColon) {
    parser.expect(tt.colon);
  }

  const typeAnnotation = parseTSType(parser, start);
  parser.popScope('TYPE');
  return typeAnnotation;
}

/** Be sure to be in a type context before calling parser. using `tsInType`. */
function parseTSType(
  parser: JSParser,
  start: Position = parser.getPosition(),
): AnyTSPrimary {
  parser.pushScope('TYPE', true);

  const type = parseTSNonConditionalType(parser);
  if (parser.hasPrecedingLineBreak() || !parser.eat(tt._extends)) {
    parser.popScope('TYPE');
    return type;
  }

  const checkType = type;

  const extendsType = parseTSNonConditionalType(parser);
  parser.expect(tt.question);

  const trueType = parseTSType(parser);
  parser.expect(tt.colon);

  const falseType = parseTSType(parser);
  parser.popScope('TYPE');

  return {
    loc: parser.finishLoc(start),
    type: 'TSConditionalType',
    checkType,
    extendsType,
    trueType,
    falseType,
  };
}

function parseTSNonConditionalType(parser: JSParser): AnyTSPrimary {
  if (tsIsStartOfFunctionType(parser)) {
    return parseTSFunctionType(parser);
  }

  if (parser.match(tt._new)) {
    // As in `new () => Date`
    return parseTSConstructorType(parser);
  }

  return parseUnionTypeAnnotationOrHigher(parser);
}

export function parseTSTypeAssertion(parser: JSParser): TSTypeAssertion {
  const start = parser.getPosition();
  const _const = tryTSNextParseConstantContext(parser);
  const typeAnnotation = _const || tsNextThenParseType(parser);
  parser.expectRelational('>');

  const expression = parseMaybeUnary(parser, 'ts type assertion');
  if (_const) {
    tsCheckLiteralForConstantContext(parser, expression);
  }

  return {
    loc: parser.finishLoc(start),
    type: 'TSTypeAssertion',
    expression,
    typeAnnotation,
  };
}

export function parseTSHeritageClause(
  parser: JSParser,
  descriptor: string,
): Array<TSExpressionWithTypeArguments> {
  const originalStart = parser.state.startPos;
  const delimitedList = parseTSDelimitedList(
    parser,
    'HeritageClauseElement',
    parseTSExpressionWithTypeArguments,
  );

  if (!delimitedList.length) {
    parser.addDiagnostic({
      start: originalStart,
      message: `'${descriptor}' list cannot be empty.`,
    });
  }

  return delimitedList;
}

function parseTSExpressionWithTypeArguments(
  parser: JSParser,
): TSExpressionWithTypeArguments {
  const start = parser.getPosition();

  // Note: TS uses parseLeftHandSideExpressionOrHigher,
  // then has grammar errors later if it's not an EntityName.
  const expression = parseTSEntityName(parser, /* allowReservedWords */ false);

  let typeParameters;
  if (parser.isRelational('<')) {
    typeParameters = parseTSTypeArguments(parser);
  }

  return {
    loc: parser.finishLoc(start),
    type: 'TSExpressionWithTypeArguments',
    expression,
    typeParameters,
  };
}

export function parseTSInterfaceDeclaration(
  parser: JSParser,
  start: Position,
): TSInterfaceDeclaration {
  parser.pushScope('TYPE', true);
  const id = parseBindingIdentifier(parser);
  const typeParameters = tryParseTSTypeParameters(parser);

  let _extends;
  if (parser.eat(tt._extends)) {
    _extends = parseTSHeritageClause(parser, 'extends');
  }

  const bodyStart = parser.getPosition();
  const bodyItems = parseTSObjectTypeMembers(parser);
  const body: TSInterfaceBody = {
    loc: parser.finishLoc(bodyStart),
    type: 'TSInterfaceBody',
    body: bodyItems,
  };

  parser.popScope('TYPE');
  return {
    loc: parser.finishLoc(start),
    type: 'TSInterfaceDeclaration',
    id,
    body,
    typeParameters,
    extends: _extends,
  };
}

export function parseTSTypeAliasTypeAnnotation(
  parser: JSParser,
  start: Position,
): TypeAliasTypeAnnotation {
  const id = parseBindingIdentifier(parser);
  const typeParameters = tryParseTSTypeParameters(parser);
  const typeAnnotation = tsExpectThenParseType(parser, tt.eq);
  parser.semicolon();
  return {
    loc: parser.finishLoc(start),
    type: 'TypeAliasTypeAnnotation',
    id,
    typeParameters,
    right: typeAnnotation,
  };
}

function tsInNoContext<T>(parser: JSParser, cb: ParserCallback<T>): T {
  const oldContext = parser.state.context;
  parser.state.context = [oldContext[0]];
  const res = cb(parser);
  parser.state.context = oldContext;
  return res;
}

function tsEatThenParseType(
  parser: JSParser,
  token: TokenType,
): AnyTSPrimary | undefined {
  if (parser.match(token)) {
    return tsNextThenParseType(parser);
  }
}

function tsExpectThenParseType(
  parser: JSParser,
  token: TokenType,
): AnyTSPrimary {
  return tsDoThenParseType(parser, () => {
    parser.expect(token);
  });
}

export function tsNextThenParseType(parser: JSParser): AnyTSPrimary {
  return tsDoThenParseType(parser, () => parser.next());
}

function tsDoThenParseType(parser: JSParser, cb: () => void): AnyTSPrimary {
  cb();
  return parseTSType(parser);
}

function parseTSEnumMember(parser: JSParser): TSEnumMember {
  const start = parser.getPosition();
  // Computed property names are grammar errors in an enum, so accept just string literal or identifier.
  const id: StringLiteral | Identifier = parser.match(tt.string)
    ? parseStringLiteral(parser)
    : parseIdentifier(parser, /* liberal */ true);

  let initializer: undefined | AnyExpression;
  if (parser.eat(tt.eq)) {
    initializer = parseMaybeAssign<AnyExpression>(
      parser,
      'ts enum member initializer',
    );
  }

  return {
    loc: parser.finishLoc(start),
    type: 'TSEnumMember',
    initializer,
    id,
  };
}

export function parseTSEnumDeclaration(
  parser: JSParser,
  start: Position,
  isConst: boolean,
): TSEnumDeclaration {
  parser.addDiagnosticFilter({
    message: 'enum is a reserved word',
    start,
  });

  const id = parseBindingIdentifier(parser);

  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'ts enum declaration',
  );

  const members = parseTSDelimitedList(
    parser,
    'EnumMembers',
    parseTSEnumMember,
  );
  parser.expectClosing(openContext);

  return {
    loc: parser.finishLoc(start),
    type: 'TSEnumDeclaration',
    members,
    id,
    const: isConst,
  };
}

export function parseTSModuleBlock(parser: JSParser): TSModuleBlock {
  const start = parser.getPosition();

  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'ts module block',
  );

  // Inside of a module block is considered 'top-level', meaning it can have imports and exports.
  const {body} = parseBlockOrModuleBlockBody(
    parser,
    /* allowDirectives */ false,
    /* topLevel */ true,
    openContext,
  );
  return {
    loc: parser.finishLoc(start),
    type: 'TSModuleBlock',
    body,
  };
}

export function parseTSModuleOrNamespaceDeclaration(
  parser: JSParser,
  start: Position,
): TSModuleDeclaration {
  const id = parseBindingIdentifier(parser);

  let body;
  if (parser.eat(tt.dot)) {
    body = parseTSModuleOrNamespaceDeclaration(parser, parser.getPosition());
  } else {
    body = parseTSModuleBlock(parser);
  }

  return {
    loc: parser.finishLoc(start),
    type: 'TSModuleDeclaration',
    id,
    body,
  };
}

export function parseTSAmbientExternalModuleDeclaration(
  parser: JSParser,
  start: Position,
): TSModuleDeclaration {
  let global;
  let id;
  if (parser.isContextual('global')) {
    global = true;
    id = parseBindingIdentifier(parser);
  } else if (parser.match(tt.string)) {
    id = parseStringLiteral(parser);
  } else {
    throw parser.unexpected();
  }

  let body;
  if (parser.match(tt.braceL)) {
    body = parseTSModuleBlock(parser);
  } else {
    parser.semicolon();
  }

  return {
    loc: parser.finishLoc(start),
    type: 'TSModuleDeclaration',
    id,
    global,
    body,
  };
}

export function parseTSImportEqualsDeclaration(
  parser: JSParser,
  start: Position,
  isExport: boolean = false,
): TSImportEqualsDeclaration {
  const id = parseBindingIdentifier(parser);
  parser.expect(tt.eq);

  const moduleReference = parseTSModuleReference(parser);
  parser.semicolon();

  return {
    loc: parser.finishLoc(start),
    type: 'TSImportEqualsDeclaration',
    id,
    moduleReference,
    isExport,
  };
}

function tsIsExternalModuleReference(parser: JSParser): boolean {
  return (
    parser.isContextual('require') &&
    parser.lookaheadState().tokenType === tt.parenL
  );
}

function parseTSModuleReference(parser: JSParser): AnyTSModuleReference {
  return tsIsExternalModuleReference(parser)
    ? parseTSExternalModuleReference(parser)
    : parseTSEntityName(parser, /* allowReservedWords */ false);
}

function parseTSExternalModuleReference(
  parser: JSParser,
): TSExternalModuleReference {
  const start = parser.getPosition();
  parser.expectContextual('require');
  const openContext = parser.expectOpening(
    tt.parenL,
    tt.parenR,
    'ts external module reference',
  );

  let expression: StringLiteral;
  if (parser.match(tt.string)) {
    expression = parseStringLiteral(parser);
  } else {
    parser.addDiagnostic({
      message: 'Invalid TS external module reference expression',
    });

    // Skip as much of the next expression as we can
    parseExpressionAtom(parser, 'ts external module reference expression');

    // Create a fake string literal
    expression = {
      loc: parser.finishLoc(start),
      type: 'StringLiteral',
      value: '',
    };
  }

  parser.expectClosing(openContext);

  return {
    loc: parser.finishLoc(start),
    type: 'TSExternalModuleReference',
    expression,
  };
}

// Utilities

type ParserCallback<T> = (parser: JSParser) => T;

function lookaheadTS<T>(parser: JSParser, f: ParserCallback<T>): T {
  const state = parser.cloneState();
  const res = f(parser);
  parser.state = state;
  return res;
}

function tryTSParse<T>(
  parser: JSParser,
  f: ParserCallback<undefined | false | T>,
): undefined | T {
  const state = parser.cloneState();
  const result = f(parser);
  if (result === undefined || result === false) {
    parser.state = state;
    return undefined;
  } else {
    return result;
  }
}

export type TSDeclareNode =
  | TSEnumDeclaration
  | FunctionDeclaration
  | ClassDeclaration
  | VariableDeclarationStatement
  | TSDeclareFunction
  | TSModuleDeclaration
  | TypeAliasTypeAnnotation
  | TSInterfaceDeclaration;

export function parseTSDeclare(
  parser: JSParser,
  start: Position,
): TSDeclareNode {
  let starttype = parser.state.tokenType;
  let kind: undefined | VariableDeclarationKind;
  if (parser.isContextual('let')) {
    starttype = tt._var;
    kind = 'let';
  }

  if (
    starttype === tt._const &&
    parser.match(tt._const) &&
    parser.isLookaheadContextual('enum')
  ) {
    // `const enum = 0;` not allowed because 'enum' is a strict mode reserved word.
    parser.expect(tt._const);
    parser.expectContextual('enum');
    return {
      declare: true,
      ...parseTSEnumDeclaration(parser, start, /* isConst */ true),
    };
  }

  switch (starttype) {
    case tt._function:
      return {
        ...parseFunctionDeclaration(parser, start, false),
        declare: true,
      };

    case tt._class:
      return {
        ...parseClassDeclaration(parser, start),
        declare: true,
      };

    case tt._const:
    case tt._var:
      kind =
        kind === undefined
          ? assertVarKind(String(parser.state.tokenValue))
          : kind;
      return {
        declare: true,
        ...parseVarStatement(parser, start, kind),
      };

    case tt.name: {
      const value = String(parser.state.tokenValue);

      if (value === 'global') {
        return {
          declare: true,
          ...parseTSAmbientExternalModuleDeclaration(parser, start),
        };
      } else if (isTSDeclarationStart(parser)) {
        const id = parseReferenceIdentifier(parser);
        const decl = parseTypeExpressionStatement(parser, start, id);

        if (decl === undefined) {
          throw new Error('Should have returned a node');
        }

        if (
          decl.type !== 'TSInterfaceDeclaration' &&
          decl.type !== 'TypeAliasTypeAnnotation' &&
          decl.type !== 'TSEnumDeclaration' &&
          decl.type !== 'FunctionDeclaration' &&
          decl.type !== 'ClassDeclaration' &&
          decl.type !== 'VariableDeclarationStatement' &&
          decl.type !== 'TSDeclareFunction' &&
          decl.type !== 'TSModuleDeclaration'
        ) {
          throw new Error(
            'Encountered a non-TS declare node when calling parseTypeExpressionStatement',
          );
        }

        return {...decl, declare: true};
      }
    }
  }

  parser.addDiagnostic({
    message: 'Unknown typescript declare start',
  });

  // Fake node
  const loc = parser.finishLoc(start);
  return {
    type: 'VariableDeclarationStatement',
    loc,
    declaration: {
      type: 'VariableDeclaration',
      loc,
      kind: 'var',
      declarations: [
        {
          type: 'VariableDeclarator',
          loc,
          id: toBindingIdentifier(
            parser.createUnknownIdentifier('typescript declare start', start),
          ),
          init: undefined,
        },
      ],
    },
  };
}

export function parseTSAbstractClass(
  parser: JSParser,
  start: Position,
): ClassDeclaration {
  return {
    ...parseClassDeclaration(parser, start),
    abstract: true,
  };
}

export function parseTSExportDefaultAbstractClass(
  parser: JSParser,
  start: Position,
): ClassDeclaration {
  return {
    ...parseExportDefaultClassDeclaration(parser, start),
    abstract: true,
  };
}

export function parseTSTypeArguments(
  parser: JSParser,
): TSTypeParameterInstantiation {
  const start = parser.getPosition();
  parser.pushScope('TYPE', true);

  const params = tsInNoContext(parser, () => {
    parser.expectRelational('<');
    return parseTSDelimitedList(
      parser,
      'TypeParametersOrArguments',
      parseTSType,
    );
  });

  // This reads the next token after the `>` too, so do parser.in the enclosing context.
  // But be sure not to parse a regex in the jsx expression `<C<number> />`, so set exprAllowed = false
  parser.state.exprAllowed = false;
  parser.popScope('TYPE');
  parser.expectRelational('>');

  return {
    loc: parser.finishLoc(start),
    type: 'TSTypeParameterInstantiation',
    params,
  };
}

export function isTSDeclarationStart(parser: JSParser): boolean {
  if (parser.match(tt.name)) {
    switch (parser.state.tokenValue) {
      case 'abstract':
      case 'declare':
      case 'enum':
      case 'interface':
      case 'module':
      case 'namespace':
      case 'type':
        return true;
    }
  }

  return false;
}

export function parseTSAccessModifier(
  parser: JSParser,
): undefined | ConstTSAccessibility {
  return parseTSModifier(parser, ['public', 'protected', 'private']);
}

export function isTSAbstractClass(parser: JSParser): boolean {
  return (
    parser.isContextual('abstract') &&
    parser.lookaheadState().tokenType === tt._class
  );
}

export function parseTSExport(
  parser: JSParser,
  start: Position,
):
  | undefined
  | TSNamespaceExportDeclaration
  | TSExportAssignment
  | TSImportEqualsDeclaration {
  if (!parser.isSyntaxEnabled('ts')) {
    return undefined;
  }

  if (parser.match(tt._import)) {
    // `export const A =B;`
    parser.expect(tt._import);
    return parseTSImportEqualsDeclaration(parser, start, /* isExport */ true);
  }

  if (parser.eat(tt.eq)) {
    // `export = x;`
    const expression = parseExpression(parser, 'ts export assignment');
    parser.semicolon();
    return {
      loc: parser.finishLoc(start),
      type: 'TSExportAssignment',
      expression,
    };
  }

  if (parser.eatContextual('as')) {
    // `export as namespace A;`
    // See `parseNamespaceExportDeclaration` in TypeScript's own parser
    parser.expectContextual('namespace');
    const id = parseIdentifier(parser);
    parser.semicolon();
    return {
      loc: parser.finishLoc(start),
      type: 'TSNamespaceExportDeclaration',
      id,
    };
  }
}
