/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSParser} from '../parser';
import {Position} from '@romejs/parser-core';
import {types as tt} from '../tokenizer/types';
import {
  AnyExportExternalSpecifier,
  AnyNode,
  AnyStatement,
  BindingIdentifier,
  ConstExportModuleKind,
  ConstImportModuleKind,
  ExportAllDeclaration,
  ExportDefaultDeclaration,
  ExportDefaultSpecifier,
  ExportExternalDeclaration,
  ExportExternalSpecifier,
  ExportLocalDeclaration,
  ExportLocalSpecifier,
  ExportNamespaceSpecifier,
  ImportDeclaration,
  ImportDefaultSpecifier,
  ImportNamespaceSpecifier,
  ImportSpecifier,
  ImportSpecifierLocal,
  StringLiteral,
  TSExportAssignment,
  TSImportEqualsDeclaration,
  TSNamespaceExportDeclaration,
} from '@romejs/js-ast';
import {getBindingIdentifiers} from '@romejs/js-ast-utils';
import {
  checkLVal,
  checkReservedType,
  checkReservedWord,
  hasTypeImportKind,
  isAsyncFunctionDeclarationStart,
  isLetStart,
  isMaybeDefaultImport,
  isTSAbstractClass,
  isTSDeclarationStart,
  parseBindingIdentifier,
  parseExportDefaultClassDeclaration,
  parseExportDefaultFunctionDeclaration,
  parseExpressionAtom,
  parseFlowOpaqueType,
  parseFlowRestrictedIdentifier,
  parseIdentifier,
  parseInterface,
  parseMaybeAssign,
  parseReferenceIdentifier,
  parseStatement,
  parseStringLiteral,
  parseTSExport,
  parseTSExportDefaultAbstractClass,
  parseTSImportEqualsDeclaration,
  parseTSInterfaceDeclaration,
  parseTypeAlias,
  toBindingIdentifier,
  toIdentifier,
} from './index';
import {descriptions} from '@romejs/diagnostics';

export type ParseExportResult =
  | AnyStatement
  | ExportAllDeclaration
  | ExportLocalDeclaration
  | ExportExternalDeclaration
  | ExportDefaultDeclaration
  | TSNamespaceExportDeclaration
  | TSExportAssignment
  | TSImportEqualsDeclaration;

