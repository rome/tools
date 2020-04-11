/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  ClassPropertyMeta,
  PrivateName,
  ClassHead,
  AnyObjectPropertyKey,
  FlowVariance,
  BindingIdentifier,
  ConstTSAccessibility,
  ClassExpression,
  ClassDeclaration,
  AnyClassMember,
  StaticPropertyKey,
  ClassMethodKind,
  ClassProperty,
  ClassPrivateProperty,
  AnyExpression,
  ClassMethod,
  TSDeclareMethod,
  ClassPrivateMethod,
  TSTypeParameterDeclaration,
  FlowTypeParameterDeclaration,
  AnyTypeArguments,
  TSExpressionWithTypeArguments,
  FlowClassImplements,
} from '@romejs/js-ast';
import {Position, SourceLocation} from '@romejs/parser-core';
import {JSParser} from '../parser';
import {types as tt} from '../tokenizer/types';
import {
  parseTSAccessModifier,
  parseIdentifier,
  maybeParseTypeParameters,
  parseTSModifier,
  hasTSModifier,
  checkGetterSetterParamCount,
  parsePrimaryTypeAnnotation,
  parseObjectPropertyKey,
  parseMethod,
  tryTSParseIndexSignature,
  parseMaybeAssign,
  parseExpressionWithPossibleSubscripts,
  maybeParseTypeArguments,
  parseClassImplements,
} from './index';
import {inc, dec} from '@romejs/ob1';
import {parseBindingIdentifier, toBindingIdentifier} from './expression';
import {descriptions} from '@romejs/diagnostics';

export function parseClassExpression(
  parser: JSParser,
  start: Position,
): ClassExpression {
  return parser.finalizeNode({
    ...parseClass(parser, start, true),
    type: 'ClassExpression',
  });
}

export function parseExportDefaultClassDeclaration(
  parser: JSParser,
  start: Position,
): ClassDeclaration {
  let {id, ...shape} = parseClass(parser, start, true);

  if (id === undefined) {
    id = {
      type: 'BindingIdentifier',
      name: '*default*',
      // Does this `loc` make sense?
      loc: shape.loc,
    };
  }

  return parser.finalizeNode({
    ...shape,
    type: 'ClassDeclaration',
    id,
  });
}

export function parseClassDeclaration(
  parser: JSParser,
  start: Position,
): ClassDeclaration {
  const {id, ...shape} = parseClass(parser, start, false);

  if (id === undefined) {
    throw new Error('Expected id');
  }

  return parser.finalizeNode({
    ...shape,
    type: 'ClassDeclaration',
    id,
  });
}

// Parse a class declaration or expression
export function parseClass(
  parser: JSParser,
  start: Position,
  optionalId: boolean,
): {
  loc: SourceLocation;
  meta: ClassHead;
  id: undefined | BindingIdentifier;
} {
  parser.pushScope('METHOD', false);
  parser.pushScope('STRICT', true);

  parser.next();
  const {id, typeParameters} = parseClassId(parser, optionalId);
  const {superClass, superTypeParameters, implemented} = parseClassSuper(parser);

  parser.pushScope('CLASS', superClass === undefined ? 'normal' : 'derived');

  const body = parseClassBody(parser);

  parser.popScope('CLASS');
  parser.popScope('STRICT');
  parser.popScope('METHOD');

  const meta: ClassHead = parser.finishNode(start, {
    type: 'ClassHead',
    body,
    typeParameters,
    superClass,
    superTypeParameters,
    implements: implemented,
  });

  return {
    loc: parser.finishLoc(start),
    id,
    meta,
  };
}

function isClassProperty(parser: JSParser): boolean {
  return parser.match(tt.bang) || parser.match(tt.colon) || parser.match(tt.eq) ||
        parser.match(tt.semi) ||
      parser.match(tt.braceR);
}

function isClassMethod(parser: JSParser): boolean {
  return parser.match(tt.parenL) || parser.isRelational('<');
}

function isNonstaticConstructor(
  parser: JSParser,
  key: AnyObjectPropertyKey,
  meta: ClassPropertyMeta,
): boolean {
  // Class property
  if (parser.match(tt.colon)) {
    return false;
  }

  // Static
  if (meta.static) {
    return false;
  }

  if (key.type === 'StaticPropertyKey' && key.value.type === 'Identifier' &&
        key.value.name ===
        'constructor') {
    return true;
  }

  if (key.value.type === 'StringLiteral' && key.value.value === 'constructor') {
    return true;
  }

  return false;
}

