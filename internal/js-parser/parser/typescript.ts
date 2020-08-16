/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position} from "@internal/parser-core";
import {
	JSParser,
	cloneState,
	createUnknownIdentifier,
	eat,
	eatContextual,
	expect,
	expectClosing,
	expectContextual,
	expectOpening,
	expectRelational,
	hasPrecedingLineBreak,
	isContextual,
	isLineTerminator,
	isLookaheadContextual,
	isRelational,
	isSyntaxEnabled,
	lookaheadState,
	match,
	next,
	popScope,
	pushScope,
	semicolon,
	unexpectedDiagnostic,
} from "../parser";
import {TokenType, types as tt} from "../tokenizer/types";
import {
	assertVarKind,
	hasCommaAfterRest,
	parseBindingIdentifier,
	parseBindingListNonEmpty,
	parseBlockOrModuleBlockBody,
	parseClassDeclaration,
	parseExportDefaultClassDeclaration,
	parseExpression,
	parseExpressionAtom,
	parseFunctionDeclaration,
	parseIdentifier,
	parseIdentifierName,
	parseMaybeAssign,
	parseMaybeUnary,
	parseObjectPropertyKey,
	parseReferenceIdentifier,
	parseStringLiteral,
	parseTSConstKeyword,
	parseTemplate,
	parseVarStatement,
	toBindingIdentifier,
	toReferenceIdentifier,
} from "./index";
import {
	AnyJSExpression,
	AnyJSTargetAssignmentPattern,
	AnyJSTargetBindingPattern,
	AnyNode,
	AnyTSEntityName,
	AnyTSKeywordTypeAnnotation,
	AnyTSLiteralTypeAnnotation,
	AnyTSModuleReference,
	AnyTSPrimary,
	AnyTSTypeElement,
	ConstTSAccessibility,
	ConstTSModifier,
	JSAmbiguousFlowTypeCastExpression,
	JSBindingIdentifier,
	JSClassDeclaration,
	JSFunctionDeclaration,
	JSIdentifier,
	JSPatternMeta,
	JSStringLiteral,
	JSVariableDeclarationKind,
	JSVariableDeclarationStatement,
	TSCallSignatureDeclaration,
	TSConstKeyword,
	TSConstructSignatureDeclaration,
	TSConstructorType,
	TSDeclareFunction,
	TSEnumDeclaration,
	TSEnumMember,
	TSExportAssignment,
	TSExpressionWithTypeArguments,
	TSExternalModuleReference,
	TSFunctionType,
	TSImportEqualsDeclaration,
	TSImportType,
	TSIndexSignature,
	TSInferType,
	TSInterfaceBody,
	TSInterfaceDeclaration,
	TSMappedType,
	TSMappedTypeBoolean,
	TSMethodSignature,
	TSModuleBlock,
	TSModuleDeclaration,
	TSNamespaceExportDeclaration,
	TSObjectTypeAnnotation,
	TSParenthesizedType,
	TSPropertySignature,
	TSSignatureDeclarationMeta,
	TSTemplateLiteralTypeAnnotation,
	TSThisType,
	TSTupleElement,
	TSTupleType,
	TSTypeAlias,
	TSTypeAssertion,
	TSTypeOperator,
	TSTypeParameter,
	TSTypeParameterDeclaration,
	TSTypeParameterInstantiation,
	TSTypePredicate,
	TSTypeQuery,
	TSTypeReference,
} from "@internal/ast";
import {descriptions} from "@internal/diagnostics";
import {NumberTokenValue} from "../tokenizer";
import {toTargetAssignmentPattern} from "./lval";
import {markup} from "@internal/markup";
import {VoidCallback} from "@internal/typescript-helpers";

type ParsingContext =
	| "EnumMembers"
	| "HeritageClauseElement"
	| "TupleElementTypes"
	| "TypeMembers"
	| "TypeParametersOrArguments";

// Doesn't handle 'void' or 'null' because those are keywords, not identifiers.
function keywordTypeFromName(
	value: string,
): AnyTSKeywordTypeAnnotation["type"] | undefined {
	switch (value) {
		case "any":
			return "TSAnyKeywordTypeAnnotation";

		case "boolean":
			return "TSBooleanKeywordTypeAnnotation";

		case "bigint":
			return "TSBigIntKeywordTypeAnnotation";

		case "never":
			return "TSNeverKeywordTypeAnnotation";

		case "number":
			return "TSNumberKeywordTypeAnnotation";

		case "object":
			return "TSObjectKeywordTypeAnnotation";

		case "string":
			return "TSStringKeywordTypeAnnotation";

		case "symbol":
			return "TSSymbolKeywordTypeAnnotation";

		case "undefined":
			return "TSUndefinedKeywordTypeAnnotation";

		case "unknown":
			return "TSUnknownKeywordTypeAnnotation";

		default:
			return undefined;
	}
}

function tsIsIdentifier(parser: JSParser): boolean {
	// TODO: actually a bit more complex in TypeScript, but shouldn't matter.
	// See https://github.com/Microsoft/TypeScript/issues/15008
	return match(parser, tt.name);
}

function tsNextTokenCanFollowModifier(parser: JSParser) {
	// Note: TypeScript's implementation is much more complicated because
	// more things are considered modifiers there.
	// This implementation only handles modifiers not handled by @babel/parser itself. And 'static'.
	// TODO: Would be nice to avoid lookahead. Want a hasLineBreakUpNext() method...
	next(parser);
	return (
		!hasPrecedingLineBreak(parser) &&
		!match(parser, tt.parenL) &&
		!match(parser, tt.parenR) &&
		!match(parser, tt.colon) &&
		!match(parser, tt.eq) &&
		!match(parser, tt.question) &&
		!match(parser, tt.bang)
	);
}

/** Parses a modifier matching one the given modifier names.*/
export function parseTSModifier<T extends ConstTSModifier>(
	parser: JSParser,
	allowedModifiers: Array<T>,
): undefined | T {
	if (!match(parser, tt.name)) {
		return undefined;
	}

	const start = parser.getPosition();

	// @ts-ignore: We are lying here but we validate it in all the correct places
	const modifier: T = String(parser.state.tokenValue);
	if (
		allowedModifiers.includes(modifier) &&
		tryTSParse(parser, tsNextTokenCanFollowModifier)
	) {
		expectTSEnabled(parser, "access modifier", start);
		return modifier;
	} else {
		return undefined;
	}
}

export function hasTSModifier(
	parser: JSParser,
	allowedModifiers: Array<ConstTSModifier>,
): boolean {
	return parseTSModifier(parser, allowedModifiers) !== undefined;
}

function tsIsListTerminator(parser: JSParser, kind: ParsingContext): boolean {
	if (match(parser, tt.eof)) {
		return true;
	}

	switch (kind) {
		case "EnumMembers":
		case "TypeMembers":
			return match(parser, tt.braceR);

		case "HeritageClauseElement":
			return match(parser, tt.braceL);

		case "TupleElementTypes":
			return match(parser, tt.bracketR);

		case "TypeParametersOrArguments":
			return isRelational(parser, ">");
	}

	throw new Error("Unreachable");
}

