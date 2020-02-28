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
  AnyStatement,
  ExportAllDeclaration,
  ExportNamedDeclaration,
  ExportDefaultDeclaration,
  TSNamespaceExportDeclaration,
  TSExportAssignment,
  TSImportEqualsDeclaration,
  ConstExportModuleKind,
  ExportLocalSpecifier,
  AnyExportExternalSpecifier,
  ExportExternalDeclaration,
  StringLiteral,
  ImportDeclaration,
  AnyNode,
  ConstImportModuleKind,
  BindingIdentifier,
  AnyImportSpecifier,
  ImportSpecifier,
  ImportDefaultSpecifier,
  ImportNamespaceSpecifier,
  ImportSpecifierLocal,
} from '@romejs/js-ast';
import {getBindingIdentifiers} from '@romejs/js-ast-utils';
import {
  parseTSExport,
  parseIdentifier,
  isAsyncFunctionDeclarationStart,
  isTSAbstractClass,
  parseTSInterfaceDeclaration,
  isLetStart,
  parseMaybeAssign,
  parseTypeAlias,
  parseFlowOpaqueType,
  parseInterface,
  parseStatement,
  isTSDeclarationStart,
  parseExpressionAtom,
  checkReservedWord,
  hasTypeImportKind,
  parseTSImportEqualsDeclaration,
  parseFlowRestrictedIdentifier,
  checkLVal,
  isMaybeDefaultImport,
  checkReservedType,
  parseStringLiteral,
  parseBindingIdentifier,
  toBindingIdentifier,
  parseReferenceIdentifier,
  toIdentifier,
  parseTSExportDefaultAbstractClass,
  parseExportDefaultFunctionDeclaration,
  parseExportDefaultClassDeclaration,
} from './index';

export type ParseExportResult =
  | AnyStatement
  | ExportAllDeclaration
  | ExportNamedDeclaration
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
  let specifiers: undefined | Array<ExportLocalSpecifier>;

  // export * from '...'';
  if (shouldParseExportStar(parser)) {
    return parseExportStar(parser, start);
  } else if (isExportDefaultSpecifier(parser)) {
    const defStart = parser.getPosition();
    const defExported = parseIdentifier(parser, true);
    let specifiers: Array<AnyExportExternalSpecifier> = [];

    specifiers.push({
      loc: parser.finishLoc(defStart),
      type: 'ExportDefaultSpecifier',
      exported: defExported,
    });

    if (
      parser.match(tt.comma) &&
      parser.lookaheadState().tokenType === tt.star
    ) {
      parser.expect(tt.comma);
      const specifierStart = parser.getPosition();
      parser.expect(tt.star);
      parser.expectContextual('as');
      const exported = parseIdentifier(parser);
      specifiers.push({
        loc: parser.finishLoc(specifierStart),
        type: 'ExportNamespaceSpecifier',
        exported,
      });
    } else {
      specifiers = [
        ...specifiers,
        ...convertLocalToExternalSpecifiers(
          parseExportLocalSpecifiersMaybe(parser),
        ),
      ];
    }

    const source = parseExportFromExpect(parser);
    return createExportExternalDeclaration(parser, start, specifiers, source);
  } else if (parser.eat(tt._default)) {
    // export default ...
    const declaration = parseExportDefaultExpression(parser);
    checkExport(parser, specifiers, declaration, true, true);

    const node: ExportDefaultDeclaration = {
      loc: parser.finishLoc(start),
      type: 'ExportDefaultDeclaration',
      declaration,
    };
    return node;
  } else if (shouldParseExportDeclaration(parser)) {
    let source;
    ({declaration, source, specifiers, exportKind} = parseExportDeclaration(
      parser,
    ));

    if (source !== undefined) {
      if (declaration !== undefined) {
        throw new Error(
          "When there's a source we don't also expect a declaration",
        );
      }

      return createExportExternalDeclaration(
        parser,
        start,
        specifiers,
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
      message:
        'Started with `export async` so we expected to receive an async function but no function keyword was found',
    });
    declaration = undefined;
    specifiers = [];
  } else {
    // export { x, y as z } [from '...']';
    specifiers = parseExportSpecifiers(parser);

    const source = parseExportFrom(parser, false);
    if (source !== undefined) {
      return createExportExternalDeclaration(parser, start, specifiers, source);
    }
  }

  checkExport(parser, specifiers, declaration, true, false);

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
        message: 'Invalid export declaration',
      });
      return declaration;
    }
  }

  const node: ExportNamedDeclaration = {
    loc: parser.finishLoc(start),
    type: 'ExportNamedDeclaration',
    exportKind,
    specifiers,
    declaration,
  };
  return node;
}