export function parseExport(
  parser: JSParser,
  start: Position,
): ParseExportResult {
  const tsNode = parseTSExport(parser, start);
  if (tsNode !== undefined) {
    return tsNode;
  }

  let exportKind: ConstExportModuleKind = 'value';
  let declaration: undefined | AnyStatement;
  let localSpecifiers: undefined | Array<ExportLocalSpecifier>;

  // export * from '...'';
  if (shouldParseExportStar(parser)) {
    return parseExportStar(parser, start);
  } else if (isExportDefaultSpecifier(parser)) {
    const defStart = parser.getPosition();
    const defExported = parseIdentifier(parser, true);

    let namedSpecifiers: Array<ExportLocalSpecifier> = [];
    let defaultSpecifier: ExportDefaultSpecifier = parser.finishNode(
      defStart,
      {
        type: 'ExportDefaultSpecifier',
        exported: defExported,
      },
    );
    let namespaceSpecifier: undefined | ExportNamespaceSpecifier;

    if (parser.match(tt.comma) && parser.lookaheadState().tokenType === tt.star) {
      parser.expect(tt.comma);
      const specifierStart = parser.getPosition();
      parser.expect(tt.star);
      parser.expectContextual('as');
      const exported = parseIdentifier(parser);
      namespaceSpecifier = parser.finishNode(
        specifierStart,
        {
          type: 'ExportNamespaceSpecifier',
          exported,
        },
      );
    } else {
      namedSpecifiers = parseExportLocalSpecifiersMaybe(parser);
    }

    const source = parseExportFromExpect(parser);
    return createExportExternalDeclaration(
      parser,
      start,
      defaultSpecifier,
      namespaceSpecifier,
      namedSpecifiers,
      source,
    );
  } else if (parser.eat(tt._default)) {
    // export default ...
    const declaration = parseExportDefaultExpression(parser);
    checkExport(
      parser,
      {
        specifiers: localSpecifiers,
        declaration,
        isDefault: true,
      },
    );

    const node: ExportDefaultDeclaration = parser.finishNode(
      start,
      {
        type: 'ExportDefaultDeclaration',
        declaration,
      },
    );
    return node;
  } else if (shouldParseExportDeclaration(parser)) {
    let source;
    ({
      declaration,
      source,
      localSpecifiers,
      exportKind,
    } = parseExportDeclaration(parser));

    if (source !== undefined) {
      if (declaration !== undefined) {
        throw new Error(
          "When there's a source we don't also expect a declaration",
        );
      }

      return createExportExternalDeclaration(
        parser,
        start,
        undefined,
        undefined,
        localSpecifiers === undefined ? [] : localSpecifiers,
        source,
        exportKind,
      );
    }
  } else if (
    parser.isContextual('async') &&
    !isAsyncFunctionDeclarationStart(parser)
  ) {
    const next = parser.lookaheadState();

    parser.addDiagnostic({
      start: next.startPos,
      end: next.endPos,
      description: descriptions.JS_PARSER.EXPORT_ASYNC_NO_FUNCTION_KEYWORD,
    });
    declaration = undefined;
    localSpecifiers = [];
  } else {
    // export { x, y as z } [from '...']';
    localSpecifiers = parseExportSpecifiers(parser);

    const source = parseExportFrom(parser, false);
    if (source !== undefined) {
      return createExportExternalDeclaration(
        parser,
        start,
        undefined,
        undefined,
        localSpecifiers,
        source,
      );
    }
  }

  checkExport(
    parser,
    {
      specifiers: localSpecifiers,
      declaration,
      isDefault: false,
    },
  );

  if (declaration !== undefined) {
    if (
      declaration.type !== 'VariableDeclarationStatement' &&
      declaration.type !== 'ClassDeclaration' &&
      declaration.type !== 'FunctionDeclaration' &&
      declaration.type !== 'TSModuleDeclaration' &&
      declaration.type !== 'TSEnumDeclaration' &&
      declaration.type !== 'FlowInterfaceDeclaration' &&
      declaration.type !== 'TypeAliasTypeAnnotation' &&
      declaration.type !== 'TSInterfaceDeclaration' &&
      declaration.type !== 'TSDeclareFunction' &&
      declaration.type !== 'FlowOpaqueType'
    ) {
      parser.addDiagnostic({
        loc: declaration.loc,
        description: descriptions.JS_PARSER.INVALID_EXPORT_DECLARATION,
      });
      return declaration;
    }
  }

  const node: ExportLocalDeclaration = parser.finishNode(
    start,
    {
      type: 'ExportLocalDeclaration',
      exportKind,
      specifiers: localSpecifiers,
      declaration,
    },
  );
  return node;
}

function createExportExternalDeclaration(
  parser: JSParser,
  start: Position,
  defaultSpecifier: undefined | ExportDefaultSpecifier,
  namespaceSpecifier: undefined | ExportNamespaceSpecifier,
  namedSpecifiers: Array<ExportLocalSpecifier>,
  source: StringLiteral,
  exportKind?: ConstExportModuleKind,
): ExportExternalDeclaration {
  checkExport(
    parser,
    {
      specifiers: [defaultSpecifier, namespaceSpecifier, ...namedSpecifiers],
      declaration: undefined,
      isDefault: false,
      localIsExternal: true,
    },
  );

  const node = parser.finishNode(
    start,
    {
      type: 'ExportExternalDeclaration',
      exportKind,
      source,
      namedSpecifiers: [],
      defaultSpecifier,
      namespaceSpecifier,
    },
  );

  // We convert the specifiers after we've finished the ExportExternalDeclaration node
  // as the comment attachment logic may mess with the specifiers and so we need to
  // clone them after
  return {
    ...node,
    namedSpecifiers: convertLocalToExternalSpecifiers(parser, namedSpecifiers),
  };
}