function expectTSEnabled(
	parser: JSParser,
	label: string,
	start: Position = parser.getPosition(),
) {
	if (isSyntaxEnabled(parser, "ts")) {
		return;
	}

	unexpectedDiagnostic(
		parser,
		{
			start,
			description: descriptions.JS_PARSER.TS_REQUIRED(label),
		},
	);
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
		if (element === undefined) {
			break;
		}

		result.push(element);

		if (eat(parser, tt.comma)) {
			continue;
		}

		if (tsIsListTerminator(parser, kind)) {
			break;
		}

		// This will fail with an error about a missing comma
		if (expect(parser, tt.comma)) {
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
			expect(parser, tt.bracketL);
		} else {
			expectRelational(parser, "<");
		}
	}

	const result = parseTSDelimitedList(parser, kind, parseElement);

	if (bracket) {
		expect(parser, tt.bracketR);
	} else {
		expectRelational(parser, ">");
	}

	return result;
}

function parseTSImportType(parser: JSParser): TSImportType {
	const start = parser.getPosition();
	expect(parser, tt._import);
	const openContext = expectOpening(
		parser,
		tt.parenL,
		tt.parenR,
		"ts import type",
	);

	if (!match(parser, tt.string)) {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.TS_IMPORT_ARG_NOT_STRING,
			},
		);
	}

	const argument = parseExpressionAtom(parser, "ts import argument");
	expectClosing(parser, openContext);

	let qualifier;
	if (eat(parser, tt.dot)) {
		qualifier = parseTSEntityName(parser, /* allowReservedWords */ true);
	}

	let typeParameters;
	if (isRelational(parser, "<")) {
		typeParameters = parseTSTypeArguments(parser);
	}

	return parser.finishNode(
		start,
		{
			type: "TSImportType",
			argument,
			qualifier,
			typeParameters,
		},
	);
}

function parseTSEntityName(
	parser: JSParser,
	allowReservedWords: boolean,
): AnyTSEntityName {
	let entity: AnyTSEntityName = parseReferenceIdentifier(parser);
	while (eat(parser, tt.dot)) {
		const start: Position = parser.getLoc(entity).start;
		const right = parseIdentifier(parser, allowReservedWords);
		entity = parser.finishNode(
			start,
			{
				type: "TSQualifiedName",
				left: entity,
				right,
			},
		);
	}
	return entity;
}

function parseTSTypeReference(parser: JSParser): TSTypeReference {
	const start = parser.getPosition();
	const typeName = parseTSEntityName(parser, /* allowReservedWords */ false);
	let typeParameters;
	if (!hasPrecedingLineBreak(parser) && isRelational(parser, "<")) {
		typeParameters = parseTSTypeArguments(parser);
	}
	return parser.finishNode(
		start,
		{
			type: "TSTypeReference",
			typeName,
			typeParameters,
		},
	);
}

function parseTSThisTypePredicate(
	parser: JSParser,
	lhs: TSThisType,
): TSTypePredicate {
	next(parser);
	const start = parser.getLoc(lhs).start;
	const parameterName = lhs;
	const typeAnnotation = parseTSTypeAnnotation(parser, /* eatColon */ false);

	return parser.finishNode(
		start,
		{
			type: "TSTypePredicate",
			asserts: false,
			parameterName,
			typeAnnotation,
		},
	);
}

function parseTSThisTypeNode(parser: JSParser): TSThisType {
	const start = parser.getPosition();
	next(parser);
	return parser.finishNode(
		start,
		{
			type: "TSThisType",
		},
	);
}

function parseTSTypeQuery(parser: JSParser): TSTypeQuery {
	const start = parser.getPosition();
	expect(parser, tt._typeof);
	let exprName;
	if (match(parser, tt._import)) {
		exprName = parseTSImportType(parser);
	} else {
		exprName = parseTSEntityName(parser, /* allowReservedWords */ true);
	}
	return parser.finishNode(
		start,
		{
			type: "TSTypeQuery",
			exprName,
		},
	);
}

export function ambiguousTypeCastToParameter(
	parser: JSParser,
	node: JSAmbiguousFlowTypeCastExpression,
): AnyJSTargetAssignmentPattern {
	const start = parser.getPosition();
	const expr = toTargetAssignmentPattern(parser, node.expression, "parameter");

	const meta: JSPatternMeta = parser.finishNode(
		start,
		{
			type: "JSPatternMeta",
			optional: node.optional,
			typeAnnotation: node.typeAnnotation,
		},
	);

	return parser.finishNode(
		start,
		{
			...expr,
			// @ts-ignore
			meta,
		},
	);
}

export function maybeParseTSTypeParameters(
	parser: JSParser,
): undefined | TSTypeParameterDeclaration {
	if (isRelational(parser, "<")) {
		return parseTSTypeParameters(parser);
	} else {
		return undefined;
	}
}

export function maybeParseTSTypeArguments(
	parser: JSParser,
): undefined | TSTypeParameterInstantiation {
	if (isRelational(parser, "<")) {
		return parseTSTypeArguments(parser);
	} else {
		return undefined;
	}
}

function parseTSTypeParameter(parser: JSParser): TSTypeParameter {
	const start = parser.getPosition();
	expectTSEnabled(parser, "type parameters", start);

	const name = parseIdentifierName(parser);
	const constraint = tsEatThenParseType(parser, tt._extends);
	const _default = tsEatThenParseType(parser, tt.eq);
	return parser.finishNode(
		start,
		{
			type: "TSTypeParameter",
			name,
			constraint,
			default: _default,
		},
	);
}

function tryParseTSTypeParameters(
	parser: JSParser,
): undefined | TSTypeParameterDeclaration {
	if (isRelational(parser, "<")) {
		return parseTSTypeParameters(parser);
	} else {
		return undefined;
	}
}

export function parseTSTypeParameters(
	parser: JSParser,
): TSTypeParameterDeclaration {
	const start = parser.getPosition();

	expectRelational(parser, "<");

	const params = parseTSBracketedList(
		parser,
		"TypeParametersOrArguments",
		parseTSTypeParameter,
		/* bracket */ false,
		/* skipFirstToken */ true,
	);

	return parser.finishNode(
		start,
		{
			type: "TSTypeParameterDeclaration",
			params,
		},
	);
}

export function tryTSNextParseConstantContext(
	parser: JSParser,
): undefined | TSConstKeyword {
	if (lookaheadState(parser).tokenType === tt._const) {
		next(parser);
		return parseTSConstKeyword(parser);
	} else {
		return undefined;
	}
}

export function tsCheckLiteralForConstantContext(
	parser: JSParser,
	node: AnyNode,
) {
	switch (node.type) {
		case "JSStringLiteral":
		case "JSTemplateLiteral":
		case "JSNumericLiteral":
		case "JSBooleanLiteral":
		case "JSSpreadElement":
		case "JSObjectMethod":
		case "JSObjectExpression":
			break;

		case "JSArrayExpression": {
			for (const elem of node.elements) {
				if (elem) {
					tsCheckLiteralForConstantContext(parser, elem);
				}
			}
			break;
		}

		case "JSObjectProperty": {
			tsCheckLiteralForConstantContext(parser, node.value);
			break;
		}

		case "JSUnaryExpression": {
			tsCheckLiteralForConstantContext(parser, node.argument);
			break;
		}

		default:
			unexpectedDiagnostic(
				parser,
				{
					loc: node.loc,
					description: descriptions.JS_PARSER.TS_CONSTANT_NOT_LITERAL,
				},
			);
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
	} else if (match(parser, returnToken)) {
		typeAnnotation = parseTSTypeOrTypePredicateAnnotation(parser, returnToken);
	}

	return {
		typeAnnotation,
		meta: parser.finishNode(
			start,
			{
				type: "TSSignatureDeclarationMeta",
				typeParameters,
				parameters,
				rest,
			},
		),
	};
}