type ClassBodyState = {hadConstructor: boolean};

function parseClassBody(parser: JSParser): Array<AnyClassMember> {
  // class bodies are implicitly strict
  parser.pushScope('STRICT', true);
  parser.state.classLevel = inc(parser.state.classLevel);

  const state: ClassBodyState = {hadConstructor: false};

  const body = [];

  const openContext = parser.expectOpening(tt.braceL, tt.braceR, 'class body');

  while (true) {
    if (parser.match(tt.braceR) || parser.match(tt.eof)) {
      break;
    }

    if (parser.eat(tt.semi)) {
      continue;
    }

    const member = parseClassMember(parser, state);
    if (member !== undefined) {
      body.push(member);
    }
  }

  parser.expectClosing(openContext);

  parser.state.classLevel = dec(parser.state.classLevel);
  parser.popScope('STRICT');

  return body;
}

function parseClassMember(
  parser: JSParser,
  state: ClassBodyState,
): undefined | AnyClassMember {
  const start = parser.getPosition();
  const escapePosition = parser.state.escapePosition;

  let accessibility: undefined | ConstTSAccessibility;
  if (parser.isSyntaxEnabled('ts')) {
    accessibility = parseTSAccessModifier(parser);
  }

  let isStatic = false;
  if (parser.match(tt.name) && parser.state.tokenValue === 'static') {
    const keyId = parseIdentifier(parser, true); // eats 'static'
    const key: StaticPropertyKey = {
      type: 'StaticPropertyKey',
      value: keyId,
      loc: keyId.loc,
    };

    const meta: ClassPropertyMeta = parser.finishNode(start, {
      type: 'ClassPropertyMeta',
      static: false,
      typeAnnotation: undefined,
      accessibility,
      optional: false,
      abstract: false,
      readonly: false,
    });

    if (isClassMethod(parser)) {
      // A method named 'static'
      return parseClassMethod(parser, {
        start,
        meta,
        key,
        kind: 'method',
        isStatic: false,
        isAsync: false,
        isGenerator: false,
        isConstructor: false,
      });
    }

    if (isClassProperty(parser)) {
      // A property named 'static'
      return parseClassProperty(parser, start, key, meta);
    }

    if (escapePosition !== undefined) {
      parser.addDiagnostic({
        index: escapePosition,
        description: descriptions.JS_PARSER.ESCAPE_SEQUENCE_IN_WORD('static'),
      });
    }

    // Otherwise something static
    isStatic = true;
  }

  return parseClassMemberWithIsStatic(
    parser,
    start,
    state,
    isStatic,
    accessibility,
  );
}