function convertLocalToExternalSpecifiers(
  parser: JSParser,
  specifiers: Array<ExportLocalSpecifier> = [],
): Array<ExportExternalSpecifier> {
  return specifiers.map((specifier) => {
    return {
      ...specifier,
      type: 'ExportExternalSpecifier',
      local: toIdentifier(parser, specifier.local),
    };
  });
}

function parseExportDefaultExpression(
  parser: JSParser,
): ExportDefaultDeclaration['declaration'] {
  if (parser.isSyntaxEnabled('ts')) {
    if (isTSAbstractClass(parser)) {
      const start = parser.getPosition();
      parser.next(); // Skip 'abstract'
      return parseTSExportDefaultAbstractClass(parser, start);
    }

    if (parser.state.tokenValue === 'interface' && !parser.isLineTerminator()) {
      const start = parser.getPosition();
      parser.next();
      return parseTSInterfaceDeclaration(parser, start);
    }
  }

  const start = parser.getPosition();
  const isAsync = isAsyncFunctionDeclarationStart(parser);
  if (parser.eat(tt._function) || isAsync) {
    if (isAsync) {
      parser.eatContextual('async');
      parser.expect(tt._function);
    }

    return parseExportDefaultFunctionDeclaration(parser, start, isAsync);
  }

  if (parser.match(tt._class)) {
    return parseExportDefaultClassDeclaration(parser, start);
  }

  if (parser.match(tt._const) || parser.match(tt._var) || isLetStart(parser)) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.INVALID_EXPORT_DEFAULT,
    });
  }

  const res = parseMaybeAssign(parser, 'export default declaration');
  parser.semicolon();
  return res;
}

function parseExportDeclaration(
  parser: JSParser,
): {
  exportKind: ConstExportModuleKind;
  declaration?: AnyStatement;
  localSpecifiers?: Array<ExportLocalSpecifier>;
  source?: StringLiteral;
} {
  if (parser.isContextual('type')) {
    const start = parser.getPosition();
    parser.next();

    if (parser.match(tt.braceL)) {
      // export { foo, bar };
      const specifiers = parseExportSpecifiers(parser);
      const source = parseExportFrom(parser, false);
      return {
        exportKind: 'type',
        localSpecifiers: specifiers,
        source,
      };
    } else {
      // export type Foo = Bar;
      return {
        exportKind: 'type',
        declaration: parseTypeAlias(parser, start),
      };
    }
  }

  if (parser.isContextual('opaque')) {
    const declarationNode = parser.getPosition();
    parser.next();
    // export opaque type Foo = Bar;
    return {
      exportKind: 'type',
      declaration: parseFlowOpaqueType(parser, declarationNode, false),
    };
  }

  if (parser.isContextual('interface')) {
    const declarationNode = parser.getPosition();
    parser.next();
    return {
      exportKind: 'type',
      declaration: parseInterface(parser, declarationNode),
    };
  }

  return {
    exportKind: 'value',
    declaration: parseStatement(parser),
  };
}

function isExportDefaultSpecifier(parser: JSParser): boolean {
  const lookahead = parser.lookaheadState();
  if (
    lookahead.tokenType === tt.comma ||
    (lookahead.tokenType === tt.name && lookahead.tokenValue === 'from')
  ) {
    return true;
  }

  if (parser.isSyntaxEnabled('ts') && isTSDeclarationStart(parser)) {
    return false;
  }

  if (
    parser.match(tt.name) &&
    (parser.state.tokenValue === 'type' ||
    parser.state.tokenValue === 'interface' ||
    parser.state.tokenValue === 'opaque')
  ) {
    return false;
  }

  if (parser.match(tt.name)) {
    return (
      parser.state.tokenValue !== 'async' && parser.state.tokenValue !== 'let'
    );
  }

  if (!parser.match(tt._default)) {
    return false;
  }

  return false;
}