function parseTSBindingListForSignature(
	parser: JSParser,
): {
	list: Array<AnyJSTargetBindingPattern>;
	rest: undefined | AnyJSTargetBindingPattern;
} {
	const openContext = expectOpening(
		parser,
		tt.parenL,
		tt.parenR,
		"ts signature parameters",
	);
	const {list: patterns, rest} = parseBindingListNonEmpty(parser, openContext);
	const validPatterns: Array<AnyJSTargetBindingPattern> = [];

	for (const pattern of patterns) {
		if (
			pattern.type === "JSBindingIdentifier" ||
			pattern.type === "JSBindingObjectPattern" ||
			pattern.type === "JSBindingArrayPattern"
		) {
			validPatterns.push(pattern);
		} else {
			unexpectedDiagnostic(
				parser,
				{
					loc: pattern.loc,
					description: descriptions.JS_PARSER.TS_INVALID_SIGNATURE_BINDING_NODE,
				},
			);
		}
	}

	return {list: validPatterns, rest};
}

function parseTSTypeMemberSemicolon(parser: JSParser): void {
	if (!eat(parser, tt.comma)) {
		semicolon(parser);
	}
}

function parseTSConstructSignatureDeclaration(
	parser: JSParser,
): TSConstructSignatureDeclaration {
	const start = parser.getPosition();
	expect(parser, tt._new);
	const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
		parser,
		tt.colon,
	);
	semicolon(parser);
	return parser.finishNode(
		start,
		{
			type: "TSConstructSignatureDeclaration",
			meta,
			typeAnnotation,
		},
	);
}

function parseTSCallSignatureDeclaration(
	parser: JSParser,
): TSCallSignatureDeclaration {
	const start = parser.getPosition();
	const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
		parser,
		tt.colon,
	);
	semicolon(parser);
	return parser.finishNode(
		start,
		{
			type: "TSCallSignatureDeclaration",
			meta,
			typeAnnotation,
		},
	);
}

function tsIsUnambiguouslyIndexSignature(parser: JSParser) {
	next(parser); // Skip '{'
	return eat(parser, tt.name) && match(parser, tt.colon);
}

export function tryTSParseIndexSignature(
	parser: JSParser,
	start: Position,
): undefined | TSIndexSignature {
	if (
		!(match(parser, tt.bracketL) &&
		lookaheadTS(parser, tsIsUnambiguouslyIndexSignature))
	) {
		return undefined;
	}

	expect(parser, tt.bracketL);

	const idStart = parser.getPosition();
	const id = parseBindingIdentifier(parser);

	const keyTypeAnnotation = parseTSTypeAnnotation(parser);
	const key = parser.finishNode(
		idStart,
		{
			...id,
			meta: parser.finishNode(
				idStart,
				{
					...id.meta,
					type: "JSPatternMeta",
					typeAnnotation: keyTypeAnnotation,
				},
			),
		},
	);

	expect(parser, tt.bracketR);

	const typeAnnotation = tryTSParseTypeAnnotation(parser);

	semicolon(parser);
	return parser.finishNode(
		start,
		{
			type: "TSIndexSignature",
			typeAnnotation,
			key,
		},
	);
}

function parseTSPropertyOrMethodSignature(
	parser: JSParser,
	start: Position,
	readonly: boolean,
): TSPropertySignature | TSMethodSignature {
	const key = parseObjectPropertyKey(parser);
	const optional = eat(parser, tt.question);

	if (!readonly && (match(parser, tt.parenL) || isRelational(parser, "<"))) {
		const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
			parser,
			tt.colon,
		);
		parseTSTypeMemberSemicolon(parser);
		return parser.finishNode(
			start,
			{
				type: "TSMethodSignature",
				optional,
				meta,
				key,
				returnType: typeAnnotation,
			},
		);
	} else {
		const typeAnnotation = tryTSParseTypeAnnotation(parser);
		parseTSTypeMemberSemicolon(parser);
		return parser.finishNode(
			start,
			{
				type: "TSPropertySignature",
				optional,
				readonly,
				typeAnnotation,
				key,
			},
		);
	}
}

function parseTSTypeMember(parser: JSParser): AnyTSTypeElement {
	if (match(parser, tt.parenL) || isRelational(parser, "<")) {
		return parseTSCallSignatureDeclaration(parser);
	}

	if (
		match(parser, tt._new) &&
		lookaheadTS(parser, tsIsStartOfConstructSignature)
	) {
		return parseTSConstructSignatureDeclaration(parser);
	}

	const start = parser.getPosition();
	const readonly = hasTSModifier(parser, ["readonly"]);

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
	next(parser);
	return match(parser, tt.parenL) || isRelational(parser, "<");
}

function parseTSObjectTypeAnnotation(parser: JSParser): TSObjectTypeAnnotation {
	const start = parser.getPosition();
	const members = parseTSObjectTypeMembers(parser);
	return parser.finishNode(
		start,
		{
			type: "TSObjectTypeAnnotation",
			members,
		},
	);
}

function parseTSObjectTypeMembers(parser: JSParser): Array<AnyTSTypeElement> {
	const openContext = expectOpening(
		parser,
		tt.braceL,
		tt.braceR,
		"ts object type members",
	);
	const members = parseTSList(parser, "TypeMembers", parseTSTypeMember);
	expectClosing(parser, openContext);
	return members;
}

function tsIsStartOfMappedType(parser: JSParser): boolean {
	next(parser);

	if (eat(parser, tt.plusMin)) {
		return isContextual(parser, "readonly");
	}

	if (isContextual(parser, "readonly")) {
		next(parser);
	}

	if (!match(parser, tt.bracketL)) {
		return false;
	}

	next(parser);

	if (!tsIsIdentifier(parser)) {
		return false;
	}

	next(parser);

	return match(parser, tt._in);
}

function parseTSMappedTypeParameter(parser: JSParser): TSTypeParameter {
	const start = parser.getPosition();
	const name = parseIdentifierName(parser);
	const constraint = tsExpectThenParseType(parser, tt._in);
	return parser.finishNode(
		start,
		{
			type: "TSTypeParameter",
			name,
			constraint,
		},
	);
}

function toPlusMin(val: unknown): "+" | "-" {
	const str = String(val);
	if (str === "+" || str === "-") {
		return str;
	} else {
		throw new Error("Expected +/-");
	}
}

function parseTSMappedType(parser: JSParser): TSMappedType {
	const start = parser.getPosition();

	const openContext = expectOpening(
		parser,
		tt.braceL,
		tt.braceR,
		"ts mapped type",
	);

	let readonly: TSMappedTypeBoolean;
	if (match(parser, tt.plusMin)) {
		readonly = toPlusMin(parser.state.tokenValue);
		next(parser);
		expectContextual(parser, "readonly");
	} else if (eatContextual(parser, "readonly")) {
		readonly = true;
	}

	const paramOpenContext = expectOpening(
		parser,
		tt.bracketL,
		tt.bracketR,
		"ts mapped type parameter",
	);
	const typeParameter = parseTSMappedTypeParameter(parser);
	expectClosing(parser, paramOpenContext);

	let optional: TSMappedTypeBoolean;
	if (match(parser, tt.plusMin)) {
		optional = toPlusMin(parser.state.tokenValue);
		next(parser);
		expect(parser, tt.question);
	} else if (eat(parser, tt.question)) {
		optional = true;
	}

	const typeAnnotation = tryTSParseType(parser);
	semicolon(parser);
	expectClosing(parser, openContext);

	return parser.finishNode(
		start,
		{
			type: "TSMappedType",
			typeParameter,
			typeAnnotation,
			optional,
			readonly,
		},
	);
}