function parseClassMemberWithIsStatic(
  parser: JSParser,
  start: Position,
  state: ClassBodyState,
  isStatic: boolean,
  accessibility: undefined | ConstTSAccessibility,
): undefined | AnyClassMember {
  let abstract = false;
  let readonly = false;

  const mod = parseTSModifier(parser, ['abstract', 'readonly']);
  switch (mod) {
    case 'readonly': {
      readonly = true;
      abstract = hasTSModifier(parser, ['abstract']);
      break;
    }

    case 'abstract': {
      abstract = true;
      readonly = hasTSModifier(parser, ['readonly']);
      break;
    }
  }

  const nameOpts = {
    start,
    static: isStatic,
    accessibility,
    readonly,
    abstract,
  };

  if (!abstract && !isStatic && accessibility === undefined) {
    const indexSignature = tryTSParseIndexSignature(parser, start);
    if (indexSignature) {
      return {
        ...indexSignature,
        readonly,
      };
    }
  }

  // Must be a property (if not an index signature).
  if (readonly) {
    const {key, meta} = parseClassPropertyMeta(parser, nameOpts);
    if (key.value.type === 'PrivateName') {
      return parseClassPrivateProperty(parser, start, key.value, meta);
    } else {
      return pushClassProperty(parser, start, key, meta);
    }
  }

  // Generator methods
  if (parser.eat(tt.star)) {
    const {meta, key} = parseClassPropertyMeta(parser, nameOpts);

    if (key.value.type === 'PrivateName') {
      // Private generator method
      return parseClassPrivateMethod(parser, {
        start,
        key: key.value,
        variance: undefined,
        meta,
        isGenerator: true,
        isAsync: false,
        kind: 'method',
      });
    }

    if (isNonstaticConstructor(parser, key, meta)) {
      parser.addDiagnostic({
        loc: key.loc,
        description: descriptions.JS_PARSER.GENERATOR_CLASS_CONSTRUCTOR,
      });
    }

    return parseClassMethod(parser, {
      start,
      key,
      meta,
      kind: 'method',
      isStatic: false,
      isGenerator: true,
      isAsync: false,
      isConstructor: false,
    });
  }

  const escapePosition = parser.state.escapePosition;
  const {meta, key} = parseClassPropertyMeta(parser, nameOpts);

  // Regular method
  if (isClassMethod(parser)) {
    // Private method
    if (key.value.type === 'PrivateName') {
      return parseClassPrivateMethod(parser, {
        start,
        key: key.value,
        meta,
        isGenerator: false,
        isAsync: false,
        kind: 'method',
        variance: undefined,
      });
    }

    const isConstructor = isNonstaticConstructor(parser, key, meta);

    let kind: ClassMethodKind = 'method';
    if (isConstructor) {
      kind = 'constructor';

      // TypeScript allows multiple overloaded constructor declarations
      if (state.hadConstructor && !parser.isSyntaxEnabled('ts')) {
        parser.addDiagnostic({
          loc: key.loc,
          description: descriptions.JS_PARSER.DUPLICATE_CLASS_CONSTRUCTOR,
        });
      }
      state.hadConstructor = true;
    }

    return parseClassMethod(parser, {
      start,
      key,
      meta,
      kind,
      isStatic,
      isGenerator: false,
      isAsync: false,
      isConstructor,
    });
  }

  // Class property
  if (isClassProperty(parser)) {
    if (key.value.type === 'PrivateName') {
      return parseClassPrivateProperty(parser, start, key.value, meta);
    } else {
      return pushClassProperty(parser, start, key, meta);
    }
  }

  // Async method
  if (key.value.type === 'Identifier' && key.value.name === 'async' &&
      !parser.isLineTerminator()) {
    parser.banUnicodeEscape(escapePosition, 'async');

    // an async method
    const isGenerator = parser.eat(tt.star);

    // The so-called parsed name would have been "async": get the real name.
    const {meta, key} = parseClassPropertyMeta(parser, nameOpts);

    if (key.value.type === 'PrivateName') {
      // private async method
      return parseClassPrivateMethod(parser, {
        start,
        key: key.value,
        meta,
        isGenerator,
        isAsync: true,
        kind: 'method',
        variance: undefined,
      });
    } else {
      const method = parseClassMethod(parser, {
        start,
        key,
        meta,
        kind: 'method',
        isStatic,
        isGenerator,
        isAsync: true,
        isConstructor: false,
      });

      if (isNonstaticConstructor(parser, key, meta)) {
        parser.addDiagnostic({
          loc: key.loc,
          description: descriptions.JS_PARSER.ASYNC_CLASS_CONSTRUCTOR,
        });
      }

      return method;
    }
  }

  // Getter/setter method
  if (key.value.type === 'Identifier' && (key.value.name === 'get' ||
        key.value.name ===
        'set') && !(parser.isLineTerminator() && parser.match(tt.star))) {
    // `get\n*` is an uninitialized property named 'get' followed by a generator.

    // a getter or setter
    const kind: 'get' | 'set' = key.value.name;
    parser.banUnicodeEscape(escapePosition, kind);

    // The so-called parsed name would have been "get/set": get the real name.
    const {meta, key: methodKey} = parseClassPropertyMeta(parser, nameOpts);

    if (methodKey.value.type === 'PrivateName') {
      // private getter/setter
      const method = parseClassPrivateMethod(parser, {
        start,
        key: methodKey.value,
        meta,
        isGenerator: false,
        isAsync: false,
        kind,
        variance: undefined,
      });
      checkGetterSetterParamCount(parser, method, method.kind);
      return method;
    } else {
      const method = parseClassMethod(parser, {
        start,
        key: methodKey,
        meta,
        kind,
        isStatic: false,
        isGenerator: false,
        isAsync: false,
        isConstructor: false,
      });

      if (isNonstaticConstructor(parser, key, meta)) {
        parser.addDiagnostic({
          loc: methodKey.loc,
          description: descriptions.JS_PARSER.GET_SET_CLASS_CONSTRUCTOR,
        });
      }

      checkGetterSetterParamCount(parser, method, method.kind);
      return method;
    }
  }

  if (parser.isLineTerminator()) {
    // an uninitialized class property (due to ASI, since we don't otherwise recognize the next token)
    if (key.value.type === 'PrivateName') {
      return parseClassPrivateProperty(parser, start, key.value, meta);
    } else {
      return pushClassProperty(parser, start, key, meta);
    }
  }

  parser.addDiagnostic({
    description: descriptions.JS_PARSER.UNKNOWN_CLASS_PROPERTY_START,
  });
  return undefined;
}