function createExportExternalDeclaration(
  parser: JSParser,
  start: Position,
  rawSpecifiers:
    | undefined
    | Array<ExportLocalSpecifier | AnyExportExternalSpecifier>,
  source: StringLiteral,
  exportKind?: ConstExportModuleKind,
): ExportExternalDeclaration {
  const specifiers =
    rawSpecifiers === undefined
      ? undefined
      : convertLocalToExternalSpecifiers(rawSpecifiers);
  checkExport(parser, specifiers, undefined, true, false);

  return {
    type: 'ExportExternalDeclaration',
    exportKind,
    source,
    specifiers,
    loc: parser.finishLoc(start),
  };
}

function convertLocalToExternalSpecifiers(
  specifiers: Array<AnyExportExternalSpecifier | ExportLocalSpecifier>,
): Array<AnyExportExternalSpecifier> {
  return specifiers.map(specifier => {
    if (specifier.type === 'ExportLocalSpecifier') {
      return {
        ...specifier,
        type: 'ExportExternalSpecifier',
        local: toIdentifier(specifier.local),
      };
    } else {
      return specifier;
    }
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
      message:
        'Only expressions, functions or classes are allowed as the `default` export.',
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
  specifiers?: Array<ExportLocalSpecifier>;
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
        specifiers,
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
      parser.state.tokenValue == 'opaque')
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
        message: 'Import from only allows strings',
      });

      source = {
        type: 'StringLiteral',
        value: '',
        loc: expr.loc,
      };
    }
  } else if (expect) {
    parser.addDiagnostic({
      message: 'Expected `from` for an export node',
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
): ExportAllDeclaration | ExportNamedDeclaration | ExportExternalDeclaration {
  let exportKind: undefined | ConstExportModuleKind;
  if (parser.eatContextual('type')) {
    exportKind = 'type';
  }

  parser.expect(tt.star);

  if (parser.isContextual('as')) {
    const {source, specifiers} = parseExportNamespace(parser, exportKind);
    return {
      loc: parser.finishLoc(start),
      type: 'ExportExternalDeclaration',
      exportKind,
      specifiers,
      source,
    };
  } else {
    const source = parseExportFrom(parser, true);
    if (source === undefined) {
      throw new Error('Passed `true` above which expects there to be a string');
    }
    return {
      loc: parser.finishLoc(start),
      type: 'ExportAllDeclaration',
      exportKind,
      source,
    };
  }
}

function parseExportNamespace(
  parser: JSParser,
  exportKind: undefined | ConstExportModuleKind,
): {
  source: StringLiteral;
  specifiers: Array<AnyExportExternalSpecifier>;
} {
  if (exportKind === 'type') {
    parser.addDiagnostic({
      message: "Can't have a type export namespacer specifier",
    });
  }

  const specifierStart = parser.state.lastStartPos;
  parser.next();
  const exported = parseIdentifier(parser, true);

  let specifiers: Array<AnyExportExternalSpecifier> = [];

  specifiers.push({
    loc: parser.finishLoc(specifierStart),
    type: 'ExportNamespaceSpecifier',
    exported,
  });

  specifiers = [
    ...specifiers,
    ...convertLocalToExternalSpecifiers(
      parseExportLocalSpecifiersMaybe(parser),
    ),
  ];

  const source = parseExportFromExpect(parser);
  return {source, specifiers};
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
  specifiers:
    | undefined
    | Array<ExportLocalSpecifier | AnyExportExternalSpecifier>,
  declaration: undefined | AnyNode,
  checkNames: boolean = false,
  isDefault: boolean = false,
): void {
  if (checkNames === false) {
    return undefined;
  }

  // Check for duplicate exports
  if (isDefault) {
    // Default exports
    if (declaration !== undefined) {
      checkDuplicateExports(parser, declaration, 'default');
    }
    return undefined;
  }

  if (specifiers !== undefined && specifiers.length > 0) {
    // Named exports
    for (const specifier of specifiers) {
      checkDuplicateExports(parser, specifier, specifier.exported.name);

      if (specifier.type === 'ExportLocalSpecifier') {
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
      message:
        name === 'default'
          ? 'Only one default export allowed per module.'
          : `\`${name}\` has already been exported. Exported identifiers must be unique.`,
      advice: [
        {
          type: 'log',
          category: 'info',
          message: 'First defined here',
        },
        {
          type: 'frame',
          filename: existing.filename,
          start: existing.start,
          end: existing.end,
        },
      ],
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
      : toIdentifier(parser.cloneNode(local));
    specifiers.push({
      loc: parser.finishLoc(start),
      type: 'ExportLocalSpecifier',
      local,
      exported,
      // TODO exportKind?
    });
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

  let specifiers: undefined | Array<AnyImportSpecifier>;
  let source: StringLiteral;
  let importKind: undefined | ConstImportModuleKind;

  // import '...'
  if (parser.match(tt.string)) {
    specifiers = [];
    source = parseStringLiteral(parser);
  } else {
    ({specifiers, importKind} = parseImportSpecifiers(parser, start));

    if (parser.expectContextual('from') && parser.match(tt.string)) {
      source = parseStringLiteral(parser);
    } else {
      parser.addDiagnostic({
        message: 'import missing a source',
      });

      source = {
        type: 'StringLiteral',
        value: '',
        loc: parser.finishLoc(start),
      };
    }
  }

  parser.semicolon();
  return {
    loc: parser.finishLoc(start),
    type: 'ImportDeclaration',
    specifiers,
    source,
    importKind,
  };
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

  return {
    type: 'ImportSpecifierLocal',
    loc: parser.finishLoc(start),
    name: local,
    importKind,
  };
}

// Parses a comma-separated list of module imports.
function parseImportSpecifiers(
  parser: JSParser,
  start: Position,
): {
  specifiers: Array<
    ImportSpecifier | ImportDefaultSpecifier | ImportNamespaceSpecifier
  >;
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
        message: 'import * is not allowed',
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

  const specifiers: Array<
    ImportSpecifier | ImportDefaultSpecifier | ImportNamespaceSpecifier
  > = [];

  let first = true;

  // import defaultObj, { x, y as z } from '...'';
  if (shouldParseDefaultImport(parser, importKind)) {
    const start = parser.getPosition();

    const meta = parseImportSpecifierLocal(
      parser,
      importKind,
      'default import specifier',
    );

    specifiers.push({
      loc: parser.finishLoc(start),
      type: 'ImportDefaultSpecifier',
      local: meta,
    });

    if (!parser.eat(tt.comma)) {
      return {specifiers, importKind};
    }
  }

  if (parser.match(tt.star)) {
    const start = parser.getPosition();
    parser.next();
    parser.expectContextual('as');

    const meta = parseImportSpecifierLocal(
      parser,
      importKind,
      'import namespace specifier',
    );

    specifiers.push({
      loc: parser.finishLoc(start),
      type: 'ImportNamespaceSpecifier',
      local: meta,
    });

    return {specifiers, importKind};
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
          message:
            'ES2015 named imports do not destructure. Use another statement for destructuring after the import.',
        });
      }

      parser.expect(tt.comma);

      if (parser.eat(tt.braceR)) {
        break;
      }
    }

    specifiers.push(parseImportSpecifier(parser, importKind));
  }

  return {specifiers, importKind};
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
    const as_ident = parseIdentifier(parser, true);
    if (
      importKind !== undefined &&
      !parser.match(tt.name) &&
      parser.state.tokenType.keyword === undefined
    ) {
      // `import {type as ,` or `import {type as }`
      imported = as_ident;
      local = toBindingIdentifier(parser.cloneNode(as_ident));
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
      local = toBindingIdentifier(parser.cloneNode(imported));
    }
  } else {
    isBinding = true;
    imported = firstIdent;
    importKind = undefined;
    local = toBindingIdentifier(parser.cloneNode(imported));
  }

  const nodeIsTypeImport = hasTypeImportKind(nodeKind);
  const specifierIsTypeImport = hasTypeImportKind(importKind);

  if (nodeIsTypeImport && specifierIsTypeImport) {
    parser.addDiagnostic({
      start: firstIdentPos,
      message:
        'The `type` and `typeof` keywords on named imports can only be used on regular `import` statements. It cannot be used with `import type` or `import typeof` statements',
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

  return {
    type: 'ImportSpecifier',
    loc,
    imported,
    local: {
      type: 'ImportSpecifierLocal',
      loc,
      name: local,
      importKind,
    },
  };
}