function parseTSTupleType(parser: JSParser): TSTupleType {
	const start = parser.getPosition();
	const elementDefs = parseTSBracketedList(
		parser,
		"TupleElementTypes",
		parseTSTupleElementType,
		/* bracket */ true,
		/* skipFirstToken */ false,
	);

	// Validate the elementTypes to ensure:
	//   No mandatory elements may follow optional elements

	//   If there's a rest element, it must be at the end of the tuple
	let seenOptionalElement = false;
	const elementTypes: Array<TSTupleElement> = [];
	let rest: undefined | TSTupleElement;
	for (const {type, isRest} of elementDefs) {
		if (rest !== undefined) {
			// No elements should come after a rest, we should have already produced an error
			continue;
		}

		if (type.optional) {
			seenOptionalElement = true;
		} else if (seenOptionalElement && !isRest) {
			unexpectedDiagnostic(
				parser,
				{
					loc: type.loc,
					description: descriptions.JS_PARSER.TS_REQUIRED_FOLLOWS_OPTIONAL,
				},
			);
		}

		if (isRest) {
			rest = type;
		} else {
			elementTypes.push(type);
		}
	}

	return parser.finishNode(
		start,
		{
			type: "TSTupleType",
			elementTypes,
			rest,
		},
	);
}

function parseTSTupleElementTypeInner(
	parser: JSParser,
): {
	name: undefined | JSBindingIdentifier;
	typeAnnotation: AnyTSPrimary;
	optional: boolean;
} {
	let typeAnnotation = parseTSType(parser);
	let optional = eat(parser, tt.question);
	let name: undefined | JSBindingIdentifier;

	if (eat(parser, tt.colon)) {
		if (
			typeAnnotation.type === "TSTypeReference" &&
			typeAnnotation.typeName.type === "JSReferenceIdentifier"
		) {
			name = toBindingIdentifier(parser, typeAnnotation.typeName);
			typeAnnotation = parseTSType(parser);
		} else {
			unexpectedDiagnostic(
				parser,
				{
					loc: typeAnnotation.loc,
					description: descriptions.JS_PARSER.TS_TUPLE_ELEMENT_LABEL_INCORRECT,
				},
			);
		}

		if (match(parser, tt.question)) {
			unexpectedDiagnostic(
				parser,
				{
					description: descriptions.JS_PARSER.TS_TUPLE_ELEMENT_OPTIONAL_TRAILING,
				},
			);
			next(parser);
			optional = true;
		}
	}

	return {
		typeAnnotation,
		optional,
		name,
	};
}

function parseTSTupleElementType(
	parser: JSParser,
): {
	type: TSTupleElement;
	isRest: boolean;
} {
	const start = parser.getPosition();
	let isRest = false;
	let typeAnnotation;
	let optional;
	let name;

	// parses `...TsType[]`
	if (eat(parser, tt.ellipsis)) {
		isRest = true;
		({typeAnnotation, optional, name} = parseTSTupleElementTypeInner(parser));
		hasCommaAfterRest(parser);
	} else {
		({typeAnnotation, optional, name} = parseTSTupleElementTypeInner(parser));
	}

	const elem = parser.finishNode(
		start,
		{
			type: "TSTupleElement",
			name,
			optional,
			typeAnnotation,
		},
	);

	if (optional && isRest) {
		unexpectedDiagnostic(
			parser,
			{
				loc: elem.loc,
				description: descriptions.JS_PARSER.TS_TUPLE_ELEMENT_OPTIONAL_REST,
			},
		);
	}

	return {
		isRest,
		type: elem,
	};
}

function parseTSParenthesizedType(parser: JSParser): TSParenthesizedType {
	const start = parser.getPosition();
	const openContext = expectOpening(
		parser,
		tt.parenL,
		tt.parenR,
		"ts parenthesized type",
	);
	const typeAnnotation = parseTSType(parser);
	expectClosing(parser, openContext);
	return parser.finishNode(
		start,
		{
			type: "TSParenthesizedType",
			typeAnnotation,
		},
	);
}

function parseTSFunctionType(parser: JSParser): TSFunctionType {
	const start = parser.getPosition();
	const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
		parser,
		tt.arrow,
	);

	if (typeAnnotation === undefined) {
		throw new Error(
			"Type annotation return type required as we passed tt.arrow above",
		);
	}

	return parser.finishNode(
		start,
		{
			type: "TSFunctionType",
			meta,
			typeAnnotation,
		},
	);
}

function parseTSConstructorType(parser: JSParser): TSConstructorType {
	const start = parser.getPosition();
	expect(parser, tt._new);

	const {meta, typeAnnotation} = parseTSSignatureDeclarationMeta(
		parser,
		tt.arrow,
	);

	if (typeAnnotation === undefined) {
		throw new Error(
			"Type annotation return type required as we passed tt.arrow above",
		);
	}

	return parser.finishNode(
		start,
		{
			type: "TSConstructorType",
			meta,
			typeAnnotation,
		},
	);
}

function parseTSTemplateLiteralType(
	parser: JSParser,
): TSTemplateLiteralTypeAnnotation {
	const templateNode = parseTemplate(parser, false);

	if (templateNode.expressions.length > 0) {
		unexpectedDiagnostic(
			parser,
			{
				loc: parser.getLoc(templateNode.expressions[0]),
				description: descriptions.JS_PARSER.TS_TEMPLATE_LITERAL_WITH_SUBSTITUION,
			},
		);
	}

	return {
		type: "TSTemplateLiteralTypeAnnotation",
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
				| AnyTSKeywordTypeAnnotation["type"]
				| "TSVoidKeywordTypeAnnotation"
				| "TSNullKeywordTypeAnnotation";
			if (match(parser, tt._void)) {
				type = "TSVoidKeywordTypeAnnotation";
			} else if (match(parser, tt._null)) {
				type = "TSNullKeywordTypeAnnotation";
			} else {
				type = keywordTypeFromName(String(parser.state.tokenValue));
			}

			if (type !== undefined && lookaheadState(parser).tokenType !== tt.dot) {
				const start = parser.getPosition();
				next(parser);
				return parser.finishNode(
					start,
					({
						type,
					} as AnyTSPrimary),
				);
			}
			return parseTSTypeReference(parser);
		}

		case tt.string:
		case tt.num:
		case tt._true:
		case tt._false:
		case tt.plusMin:
			return parseTSObjectTypeAnnotationAnnotation(parser);

		case tt._this: {
			const thisKeyword = parseTSThisTypeNode(parser);
			if (isContextual(parser, "is") && !hasPrecedingLineBreak(parser)) {
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
				return parseTSObjectTypeAnnotation(parser);
			}

		case tt.bracketL:
			return parseTSTupleType(parser);

		case tt.parenL:
			return parseTSParenthesizedType(parser);

		case tt.backQuote:
			return parseTSTemplateLiteralType(parser);
	}

	unexpectedDiagnostic(
		parser,
		{
			description: descriptions.JS_PARSER.TS_UNKNOWN_NON_ARRAY_START,
		},
	);
	next(parser);
	return parser.finishNode(
		parser.getPosition(),
		{
			type: "TSTypeReference",
			typeName: toReferenceIdentifier(
				parser,
				createUnknownIdentifier(parser, "ts non array type start"),
			),
		},
	);
}