function parseClassPropertyMeta(parser: JSParser, opts: {
  start: Position;
  static: boolean;
  accessibility: undefined | ConstTSAccessibility;
  readonly: boolean;
  abstract: boolean;
}): {
  key: AnyObjectPropertyKey;
  meta: ClassPropertyMeta;
} {
  let typeAnnotation;
  if (parser.match(tt.colon)) {
    typeAnnotation = parsePrimaryTypeAnnotation(parser);
  }

  const key = parseObjectPropertyKey(parser);

  if (key.type === 'StaticPropertyKey' && opts.static === true &&
        key.value.type ===
        'Identifier' && key.value.name === 'prototype') {
    parser.addDiagnostic({
      loc: key.loc,
      description: descriptions.JS_PARSER.CLASS_STATIC_PROTOTYPE_PROPERTY,
    });
  }

  if (key.value.type === 'PrivateName' && key.value.id.name === 'constructor') {
    parser.addDiagnostic({
      loc: key.loc,
      description: descriptions.JS_PARSER.CLASS_PRIVATE_FIELD_NAMED_CONSTRUCTOR,
    });
  }

  let optional = false;
  if (parser.match(tt.question)) {
    optional = true;
    parser.expectSyntaxEnabled('ts');
    parser.next();
  }

  return {
    key,
    meta: parser.finishNode(opts.start, {
      type: 'ClassPropertyMeta',
      typeAnnotation,
      optional,
      ...opts,
    }),
  };
}

function pushClassProperty(
  parser: JSParser,
  start: Position,
  key: AnyObjectPropertyKey,
  meta: ClassPropertyMeta,
): ClassProperty {
  // This only affects properties, not methods.
  if (isNonstaticConstructor(parser, key, meta)) {
    parser.addDiagnostic({
      loc: key.loc,
      description: descriptions.JS_PARSER.CLASS_PROPERTY_NAME_CONSTRUCTOR,
    });
  }

  return parseClassProperty(parser, start, key, meta);
}

function parseClassMethod(parser: JSParser, opts: {
  start: Position;
  meta: ClassPropertyMeta;
  key: AnyObjectPropertyKey;
  kind: ClassMethodKind;
  isStatic: boolean;
  isGenerator: boolean;
  isAsync: boolean;
  isConstructor: boolean;
}): ClassMethod | TSDeclareMethod {
  const {start, key, meta, kind, isGenerator, isAsync, isConstructor} = opts;

  if (key.variance !== undefined) {
    parser.addDiagnostic({
      loc: key.variance.loc,
      description: descriptions.JS_PARSER.ILLEGAL_VARIANCE,
    });
  }

  const typeParameters = maybeParseTypeParameters(parser);

  const {head, body} = parseMethod(parser, {
    kind,
    isClass: true,
    isGenerator,
    isAsync,
    isConstructor,
  });

  const method: Omit<ClassMethod, 'type' | 'body'> = {
    head: {
      ...head,
      typeParameters,
    },
    loc: parser.finishLoc(start),
    kind,
    key,
    meta,
  };

  if (body === undefined) {
    return parser.finalizeNode({
      ...method,
      type: 'TSDeclareMethod',
      body: undefined,
    });
  } else {
    if (body.type !== 'BlockStatement') {
      throw new Error('Expected BlockStatement body');
    }

    if (key.value.type === 'PrivateName') {
      throw new Error('Expected to hit other private methods instead');
    }

    return parser.finalizeNode({
      ...method,
      body,
      type: 'ClassMethod',
    });
  }
}