function parseExportLocalSpecifiersMaybe(
  parser: JSParser,
): Array<ExportLocalSpecifier> {
  if (parser.eat(tt.comma)) {
    return parseExportSpecifiers(parser);
  } else {
    return [];
  }
}

function parseExportFromExpect(parser: JSParser): StringLiteral {
  // @ts-ignore: `expect` parameter will always return a StringLiteral
  return parseExportFrom(parser, true);
}

function parseExportFrom(
  parser: JSParser,
  expect: boolean,
): undefined | StringLiteral {
  let source: undefined | StringLiteral;

  if (parser.eatContextual('from')) {
    if (parser.match(tt.string)) {
      source = parseStringLiteral(parser);
    } else {
      const expr = parseExpressionAtom(parser, 'export from');

      parser.addDiagnostic({
        loc: expr.loc,
        description: descriptions.JS_PARSER.EXPORT_FROM_NOT_STRING,
      });

      source = {
        type: 'StringLiteral',
        value: '',
        loc: expr.loc,
      };
    }
  } else if (expect) {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.EXPORT_MISSING_FROM,
    });

    source = {
      type: 'StringLiteral',
      value: '',
      loc: parser.finishLoc(parser.getPosition()),
    };
  }

  parser.semicolon();

  return source;
}

function shouldParseExportStar(parser: JSParser): boolean {
  return (
    parser.match(tt.star) ||
    (parser.isContextual('type') &&
    parser.lookaheadState().tokenType === tt.star)
  );
}

function parseExportStar(
  parser: JSParser,
  start: Position,
): ExportAllDeclaration | ExportLocalDeclaration | ExportExternalDeclaration {
  let exportKind: undefined | ConstExportModuleKind;
  if (parser.eatContextual('type')) {
    exportKind = 'type';
  }

  parser.expect(tt.star);

  if (parser.isContextual('as')) {
    const {source, namespaceSpecifier, namedSpecifiers} = parseExportNamespace(
      parser,
      exportKind,
    );
    return parser.finishNode(
      start,
      {
        type: 'ExportExternalDeclaration',
        namespaceSpecifier,
        exportKind,
        namedSpecifiers,
        source,
      },
    );
  } else {
    const source = parseExportFrom(parser, true);
    if (source === undefined) {
      throw new Error('Passed `true` above which expects there to be a string');
    }
    return parser.finishNode(
      start,
      {
        type: 'ExportAllDeclaration',
        exportKind,
        source,
      },
    );
  }
}

function parseExportNamespace(
  parser: JSParser,
  exportKind: undefined | ConstExportModuleKind,
): {
  source: StringLiteral;
  namespaceSpecifier: ExportNamespaceSpecifier;
  namedSpecifiers: Array<ExportExternalSpecifier>;
} {
  if (exportKind === 'type') {
    parser.addDiagnostic({
      description: descriptions.JS_PARSER.EXPORT_TYPE_NAMESPACE,
    });
  }

  const specifierStart = parser.state.lastStartPos;
  parser.next();
  const exported = parseIdentifier(parser, true);

  const namespaceSpecifier = parser.finishNode(
    specifierStart,
    {
      type: 'ExportNamespaceSpecifier',
      exported,
    },
  );

  const namedSpecifiers = convertLocalToExternalSpecifiers(
    parser,
    parseExportLocalSpecifiersMaybe(parser),
  );

  const source = parseExportFromExpect(parser);
  return {source, namespaceSpecifier, namedSpecifiers};
}

function shouldParseExportDeclaration(parser: JSParser): boolean {
  return (
    isTSDeclarationStart(parser) ||
    parser.isContextual('type') ||
    parser.isContextual('interface') ||
    parser.isContextual('opaque') ||
    parser.state.tokenType.keyword === 'var' ||
    parser.state.tokenType.keyword === 'const' ||
    parser.state.tokenType.keyword === 'function' ||
    parser.state.tokenType.keyword === 'class' ||
    isLetStart(parser) ||
    isAsyncFunctionDeclarationStart(parser) ||
    parser.match(tt.at)
  );
}