function parseTSObjectTypeAnnotationAnnotation(
	parser: JSParser,
): AnyTSLiteralTypeAnnotation {
	const start = parser.getPosition();

	switch (parser.state.tokenType) {
		case tt.string: {
			const value = String(parser.state.tokenValue);
			next(parser);
			return parser.finishNode(
				start,
				{
					type: "TSStringLiteralTypeAnnotation",
					value,
				},
			);
		}

		case tt.num: {
			const {tokenValue} = parser.state;
			if (!(tokenValue instanceof NumberTokenValue)) {
				throw new Error("Expected NumberTokenValue");
			}

			const {value, format} = tokenValue;
			next(parser);
			return parser.finishNode(
				start,
				{
					type: "TSNumericLiteralTypeAnnotation",
					value,
					format,
				},
			);
		}

		case tt._true:
		case tt._false: {
			const value = match(parser, tt._true);
			next(parser);
			return parser.finishNode(
				start,
				{
					type: "TSBooleanLiteralTypeAnnotation",
					value,
				},
			);
		}

		case tt.plusMin: {
			const {tokenValue} = parser.state;
			if (tokenValue === "-") {
				next(parser);

				if (!match(parser, tt.num)) {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.TYPE_NUMERIC_LITERAL_EXPECTED,
						},
					);
					next(parser);
					return parser.finishNode(
						start,
						{
							type: "TSNumericLiteralTypeAnnotation",
							value: 0,
						},
					);
				}

				const {tokenValue} = parser.state;
				if (!(tokenValue instanceof NumberTokenValue)) {
					throw new Error("Expected NumberTokenValue");
				}

				const {value, format} = tokenValue;
				next(parser);
				return parser.finishNode(
					start,
					{
						type: "TSNumericLiteralTypeAnnotation",
						value: -value,
						format,
					},
				);
			} else {
				unexpectedDiagnostic(
					parser,
					{
						description: descriptions.JS_PARSER.TYPE_NUMERIC_LITERAL_PLUS,
					},
				);
				next(parser);

				if (!match(parser, tt.num)) {
					unexpectedDiagnostic(
						parser,
						{
							description: descriptions.JS_PARSER.TYPE_NUMERIC_LITERAL_EXPECTED,
						},
					);
					next(parser);
					return parser.finishNode(
						start,
						{
							type: "TSNumericLiteralTypeAnnotation",
							value: 0,
						},
					);
				}

				return parseTSObjectTypeAnnotationAnnotation(parser);
			}
		}

		default:
			throw new Error(
				"Caller should have already validated the range of token types",
			);
	}
}

function parseTSArrayTypeOrHigher(parser: JSParser): AnyTSPrimary {
	let type = parseTSNonArrayType(parser);

	while (!hasPrecedingLineBreak(parser) && eat(parser, tt.bracketL)) {
		if (match(parser, tt.bracketR)) {
			const start = parser.getLoc(type).start;
			const elementType = type;
			expect(parser, tt.bracketR);
			type = parser.finishNode(
				start,
				{
					type: "TSArrayType",
					elementType,
				},
			);
		} else {
			const start = parser.getLoc(type).start;
			const objectType = type;
			const indexType = parseTSType(parser);
			expect(parser, tt.bracketR);
			type = parser.finishNode(
				start,
				{
					type: "TSIndexedAccessType",
					objectType,
					indexType,
				},
			);
		}
	}
	return type;
}

function parseTSTypeOperator(
	parser: JSParser,
	operator: TSTypeOperator["operator"],
): TSTypeOperator {
	const start = parser.getPosition();
	expectContextual(parser, operator);

	const typeAnnotation = parseTSTypeOperatorOrHigher(parser);

	const node: TSTypeOperator = parser.finishNode(
		start,
		{
			type: "TSTypeOperator",
			typeAnnotation,
			operator,
		},
	);

	if (operator === "readonly") {
		tsCheckTypeAnnotationForReadOnly(parser, typeAnnotation);
	}

	return node;
}

function tsCheckTypeAnnotationForReadOnly(parser: JSParser, node: AnyTSPrimary) {
	switch (node.type) {
		case "TSTupleType":
		case "TSArrayType":
			return;

		default: {
			unexpectedDiagnostic(
				parser,
				{
					loc: node.loc,
					description: descriptions.JS_PARSER.TS_INVALID_READONLY_MODIFIER,
				},
			);
			break;
		}
	}
}

function parseTSInferType(parser: JSParser): TSInferType {
	const inferStart = parser.getPosition();
	expectContextual(parser, "infer");

	const start = parser.getPosition();
	const name = parseIdentifierName(parser);
	const typeParameter: TSTypeParameter = parser.finishNode(
		start,
		{
			type: "TSTypeParameter",
			name,
		},
	);

	return parser.finishNode(
		inferStart,
		{
			type: "TSInferType",
			typeParameter,
		},
	);
}

const TS_TYPE_OPERATORS: Array<TSTypeOperator["operator"]> = [
	"keyof",
	"unique",
	"readonly",
];

function parseTSTypeOperatorOrHigher(parser: JSParser): AnyTSPrimary {
	let operator: undefined | TSTypeOperator["operator"];

	for (const op of TS_TYPE_OPERATORS) {
		if (isContextual(parser, op)) {
			operator = op;
			break;
		}
	}

	if (operator !== undefined) {
		return parseTSTypeOperator(parser, operator);
	} else if (isContextual(parser, "infer")) {
		return parseTSInferType(parser);
	} else {
		return parseTSArrayTypeOrHigher(parser);
	}
}

function parseTSUnionOrIntersectionType(
	parser: JSParser,
	kind: "JSUnionTypeAnnotation" | "TSIntersectionTypeAnnotation",
	parseConstituentType: ParserCallback<AnyTSPrimary>,
	operator: TokenType,
): AnyTSPrimary {
	eat(parser, operator);
	let type = parseConstituentType(parser);

	if (match(parser, operator)) {
		const types = [type];
		while (eat(parser, operator)) {
			types.push(parseConstituentType(parser));
		}

		const start = parser.getLoc(type).start;
		if (kind === "JSUnionTypeAnnotation") {
			type = parser.finishNode(
				start,
				{
					type: "TSUnionTypeAnnotation",
					types,
				},
			);
		} else if (kind === "TSIntersectionTypeAnnotation") {
			type = parser.finishNode(
				start,
				{
					type: "TSIntersectionTypeAnnotation",
					types,
				},
			);
		}
	}

	return type;
}

function parseTSIntersectionTypeAnnotationOrHigher(
	parser: JSParser,
): AnyTSPrimary {
	return parseTSUnionOrIntersectionType(
		parser,
		"TSIntersectionTypeAnnotation",
		parseTSTypeOperatorOrHigher,
		tt.bitwiseAND,
	);
}

function parseUnionTypeAnnotationOrHigher(parser: JSParser) {
	return parseTSUnionOrIntersectionType(
		parser,
		"JSUnionTypeAnnotation",
		parseTSIntersectionTypeAnnotationOrHigher,
		tt.bitwiseOR,
	);
}

function tsIsStartOfFunctionType(parser: JSParser) {
	if (isRelational(parser, "<")) {
		return true;
	}
	return (
		match(parser, tt.parenL) &&
		lookaheadTS(parser, tsIsUnambiguouslyStartOfFunctionType)
	);
}