function parseClassPrivateMethod(parser: JSParser, opts: {
  key: PrivateName;
  start: Position;
  meta: ClassPropertyMeta;
  isGenerator: boolean;
  isAsync: boolean;
  kind: ClassMethodKind;
  variance: undefined | FlowVariance;
}): ClassPrivateMethod {
  const {start, key, variance, meta, isGenerator, isAsync, kind} = opts;

  if (variance !== undefined) {
    parser.addDiagnostic({
      loc: variance.loc,
      description: descriptions.JS_PARSER.ILLEGAL_VARIANCE,
    });
  }

  const typeParameters = maybeParseTypeParameters(parser);
  const method = parseMethod(parser, {
    kind,
    isClass: true,
    isGenerator,
    isAsync,
    isConstructor: false,
  });

  const {body} = method;
  if (body === undefined || body.type !== 'BlockStatement') {
    throw new Error('Expected body');
  }

  return parser.finishNode(start, {
    ...method,
    body,
    meta,
    key,
    kind,
    type: 'ClassPrivateMethod',
    variance,
    head: {
      ...method.head,
      typeParameters,
    },
  });
}

function parseClassPrivateProperty(
  parser: JSParser,
  start: Position,
  key: PrivateName,
  meta: ClassPropertyMeta,
): ClassPrivateProperty {
  parser.pushScope('CLASS_PROPERTY', true);

  let typeAnnotation;
  if (parser.match(tt.colon)) {
    typeAnnotation = parsePrimaryTypeAnnotation(parser);
  }

  const value: undefined | AnyExpression = parser.eat(tt.eq)
    ? parseMaybeAssign<AnyExpression>(parser, 'class private property value')
    : undefined;
  parser.semicolon();
  parser.popScope('CLASS_PROPERTY');

  return parser.finishNode(start, {
    meta,
    key,
    type: 'ClassPrivateProperty',
    value,
    typeAnnotation,
  });
}

function parseClassProperty(
  parser: JSParser,
  start: Position,
  key: AnyObjectPropertyKey,
  meta: ClassPropertyMeta,
): ClassProperty {
  // TODO maybe parsing should be abstracted for private class properties too?
  let definite;
  if (!meta.optional && parser.eat(tt.bang)) {
    definite = true;
    parser.expectSyntaxEnabled('ts');
  }

  let typeAnnotation;
  if (parser.match(tt.colon)) {
    typeAnnotation = parsePrimaryTypeAnnotation(parser);
  }

  parser.pushScope('CLASS_PROPERTY', true);

  let value: undefined | AnyExpression;
  if (parser.match(tt.eq)) {
    parser.next();
    value = parseMaybeAssign<AnyExpression>(parser, 'class property value');
  }
  parser.semicolon();

  parser.popScope('CLASS_PROPERTY');

  if (key.value.type === 'PrivateName') {
    throw new Error(
        'PrivateName encountered in regular parseClassProperty, expects method is parsePrivateClassProperty',
      );
  }

  return parser.finishNode(start, {
    meta,
    key,
    type: 'ClassProperty',
    definite,
    typeAnnotation,
    value,
  });
}

function parseClassId(
  parser: JSParser,
  optionalId: boolean,
): {
  id: undefined | BindingIdentifier;
  typeParameters:
    | undefined
    | TSTypeParameterDeclaration
    | FlowTypeParameterDeclaration;
} {
  let idAllowed = true;

  // Allow `class implements Foo {}` in class expressions
  if (optionalId === true && parser.isContextual('implements')) {
    idAllowed = false;
  }

  let id;
  if (idAllowed) {
    if (parser.match(tt.name)) {
      id = parseBindingIdentifier(parser);
    } else if (!optionalId) {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.REQUIRED_CLASS_NAME,
      });
      id = toBindingIdentifier(parser, parser.createUnknownIdentifier(
        'required class name',
      ));
    }
  }

  const typeParameters = maybeParseTypeParameters(parser, true);
  return {id, typeParameters};
}

function parseClassSuper(parser: JSParser): {
  superClass: undefined | AnyExpression;
  superTypeParameters: undefined | AnyTypeArguments;
  implemented: ClassHead['implements'];
} {
  let superClass = parser.eat(tt._extends)
    ? parseExpressionWithPossibleSubscripts(parser, 'class heritage')
    : undefined;
  let superTypeParameters;

  if (superClass !== undefined) {
    superTypeParameters = maybeParseTypeArguments(parser);
  }

  let implemented: undefined | Array<
    | FlowClassImplements
    | TSExpressionWithTypeArguments>;
  if (parser.isContextual('implements')) {
    parser.next();
    implemented = parseClassImplements(parser);
  }

  return {superClass, superTypeParameters, implemented};
}