function checkExport(
  parser: JSParser,
  {
    specifiers,
    declaration,
    localIsExternal = false,
    isDefault = false,
  }: {
    localIsExternal?: boolean;
    specifiers?: Array<
      undefined | ExportLocalSpecifier | AnyExportExternalSpecifier
    >;
    declaration?: AnyNode;
    isDefault: boolean;
  },
): void {
  // Check for duplicate exports
  if (isDefault) {
    // Default exports
    if (declaration !== undefined) {
      checkDuplicateExports(parser, declaration, 'default');
    }
    return undefined;
  }

  if (declaration !== undefined) {
    // Exported declarations
    if (declaration.type === 'FunctionDeclaration') {
      if (declaration.id === undefined) {
        throw new Error('Expected declaration.id');
      }

      checkDuplicateExports(parser, declaration, declaration.id.name);
    }

    if (declaration.type === 'ClassDeclaration') {
      if (declaration.id === undefined) {
        throw new Error('Expected declaration.id');
      }

      checkDuplicateExports(parser, declaration, declaration.id.name);
    }

    if (declaration.type === 'VariableDeclaration') {
      for (const node of getBindingIdentifiers(declaration)) {
        checkDuplicateExports(parser, node, node.name);
      }
    }
  }

  if (specifiers !== undefined) {
    // Named exports
    for (const specifier of specifiers) {
      if (specifier === undefined) {
        continue;
      }

      checkDuplicateExports(parser, specifier, specifier.exported.name);

      if (specifier.type === 'ExportLocalSpecifier' && !localIsExternal) {
        const {local} = specifier;
        if (local !== undefined) {
          // check for keywords used as local names
          checkReservedWord(
            parser,
            local.name,
            parser.getLoc(local),
            true,
            false,
          );
        }
      }
    }
  }
}

function checkDuplicateExports(
  parser: JSParser,
  node: AnyNode,
  name: string,
): void {
  if (parser.isSyntaxEnabled('ts')) {
    // Refer to checkReservedWord for an explanation
    return undefined;
  }

  const existing = parser.state.exportedIdentifiers.get(name);
  if (existing !== undefined) {
    parser.addDiagnostic({
      loc: node.loc,
      description: descriptions.JS_PARSER.DUPLICATE_EXPORT(name, existing),
    });
  }

  parser.state.exportedIdentifiers.set(name, parser.getLoc(node));
}

// Parses a comma-separated list of module exports.
function parseExportSpecifiers(parser: JSParser): Array<ExportLocalSpecifier> {
  const specifiers: Array<ExportLocalSpecifier> = [];
  let first = true;

  // export { x, y as z } [from '...']';
  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'export specifiers',
  );

  while (true) {
    if (parser.match(tt.braceR) || parser.match(tt.eof)) {
      parser.expectClosing(openContext);
      break;
    }

    if (first) {
      first = false;
    } else {
      parser.expect(tt.comma);
      if (parser.eat(tt.braceR)) {
        break;
      }
    }

    const start = parser.getPosition();
    const local = parseReferenceIdentifier(parser, true);
    const exported = parser.eatContextual('as')
      ? parseIdentifier(parser, true)
      : toIdentifier(parser, parser.cloneNode(local));
    specifiers.push(
      parser.finishNode(
        start,
        {
          type: 'ExportLocalSpecifier',
          local,
          exported,
          // TODO exportKind?
        },
      ),
    );
  }

  return specifiers;
}

export type ParseImportResult = ImportDeclaration | TSImportEqualsDeclaration;