function tsSkipParameterStart(parser: JSParser): boolean {
	if (match(parser, tt.name) || match(parser, tt._this)) {
		next(parser);
		return true;
	}

	if (match(parser, tt.braceL)) {
		let braceStackCounter = 1;
		next(parser);

		while (braceStackCounter > 0) {
			if (match(parser, tt.braceL)) {
				braceStackCounter++;
			} else if (match(parser, tt.braceR)) {
				braceStackCounter--;
			}

			next(parser);
		}
		return true;
	}

	if (match(parser, tt.bracketL)) {
		let braceStackCounter = 1;
		next(parser);

		while (braceStackCounter > 0) {
			if (match(parser, tt.bracketL)) {
				braceStackCounter++;
			} else if (match(parser, tt.bracketR)) {
				braceStackCounter--;
			}

			next(parser);
		}
		return true;
	}

	return false;
}

function tsIsUnambiguouslyStartOfFunctionType(parser: JSParser): boolean {
	next(parser);
	if (match(parser, tt.parenR) || match(parser, tt.ellipsis)) {
		// ()
		// (...
		return true;
	}
	if (tsSkipParameterStart(parser)) {
		if (
			match(parser, tt.colon) ||
			match(parser, tt.comma) ||
			match(parser, tt.question) ||
			match(parser, tt.eq)
		) {
			// (xxx :
			// (xxx ,
			// (xxx ?
			// (xxx =
			return true;
		}
		if (match(parser, tt.parenR)) {
			next(parser);
			if (match(parser, tt.arrow)) {
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
	let start: Position = parser.getPosition();
	expectTSEnabled(parser, "type annotation", start);

	pushScope(parser, "TYPE", true);
	expect(parser, returnToken);

	let hasAsserts = eatContextual(parser, "asserts");
	let parameterName: JSIdentifier;
	let typePredicateVariable;
	if (tsIsIdentifier(parser)) {
		typePredicateVariable = tryTSParse(parser, parseTSTypePredicatePrefix);
	}
	if (typePredicateVariable === undefined) {
		if (hasAsserts) {
			parameterName = parseIdentifier(parser);
			if (parameterName === undefined) {
				throw Error("Should have an identifier after asserts");
			}
		} else {
			popScope(parser, "TYPE");
			return parseTSTypeAnnotation(parser, /* eatColon */ false, start);
		}
	} else {
		parameterName = typePredicateVariable;
	}

	let type;
	if (typePredicateVariable) {
		type = parseTSTypeAnnotation(parser, /* eatColon */ false);
		start = parser.getLoc(typePredicateVariable).start;
	}

	popScope(parser, "TYPE");

	return parser.finishNode(
		start,
		{
			type: "TSTypePredicate",
			asserts: hasAsserts,
			parameterName,
			typeAnnotation: type,
		},
	);
}

function tryTSParseTypeAnnotation(parser: JSParser): undefined | AnyTSPrimary {
	return match(parser, tt.colon) ? parseTSTypeAnnotation(parser) : undefined;
}

function tryTSParseType(parser: JSParser): undefined | AnyTSPrimary {
	return tsEatThenParseType(parser, tt.colon);
}

function parseTSTypePredicatePrefix(parser: JSParser): undefined | JSIdentifier {
	const id = parseIdentifier(parser);
	if (isContextual(parser, "is") && !hasPrecedingLineBreak(parser)) {
		next(parser);
		return id;
	} else {
		return undefined;
	}
}

export function parseTSTypeAnnotation(
	parser: JSParser,
	eatColon: boolean = true,
	start: Position = parser.getPosition(),
): AnyTSPrimary {
	expectTSEnabled(parser, "type annotation", start);

	pushScope(parser, "TYPE", true);

	if (eatColon) {
		expect(parser, tt.colon);
	}

	const typeAnnotation = parseTSType(parser, start);
	popScope(parser, "TYPE");
	return typeAnnotation;
}

/** Be sure to be in a type context before calling parser. using `tsInType`.*/
function parseTSType(
	parser: JSParser,
	start: Position = parser.getPosition(),
): AnyTSPrimary {
	pushScope(parser, "TYPE", true);

	const type = parseTSNonConditionalType(parser);
	if (hasPrecedingLineBreak(parser) || !eat(parser, tt._extends)) {
		popScope(parser, "TYPE");
		return type;
	}

	const checkType = type;

	const extendsType = parseTSNonConditionalType(parser);
	expect(parser, tt.question);

	const trueType = parseTSType(parser);
	expect(parser, tt.colon);

	const falseType = parseTSType(parser);
	popScope(parser, "TYPE");

	return parser.finishNode(
		start,
		{
			type: "TSConditionalType",
			checkType,
			extendsType,
			trueType,
			falseType,
		},
	);
}

function parseTSNonConditionalType(parser: JSParser): AnyTSPrimary {
	if (tsIsStartOfFunctionType(parser)) {
		return parseTSFunctionType(parser);
	}

	if (match(parser, tt._new)) {
		// As in `new () => Date`
		return parseTSConstructorType(parser);
	}

	return parseUnionTypeAnnotationOrHigher(parser);
}

export function parseTSTypeAssertion(parser: JSParser): TSTypeAssertion {
	const start = parser.getPosition();
	expectTSEnabled(parser, "type assertion", start);

	const _const = tryTSNextParseConstantContext(parser);
	const typeAnnotation = _const || tsNextThenParseType(parser);
	expectRelational(parser, ">");

	const expression = parseMaybeUnary(parser, "ts type assertion");
	if (_const) {
		tsCheckLiteralForConstantContext(parser, expression);
	}

	return parser.finishNode(
		start,
		{
			type: "TSTypeAssertion",
			expression,
			typeAnnotation,
		},
	);
}

export function parseTSHeritageClause(
	parser: JSParser,
	descriptor: string,
): Array<TSExpressionWithTypeArguments> {
	expectTSEnabled(parser, "heritage clause");

	const originalStart = parser.state.startPos;
	const delimitedList = parseTSDelimitedList(
		parser,
		"HeritageClauseElement",
		parseTSExpressionWithTypeArguments,
	);

	if (delimitedList.length === 0) {
		unexpectedDiagnostic(
			parser,
			{
				start: originalStart,
				description: descriptions.JS_PARSER.TS_EMPTY_LIST(descriptor),
			},
		);
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
	if (isRelational(parser, "<")) {
		typeParameters = parseTSTypeArguments(parser);
	}

	return parser.finishNode(
		start,
		{
			type: "TSExpressionWithTypeArguments",
			expression,
			typeParameters,
		},
	);
}

export function parseTSInterfaceDeclaration(
	parser: JSParser,
	start: Position,
): TSInterfaceDeclaration {
	expectTSEnabled(parser, "interface declaration", start);

	pushScope(parser, "TYPE", true);
	const id = parseBindingIdentifier(parser);
	const typeParameters = tryParseTSTypeParameters(parser);

	let _extends;
	if (eat(parser, tt._extends)) {
		_extends = parseTSHeritageClause(parser, "extends");
	}

	const bodyStart = parser.getPosition();
	const bodyItems = parseTSObjectTypeMembers(parser);
	const body: TSInterfaceBody = parser.finishNode(
		bodyStart,
		{
			type: "TSInterfaceBody",
			body: bodyItems,
		},
	);

	popScope(parser, "TYPE");
	return parser.finishNode(
		start,
		{
			type: "TSInterfaceDeclaration",
			id,
			body,
			typeParameters,
			extends: _extends,
		},
	);
}

export function parseTSTypeAlias(parser: JSParser, start: Position): TSTypeAlias {
	const id = parseBindingIdentifier(parser);
	const typeParameters = tryParseTSTypeParameters(parser);
	const typeAnnotation = tsExpectThenParseType(parser, tt.eq);
	semicolon(parser);
	return parser.finishNode(
		start,
		{
			type: "TSTypeAlias",
			id,
			typeParameters,
			right: typeAnnotation,
		},
	);
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
	if (match(parser, token)) {
		return tsNextThenParseType(parser);
	} else {
		return undefined;
	}
}

function tsExpectThenParseType(parser: JSParser, token: TokenType): AnyTSPrimary {
	return tsDoThenParseType(
		parser,
		() => {
			expect(parser, token);
		},
	);
}

export function tsNextThenParseType(parser: JSParser): AnyTSPrimary {
	return tsDoThenParseType(parser, () => next(parser));
}

function tsDoThenParseType(parser: JSParser, cb: VoidCallback): AnyTSPrimary {
	cb();
	return parseTSType(parser);
}

function parseTSEnumMember(parser: JSParser): TSEnumMember {
	const start = parser.getPosition();
	// Computed property names are grammar errors in an enum, so accept just string literal or identifier.
	const id: JSStringLiteral | JSIdentifier = match(parser, tt.string)
		? parseStringLiteral(parser)
		: parseIdentifier(parser, /* liberal */ true);

	let initializer: undefined | AnyJSExpression;
	if (eat(parser, tt.eq)) {
		initializer = parseMaybeAssign<AnyJSExpression>(
			parser,
			"ts enum member initializer",
		);
	}

	return parser.finishNode(
		start,
		{
			type: "TSEnumMember",
			initializer,
			id,
		},
	);
}

export function parseTSEnumDeclaration(
	parser: JSParser,
	start: Position,
	isConst: boolean,
): TSEnumDeclaration {
	parser.addDiagnosticFilter({
		message: descriptions.JS_PARSER.RESERVED_WORD("enum").message,
		start,
	});

	const id = parseBindingIdentifier(parser);

	const braceOpenStart = parser.getPosition();
	const openContext = expectOpening(
		parser,
		tt.braceL,
		tt.braceR,
		"ts enum declaration",
	);

	const members = parseTSDelimitedList(parser, "EnumMembers", parseTSEnumMember);
	expectClosing(parser, openContext);

	return parser.finishNodeWithStarts(
		[braceOpenStart, start],
		{
			type: "TSEnumDeclaration",
			members,
			id,
			const: isConst,
		},
	);
}

export function parseTSModuleBlock(parser: JSParser): TSModuleBlock {
	const start = parser.getPosition();

	const openContext = expectOpening(
		parser,
		tt.braceL,
		tt.braceR,
		"ts module block",
	);

	// Inside of a module block is considered 'top-level', meaning it can have imports and exports.
	const {body} = parseBlockOrModuleBlockBody(
		parser,
		/* allowDirectives */ false,
		/* topLevel */ true,
		openContext,
	);
	return parser.finishNode(
		start,
		{
			type: "TSModuleBlock",
			body,
		},
	);
}

export function parseTSModuleOrNamespaceDeclaration(
	parser: JSParser,
	start: Position,
): TSModuleDeclaration {
	const id = parseBindingIdentifier(parser);

	let body;
	if (eat(parser, tt.dot)) {
		body = parseTSModuleOrNamespaceDeclaration(parser, parser.getPosition());
	} else {
		body = parseTSModuleBlock(parser);
	}

	return parser.finishNode(
		start,
		{
			type: "TSModuleDeclaration",
			id,
			body,
		},
	);
}

export function parseTSAmbientExternalModuleDeclaration(
	parser: JSParser,
	start: Position,
): TSModuleDeclaration {
	let global;
	let id;
	if (isContextual(parser, "global")) {
		global = true;
		id = parseBindingIdentifier(parser);
	} else if (match(parser, tt.string)) {
		id = parseStringLiteral(parser);
	} else {
		throw parser.unexpected();
	}

	let body;
	if (match(parser, tt.braceL)) {
		body = parseTSModuleBlock(parser);
	} else {
		semicolon(parser);
	}

	return parser.finishNode(
		start,
		{
			type: "TSModuleDeclaration",
			id,
			global,
			body,
		},
	);
}

export function parseTSImportEqualsDeclaration(
	parser: JSParser,
	start: Position,
	isExport: boolean = false,
): TSImportEqualsDeclaration {
	expectTSEnabled(parser, "import equals declaration", start);

	const id = parseBindingIdentifier(parser);
	expect(parser, tt.eq);

	const moduleReference = parseTSModuleReference(parser);
	semicolon(parser);

	return parser.finishNode(
		start,
		{
			type: "TSImportEqualsDeclaration",
			id,
			moduleReference,
			isExport,
		},
	);
}

function tsIsExternalModuleReference(parser: JSParser): boolean {
	return (
		isContextual(parser, "require") &&
		lookaheadState(parser).tokenType === tt.parenL
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
	expectContextual(parser, "require");
	const openContext = expectOpening(
		parser,
		tt.parenL,
		tt.parenR,
		"ts external module reference",
	);

	let expression: JSStringLiteral;
	if (match(parser, tt.string)) {
		expression = parseStringLiteral(parser);
	} else {
		unexpectedDiagnostic(
			parser,
			{
				description: descriptions.JS_PARSER.TS_EXTERNAL_MODULE_REFERENCE_ARG_NOT_STRING,
			},
		);

		// Skip as much of the next expression as we can
		parseExpressionAtom(parser, "ts external module reference expression");

		// Create a fake string literal
		expression = parser.finishNode(
			start,
			{
				type: "JSStringLiteral",
				value: "",
			},
		);
	}

	expectClosing(parser, openContext);

	return parser.finishNode(
		start,
		{
			type: "TSExternalModuleReference",
			expression,
		},
	);
}

// Utilities
type ParserCallback<T> = (parser: JSParser) => T;

function lookaheadTS<T>(parser: JSParser, f: ParserCallback<T>): T {
	const state = cloneState(parser);
	const res = f(parser);
	parser.state = state;
	return res;
}

function tryTSParse<T>(
	parser: JSParser,
	f: ParserCallback<undefined | false | T>,
): undefined | T {
	const state = cloneState(parser);
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
	| JSFunctionDeclaration
	| JSClassDeclaration
	| JSVariableDeclarationStatement
	| TSDeclareFunction
	| TSModuleDeclaration
	| TSTypeAlias
	| TSInterfaceDeclaration;

export function parseTSDeclare(parser: JSParser, start: Position): TSDeclareNode {
	let starttype = parser.state.tokenType;
	let kind: undefined | JSVariableDeclarationKind;
	if (isContextual(parser, "let")) {
		starttype = tt._var;
		kind = "let";
	}

	if (
		starttype === tt._const &&
		match(parser, tt._const) &&
		isLookaheadContextual(parser, "enum")
	) {
		// `const enum = 0;` not allowed because 'enum' is a strict mode reserved word.
		expect(parser, tt._const);
		expectContextual(parser, "enum");
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
		case tt._var: {
			kind =
				kind === undefined
					? assertVarKind(String(parser.state.tokenValue))
					: kind;
			return {
				declare: true,
				...parseVarStatement(parser, start, kind),
			};
		}

		case tt.name: {
			const value = String(parser.state.tokenValue);

			if (value === "global") {
				return {
					declare: true,
					...parseTSAmbientExternalModuleDeclaration(parser, start),
				};
			} else if (isTSDeclarationStart(parser)) {
				const id = parseReferenceIdentifier(parser);
				const decl = parseTSTypeExpressionStatement(parser, start, id);

				if (decl === undefined) {
					throw new Error("Should have returned a node");
				}

				if (
					decl.type !== "TSInterfaceDeclaration" &&
					decl.type !== "TSTypeAlias" &&
					decl.type !== "TSEnumDeclaration" &&
					decl.type !== "JSFunctionDeclaration" &&
					decl.type !== "JSClassDeclaration" &&
					decl.type !== "JSVariableDeclarationStatement" &&
					decl.type !== "TSDeclareFunction" &&
					decl.type !== "TSModuleDeclaration"
				) {
					throw new Error(
						"Encountered a non-TS declare node when calling parseTSTypeExpressionStatement",
					);
				}

				return {...decl, declare: true};
			}
		}
	}

	unexpectedDiagnostic(
		parser,
		{
			description: descriptions.JS_PARSER.TS_UNKNOWN_DECLARE_START,
		},
	);

	// Fake node
	const loc = parser.finishLoc(start);
	return {
		type: "JSVariableDeclarationStatement",
		loc,
		declaration: {
			type: "JSVariableDeclaration",
			loc,
			kind: "var",
			declarations: [
				{
					type: "JSVariableDeclarator",
					loc,
					id: toBindingIdentifier(
						parser,
						createUnknownIdentifier(parser, "typescript declare start", start),
					),
					init: undefined,
				},
			],
		},
	};
}

export function parseTSTypeExpressionStatement(
	parser: JSParser,
	start: Position,
	expr: AnyJSExpression,
): undefined | TSDeclareNode | TSTypeAlias | TSInterfaceDeclaration {
	// TODO TypeScript does not like isLineTerminator(parser, )
	if (expr.type !== "JSReferenceIdentifier") {
		return undefined;
	}

	if (hasPrecedingLineBreak(parser)) {
		return undefined;
	}

	switch (expr.name) {
		case "declare":
			if (
				match(parser, tt._class) ||
				match(parser, tt.name) ||
				match(parser, tt._function) ||
				match(parser, tt._const) ||
				match(parser, tt._var) ||
				match(parser, tt._export)
			) {
				return parseTSDeclare(parser, start);
			} else {
				break;
			}

		case "interface": {
			parser.addDiagnosticFilter({
				message: markup`interface is a reserved word`,
				start,
			});

			return parseTSInterfaceDeclaration(parser, start);
		}

		case "type": {
			expectTSEnabled(parser, "type alias", start);
			// TODO perform some lookahead to make sure we want to do this
			return parseTSTypeAlias(parser, start);
		}

		case "abstract":
			if (match(parser, tt._class)) {
				expectTSEnabled(parser, "abstract class", start);
				return parseTSAbstractClass(parser, start);
			} else {
				break;
			}

		case "enum": {
			if (match(parser, tt.name)) {
				expectTSEnabled(parser, "enum declaration", start);
				return parseTSEnumDeclaration(parser, start, /* isConst */ false);
			} else {
				break;
			}
		}

		case "module":
			if (match(parser, tt.string)) {
				expectTSEnabled(parser, "ambient external module declaration", start);
				return parseTSAmbientExternalModuleDeclaration(parser, start);
			} else if (match(parser, tt.name) && !isLineTerminator(parser)) {
				expectTSEnabled(parser, "module or namespace declaration", start);
				return parseTSModuleOrNamespaceDeclaration(parser, start);
			} else {
				break;
			}

		case "namespace": {
			if (!match(parser, tt.name)) {
				return undefined;
			}

			expectTSEnabled(parser, "module or namespace declaration", start);
			return parseTSModuleOrNamespaceDeclaration(parser, start);
		}

		// TODO abstract this into typescript.js
		case "global":
			// `global { }` (with no `declare`) may appear inside an ambient module declaration.
			// Would like to use parseTSAmbientExternalModuleDeclaration here, but already ran past 'global'.
			if (match(parser, tt.braceL)) {
				expectTSEnabled(parser, "module declaration", start);
				const global = true;
				const id = toBindingIdentifier(parser, expr);
				const body = parseTSModuleBlock(parser);
				return parser.finishNode(
					start,
					{
						type: "TSModuleDeclaration",
						global,
						id,
						body,
					},
				);
			}
	}

	return undefined;
}

export function parseTSAbstractClass(
	parser: JSParser,
	start: Position,
): JSClassDeclaration {
	return {
		...parseClassDeclaration(parser, start),
		abstract: true,
	};
}

export function parseTSExportDefaultAbstractClass(
	parser: JSParser,
	start: Position,
): JSClassDeclaration {
	return {
		...parseExportDefaultClassDeclaration(parser, start),
		abstract: true,
	};
}

export function parseTSTypeArguments(
	parser: JSParser,
): TSTypeParameterInstantiation {
	const start = parser.getPosition();
	expectTSEnabled(parser, "type arguments", start);

	pushScope(parser, "TYPE", true);

	const params = tsInNoContext(
		parser,
		() => {
			expectRelational(parser, "<");
			return parseTSDelimitedList(
				parser,
				"TypeParametersOrArguments",
				parseTSType,
			);
		},
	);

	// This reads the next token after the `>` too, so do parser.in the enclosing context.

	// But be sure not to parse a regex in the jsx expression `<C<number> />`, so set exprAllowed = false
	parser.state.exprAllowed = false;
	popScope(parser, "TYPE");
	expectRelational(parser, ">");

	return parser.finishNode(
		start,
		{
			type: "TSTypeParameterInstantiation",
			params,
		},
	);
}

export function isTSDeclarationStart(parser: JSParser): boolean {
	if (match(parser, tt.name)) {
		switch (parser.state.tokenValue) {
			case "abstract":
			case "declare":
			case "enum":
			case "interface":
			case "module":
			case "namespace":
			case "type":
				return true;
		}
	}

	return false;
}

export function parseTSAccessModifier(
	parser: JSParser,
): undefined | ConstTSAccessibility {
	return parseTSModifier(parser, ["public", "protected", "private"]);
}

export function isTSAbstractClass(parser: JSParser): boolean {
	return (
		isContextual(parser, "abstract") &&
		lookaheadState(parser).tokenType === tt._class
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
	if (!isSyntaxEnabled(parser, "ts")) {
		return undefined;
	}

	if (match(parser, tt._import)) {
		// `export const A =B;`
		expect(parser, tt._import);
		return parseTSImportEqualsDeclaration(parser, start, /* isExport */ true);
	}

	if (eat(parser, tt.eq)) {
		// `export = x;`
		const expression = parseExpression(parser, "ts export assignment");
		semicolon(parser);
		return parser.finishNode(
			start,
			{
				type: "TSExportAssignment",
				expression,
			},
		);
	}

	if (eatContextual(parser, "as")) {
		// `export as namespace A;`
		// See `parseNamespaceExportDeclaration` in TypeScript's own parser
		expectContextual(parser, "namespace");
		const id = parseIdentifier(parser);
		semicolon(parser);
		return parser.finishNode(
			start,
			{
				type: "TSNamespaceExportDeclaration",
				id,
			},
		);
	}

	return undefined;
}