export function parseImport(
  parser: JSParser,
  start: Position,
): ParseImportResult {
  if (parser.match(tt.name) && parser.lookaheadState().tokenType === tt.eq) {
    return parseTSImportEqualsDeclaration(parser, start);
  }

  let namedSpecifiers: Array<ImportSpecifier> = [];
  let namespaceSpecifier: undefined | ImportNamespaceSpecifier;
  let defaultSpecifier: undefined | ImportDefaultSpecifier;
  let source: StringLiteral;
  let importKind: undefined | ConstImportModuleKind;

  // import '...'
  if (parser.match(tt.string)) {
    source = parseStringLiteral(parser);
  } else {
    ({
      namedSpecifiers,
      namespaceSpecifier,
      defaultSpecifier,
      importKind,
    } = parseImportSpecifiers(parser, start));

    if (parser.expectContextual('from') && parser.match(tt.string)) {
      source = parseStringLiteral(parser);
    } else {
      parser.addDiagnostic({
        description: descriptions.JS_PARSER.IMPORT_MISSING_SOURCE,
      });

      source = parser.finishNode(
        start,
        {
          type: 'StringLiteral',
          value: '',
        },
      );
    }
  }

  parser.semicolon();
  return parser.finishNode(
    start,
    {
      type: 'ImportDeclaration',
      namedSpecifiers,
      namespaceSpecifier,
      defaultSpecifier,
      source,
      importKind,
    },
  );
}

// eslint-disable-next-line no-unused-vars
function shouldParseDefaultImport(
  parser: JSParser,
  kind: undefined | ConstImportModuleKind,
): boolean {
  if (hasTypeImportKind(kind)) {
    return isMaybeDefaultImport(parser.state);
  } else {
    return parser.match(tt.name);
  }
}

function parseImportSpecifierLocal(
  parser: JSParser,
  importKind: undefined | ConstImportModuleKind,
  contextDescription: string,
): ImportSpecifierLocal {
  const start = parser.getPosition();

  const local = hasTypeImportKind(importKind)
    ? parseFlowRestrictedIdentifier(parser, true)
    : parseBindingIdentifier(parser);

  checkLVal(parser, local, true, undefined, contextDescription);

  return parser.finishNode(
    start,
    {
      type: 'ImportSpecifierLocal',
      name: local,
      importKind,
    },
  );
}

// Parses a comma-separated list of module imports.
function parseImportSpecifiers(
  parser: JSParser,
  start: Position,
): {
  namedSpecifiers: Array<ImportSpecifier>;
  namespaceSpecifier: undefined | ImportNamespaceSpecifier;
  defaultSpecifier: undefined | ImportDefaultSpecifier;
  importKind: undefined | ConstImportModuleKind;
} {
  let importKind: undefined | ConstImportModuleKind = undefined;

  // Ensure that when parsing `import from './type.js` we don't mistakenly think it's an import type';

  // TODO probably need to check for a comma and `as`
  const lh = parser.lookaheadState();
  if (
    lh.tokenType !== tt.name ||
    (lh.tokenType === tt.name && lh.tokenValue !== 'from')
  ) {
    if (parser.match(tt._typeof)) {
      importKind = 'typeof';
    } else if (parser.isContextual('type')) {
      importKind = 'type';
    }
  }

  if (importKind) {
    if (importKind === 'type' && lh.tokenType === tt.star) {
      parser.addDiagnostic({
        start: lh.startPos,
        description: descriptions.JS_PARSER.IMPORT_TYPE_STAR,
      });
    }

    if (
      isMaybeDefaultImport(lh) ||
      lh.tokenType === tt.braceL ||
      lh.tokenType === tt.star
    ) {
      parser.next();
    }
  }

  let namedSpecifiers: Array<ImportSpecifier> = [];
  let namespaceSpecifier: undefined | ImportNamespaceSpecifier;
  let defaultSpecifier: undefined | ImportDefaultSpecifier;

  let first = true;

  // import defaultObj, { x, y as z } from '...'';
  if (shouldParseDefaultImport(parser, importKind)) {
    const meta = parseImportSpecifierLocal(
      parser,
      importKind,
      'default import specifier',
    );

    defaultSpecifier = parser.finishNode(
      start,
      {
        type: 'ImportDefaultSpecifier',
        local: meta,
      },
    );

    if (!parser.eat(tt.comma)) {
      return {
        namedSpecifiers,
        namespaceSpecifier,
        defaultSpecifier,
        importKind,
      };
    }
  }

  if (parser.match(tt.star)) {
    parser.next();
    parser.expectContextual('as');

    const meta = parseImportSpecifierLocal(
      parser,
      importKind,
      'import namespace specifier',
    );

    namespaceSpecifier = parser.finishNode(
      start,
      {
        type: 'ImportNamespaceSpecifier',
        local: meta,
      },
    );

    return {namedSpecifiers, namespaceSpecifier, defaultSpecifier, importKind};
  }

  const openContext = parser.expectOpening(
    tt.braceL,
    tt.braceR,
    'import specifiers',
  );

  while (true) {
    if (parser.match(tt.braceR) || parser.match(tt.eof)) {
      parser.expectClosing(openContext);
      break;
    }

    if (first) {
      first = false;
    } else {
      // Detect an attempt to deep destructure
      if (parser.eat(tt.colon)) {
        parser.addDiagnostic({
          description: descriptions.JS_PARSER.DESTRUCTURING_IN_IMPORT,
        });
      }

      parser.expect(tt.comma);

      if (parser.eat(tt.braceR)) {
        break;
      }
    }

    namedSpecifiers.push(parseImportSpecifier(parser, importKind));
  }

  return {namedSpecifiers, namespaceSpecifier, defaultSpecifier, importKind};
}

function parseImportSpecifier(
  parser: JSParser,
  nodeKind: undefined | ConstImportModuleKind,
): ImportSpecifier {
  const start = parser.getPosition();
  const firstIdentPos = parser.state.startPos;
  const firstIdent = parseIdentifier(parser, true);

  let imported;
  let local: BindingIdentifier;
  let importKind: undefined | ConstImportModuleKind = undefined;
  if (firstIdent.name === 'type') {
    importKind = 'type';
  } else if (firstIdent.name === 'typeof') {
    importKind = 'typeof';
  }

  let isBinding = false;
  if (parser.isContextual('as') && !parser.isLookaheadContextual('as')) {
    const asIdent = parseIdentifier(parser, true);
    if (
      importKind !== undefined &&
      !parser.match(tt.name) &&
      parser.state.tokenType.keyword === undefined
    ) {
      // `import {type as ,` or `import {type as }`
      imported = asIdent;
      local = toBindingIdentifier(parser, parser.cloneNode(asIdent));
    } else {
      // `import {type as foo`
      imported = firstIdent;
      importKind = undefined;
      local = parseBindingIdentifier(parser);
    }
  } else if (
    importKind !== undefined &&
    (parser.match(tt.name) || parser.state.tokenType.keyword)
  ) {
    // `import {type foo`
    imported = parseIdentifier(parser, true);
    if (parser.eatContextual('as')) {
      local = parseBindingIdentifier(parser);
    } else {
      isBinding = true;
      local = toBindingIdentifier(parser, parser.cloneNode(imported));
    }
  } else {
    isBinding = true;
    imported = firstIdent;
    importKind = undefined;
    local = toBindingIdentifier(parser, parser.cloneNode(imported));
  }

  const nodeIsTypeImport = hasTypeImportKind(nodeKind);
  const specifierIsTypeImport = hasTypeImportKind(importKind);

  if (nodeIsTypeImport && specifierIsTypeImport) {
    parser.addDiagnostic({
      start: firstIdentPos,
      description: descriptions.JS_PARSER.IMPORT_KIND_SPECIFIER_ON_IMPORT_DECLARATION_WITH_KIND,
    });
  }

  const loc = parser.finishLoc(start);

  if (nodeIsTypeImport || specifierIsTypeImport) {
    checkReservedType(parser, local.name, loc);
  }

  if (isBinding && !nodeIsTypeImport && !specifierIsTypeImport) {
    checkReservedWord(parser, local.name, loc, true, true);
  }

  checkLVal(parser, local, true, undefined, 'import specifier');

  return parser.finishNode(
    start,
    {
      type: 'ImportSpecifier',
      imported,
      local: parser.finishNode(
        start,
        {
          type: 'ImportSpecifierLocal',
          name: local,
          importKind,
        },
      ),
    },
  );
}
