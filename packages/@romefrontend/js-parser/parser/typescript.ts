/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position} from "@romefrontend/parser-core";
import {JSParser} from "../parser";
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
} from "@romefrontend/ast";
import {descriptions} from "@romefrontend/diagnostics";
import {NumberTokenValue} from "../tokenizer";
import {toTargetAssignmentPattern} from "./lval";
import {markup} from "@romefrontend/cli-layout";

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

/** Parses a modifier matching one the given modifier names.*/
export function parseTSModifier<T extends ConstTSModifier>(
	parser: JSParser,
	allowedModifiers: Array<T>,
): undefined | T {
	if (!parser.match(tt.name)) {
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
	if (parser.match(tt.eof)) {
		return true;
	}

	switch (kind) {
		case "EnumMembers":
		case "TypeMembers":
			return parser.match(tt.braceR);

		case "HeritageClauseElement":
			return parser.match(tt.braceL);

		case "TupleElementTypes":
			return parser.match(tt.bracketR);

		case "TypeParametersOrArguments":
			return parser.isRelational(">");
	}

	throw new Error("Unreachable");
}

function expectTSEnabled(
	parser: JSParser,
	label: string,
	start: Position = parser.getPosition(),
) {
	if (parser.isSyntaxEnabled("ts")) {
		return;
	}

	parser.unexpectedDiagnostic({
		start,
		description: descriptions.JS_PARSER.TS_REQUIRED(label),
	});
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
			parser.expectRelational("<");
		}
	}

	const result = parseTSDelimitedList(parser, kind, parseElement);

	if (bracket) {
		parser.expect(tt.bracketR);
	} else {
		parser.expectRelational(">");
	}

	return result;
}

function parseTSImportType(parser: JSParser): TSImportType {
	const start = parser.getPosition();
	parser.expect(tt._import);
	const openContext = parser.expectOpening(
		tt.parenL,
		tt.parenR,
		"ts import type",
	);

	if (!parser.match(tt.string)) {
		parser.unexpectedDiagnostic({
			description: descriptions.JS_PARSER.TS_IMPORT_ARG_NOT_STRING,
		});
	}

	const argument = parseExpressionAtom(parser, "ts import argument");
	parser.expectClosing(openContext);

	let qualifier;
	if (parser.eat(tt.dot)) {
		qualifier = parseTSEntityName(parser, /* allowReservedWords */ true);
	}

	let typeParameters;
	if (parser.isRelational("<")) {
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
	while (parser.eat(tt.dot)) {
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
	if (!parser.hasPrecedingLineBreak() && parser.isRelational("<")) {
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
	parser.next();
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
	parser.next();
	return parser.finishNode(
		start,
		{
			type: "TSThisType",
		},
	);
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
	if (parser.isRelational("<")) {
		return parseTSTypeParameters(parser);
	} else {
		return undefined;
	}
}

export function maybeParseTSTypeArguments(
	parser: JSParser,
): undefined | TSTypeParameterInstantiation {
	if (parser.isRelational("<")) {
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
	if (parser.isRelational("<")) {
		return parseTSTypeParameters(parser);
	} else {
		return undefined;
	}
}

export function parseTSTypeParameters(
	parser: JSParser,
): TSTypeParameterDeclaration {
	const start = parser.getPosition();

	parser.expectRelational("<");

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
	if (parser.lookaheadState().tokenType === tt._const) {
		parser.next();
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
			parser.unexpectedDiagnostic({
				loc: node.loc,
				description: descriptions.JS_PARSER.TS_CONSTANT_NOT_LITERAL,
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
	const openContext = parser.expectOpening(
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
			parser.unexpectedDiagnostic({
				loc: pattern.loc,
				description: descriptions.JS_PARSER.TS_INVALID_SIGNATURE_BINDING_NODE,
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
	parser.semicolon();
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
	parser.next(); // Skip '{'
	return parser.eat(tt.name) && parser.match(tt.colon);
}

export function tryTSParseIndexSignature(
	parser: JSParser,
	start: Position,
): undefined | TSIndexSignature {
	if (
		!(parser.match(tt.bracketL) &&
		lookaheadTS(parser, tsIsUnambiguouslyIndexSignature))
	) {
		return undefined;
	}

	parser.expect(tt.bracketL);

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

	parser.expect(tt.bracketR);

	const typeAnnotation = tryTSParseTypeAnnotation(parser);

	parser.semicolon();
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
	const optional = parser.eat(tt.question);

	if (!readonly && (parser.match(tt.parenL) || parser.isRelational("<"))) {
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
	if (parser.match(tt.parenL) || parser.isRelational("<")) {
		return parseTSCallSignatureDeclaration(parser);
	}

	if (
		parser.match(tt._new) &&
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
	parser.next();
	return parser.match(tt.parenL) || parser.isRelational("<");
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
	const openContext = parser.expectOpening(
		tt.braceL,
		tt.braceR,
		"ts object type members",
	);
	const members = parseTSList(parser, "TypeMembers", parseTSTypeMember);
	parser.expectClosing(openContext);
	return members;
}

function tsIsStartOfMappedType(parser: JSParser): boolean {
	parser.next();

	if (parser.eat(tt.plusMin)) {
		return parser.isContextual("readonly");
	}

	if (parser.isContextual("readonly")) {
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

	const openContext = parser.expectOpening(
		tt.braceL,
		tt.braceR,
		"ts mapped type",
	);

	let readonly: TSMappedTypeBoolean;
	if (parser.match(tt.plusMin)) {
		readonly = toPlusMin(parser.state.tokenValue);
		parser.next();
		parser.expectContextual("readonly");
	} else if (parser.eatContextual("readonly")) {
		readonly = true;
	}

	const paramOpenContext = parser.expectOpening(
		tt.bracketL,
		tt.bracketR,
		"ts mapped type parameter",
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
			parser.unexpectedDiagnostic({
				loc: type.loc,
				description: descriptions.JS_PARSER.TS_REQUIRED_FOLLOWS_OPTIONAL,
			});
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
	let optional = parser.eat(tt.question);
	let name: undefined | JSBindingIdentifier;

	if (parser.eat(tt.colon)) {
		if (
			typeAnnotation.type === "TSTypeReference" &&
			typeAnnotation.typeName.type === "JSReferenceIdentifier"
		) {
			name = toBindingIdentifier(parser, typeAnnotation.typeName);
			typeAnnotation = parseTSType(parser);
		} else {
			parser.unexpectedDiagnostic({
				loc: typeAnnotation.loc,
				description: descriptions.JS_PARSER.TS_TUPLE_ELEMENT_LABEL_INCORRECT,
			});
		}

		if (parser.match(tt.question)) {
			parser.unexpectedDiagnostic({
				description: descriptions.JS_PARSER.TS_TUPLE_ELEMENT_OPTIONAL_TRAILING,
			});
			parser.next();
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
	if (parser.eat(tt.ellipsis)) {
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
		parser.unexpectedDiagnostic({
			loc: elem.loc,
			description: descriptions.JS_PARSER.TS_TUPLE_ELEMENT_OPTIONAL_REST,
		});
	}

	return {
		isRest,
		type: elem,
	};
}

function parseTSParenthesizedType(parser: JSParser): TSParenthesizedType {
	const start = parser.getPosition();
	const openContext = parser.expectOpening(
		tt.parenL,
		tt.parenR,
		"ts parenthesized type",
	);
	const typeAnnotation = parseTSType(parser);
	parser.expectClosing(openContext);
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
	parser.expect(tt._new);

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
		parser.unexpectedDiagnostic({
			loc: parser.getLoc(templateNode.expressions[0]),
			description: descriptions.JS_PARSER.TS_TEMPLATE_LITERAL_WITH_SUBSTITUION,
		});
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
			if (parser.match(tt._void)) {
				type = "TSVoidKeywordTypeAnnotation";
			} else if (parser.match(tt._null)) {
				type = "TSNullKeywordTypeAnnotation";
			} else {
				type = keywordTypeFromName(String(parser.state.tokenValue));
			}

			if (type !== undefined && parser.lookaheadState().tokenType !== tt.dot) {
				const start = parser.getPosition();
				parser.next();
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
			if (parser.isContextual("is") && !parser.hasPrecedingLineBreak()) {
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

	parser.unexpectedDiagnostic({
		description: descriptions.JS_PARSER.TS_UNKNOWN_NON_ARRAY_START,
	});
	parser.next();
	return parser.finishNode(
		parser.getPosition(),
		{
			type: "TSTypeReference",
			typeName: toReferenceIdentifier(
				parser,
				parser.createUnknownIdentifier("ts non array type start"),
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
			parser.next();
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
			parser.next();
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
			const value = parser.match(tt._true);
			parser.next();
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
				parser.next();

				if (!parser.match(tt.num)) {
					parser.unexpectedDiagnostic({
						description: descriptions.JS_PARSER.TYPE_NUMERIC_LITERAL_EXPECTED,
					});
					parser.next();
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
				parser.next();
				return parser.finishNode(
					start,
					{
						type: "TSNumericLiteralTypeAnnotation",
						value: -value,
						format,
					},
				);
			} else {
				parser.unexpectedDiagnostic({
					description: descriptions.JS_PARSER.TYPE_NUMERIC_LITERAL_PLUS,
				});
				parser.next();

				if (!parser.match(tt.num)) {
					parser.unexpectedDiagnostic({
						description: descriptions.JS_PARSER.TYPE_NUMERIC_LITERAL_EXPECTED,
					});
					parser.next();
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

	while (!parser.hasPrecedingLineBreak() && parser.eat(tt.bracketL)) {
		if (parser.match(tt.bracketR)) {
			const start = parser.getLoc(type).start;
			const elementType = type;
			parser.expect(tt.bracketR);
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
			parser.expect(tt.bracketR);
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
	parser.expectContextual(operator);

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
			parser.unexpectedDiagnostic({
				loc: node.loc,
				description: descriptions.JS_PARSER.TS_INVALID_READONLY_MODIFIER,
			});
			break;
		}
	}
}

function parseTSInferType(parser: JSParser): TSInferType {
	const inferStart = parser.getPosition();
	parser.expectContextual("infer");

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
		if (parser.isContextual(op)) {
			operator = op;
			break;
		}
	}

	if (operator !== undefined) {
		return parseTSTypeOperator(parser, operator);
	} else if (parser.isContextual("infer")) {
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
	parser.eat(operator);
	let type = parseConstituentType(parser);

	if (parser.match(operator)) {
		const types = [type];
		while (parser.eat(operator)) {
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
	if (parser.isRelational("<")) {
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
	let start = parser.getPosition();
	expectTSEnabled(parser, "type annotation", start);

	parser.pushScope("TYPE", true);
	parser.expect(returnToken);

	let hasAsserts = parser.eatContextual("asserts");
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
			parser.popScope("TYPE");
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

	parser.popScope("TYPE");

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
	return parser.match(tt.colon) ? parseTSTypeAnnotation(parser) : undefined;
}

function tryTSParseType(parser: JSParser): undefined | AnyTSPrimary {
	return tsEatThenParseType(parser, tt.colon);
}

function parseTSTypePredicatePrefix(parser: JSParser): undefined | JSIdentifier {
	const id = parseIdentifier(parser);
	if (parser.isContextual("is") && !parser.hasPrecedingLineBreak()) {
		parser.next();
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

	parser.pushScope("TYPE", true);

	if (eatColon) {
		parser.expect(tt.colon);
	}

	const typeAnnotation = parseTSType(parser, start);
	parser.popScope("TYPE");
	return typeAnnotation;
}

/** Be sure to be in a type context before calling parser. using `tsInType`.*/
function parseTSType(
	parser: JSParser,
	start: Position = parser.getPosition(),
): AnyTSPrimary {
	parser.pushScope("TYPE", true);

	const type = parseTSNonConditionalType(parser);
	if (parser.hasPrecedingLineBreak() || !parser.eat(tt._extends)) {
		parser.popScope("TYPE");
		return type;
	}

	const checkType = type;

	const extendsType = parseTSNonConditionalType(parser);
	parser.expect(tt.question);

	const trueType = parseTSType(parser);
	parser.expect(tt.colon);

	const falseType = parseTSType(parser);
	parser.popScope("TYPE");

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

	if (parser.match(tt._new)) {
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
	parser.expectRelational(">");

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
		parser.unexpectedDiagnostic({
			start: originalStart,
			description: descriptions.JS_PARSER.TS_EMPTY_LIST(descriptor),
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
	if (parser.isRelational("<")) {
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

	parser.pushScope("TYPE", true);
	const id = parseBindingIdentifier(parser);
	const typeParameters = tryParseTSTypeParameters(parser);

	let _extends;
	if (parser.eat(tt._extends)) {
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

	parser.popScope("TYPE");
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
	parser.semicolon();
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
	if (parser.match(token)) {
		return tsNextThenParseType(parser);
	} else {
		return undefined;
	}
}

function tsExpectThenParseType(parser: JSParser, token: TokenType): AnyTSPrimary {
	return tsDoThenParseType(
		parser,
		() => {
			parser.expect(token);
		},
	);
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
	const id: JSStringLiteral | JSIdentifier = parser.match(tt.string)
		? parseStringLiteral(parser)
		: parseIdentifier(parser, /* liberal */ true);

	let initializer: undefined | AnyJSExpression;
	if (parser.eat(tt.eq)) {
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
	const openContext = parser.expectOpening(
		tt.braceL,
		tt.braceR,
		"ts enum declaration",
	);

	const members = parseTSDelimitedList(parser, "EnumMembers", parseTSEnumMember);
	parser.expectClosing(openContext);

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

	const openContext = parser.expectOpening(
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
	if (parser.eat(tt.dot)) {
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
	if (parser.isContextual("global")) {
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
	parser.expect(tt.eq);

	const moduleReference = parseTSModuleReference(parser);
	parser.semicolon();

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
		parser.isContextual("require") &&
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
	parser.expectContextual("require");
	const openContext = parser.expectOpening(
		tt.parenL,
		tt.parenR,
		"ts external module reference",
	);

	let expression: JSStringLiteral;
	if (parser.match(tt.string)) {
		expression = parseStringLiteral(parser);
	} else {
		parser.unexpectedDiagnostic({
			description: descriptions.JS_PARSER.TS_EXTERNAL_MODULE_REFERENCE_ARG_NOT_STRING,
		});

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

	parser.expectClosing(openContext);

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
	if (parser.isContextual("let")) {
		starttype = tt._var;
		kind = "let";
	}

	if (
		starttype === tt._const &&
		parser.match(tt._const) &&
		parser.isLookaheadContextual("enum")
	) {
		// `const enum = 0;` not allowed because 'enum' is a strict mode reserved word.
		parser.expect(tt._const);
		parser.expectContextual("enum");
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

	parser.unexpectedDiagnostic({
		description: descriptions.JS_PARSER.TS_UNKNOWN_DECLARE_START,
	});

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
						parser.createUnknownIdentifier("typescript declare start", start),
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
	// TODO TypeScript does not like parser.isLineTerminator()
	if (expr.type !== "JSReferenceIdentifier") {
		return undefined;
	}

	if (parser.hasPrecedingLineBreak()) {
		return undefined;
	}

	switch (expr.name) {
		case "declare":
			if (
				parser.match(tt._class) ||
				parser.match(tt.name) ||
				parser.match(tt._function) ||
				parser.match(tt._const) ||
				parser.match(tt._var) ||
				parser.match(tt._export)
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
			if (parser.match(tt._class)) {
				expectTSEnabled(parser, "abstract class", start);
				return parseTSAbstractClass(parser, start);
			} else {
				break;
			}

		case "enum": {
			if (parser.match(tt.name)) {
				expectTSEnabled(parser, "enum declaration", start);
				return parseTSEnumDeclaration(parser, start, /* isConst */ false);
			} else {
				break;
			}
		}

		case "module":
			if (parser.match(tt.string)) {
				expectTSEnabled(parser, "ambient external module declaration", start);
				return parseTSAmbientExternalModuleDeclaration(parser, start);
			} else if (parser.match(tt.name) && !parser.isLineTerminator()) {
				expectTSEnabled(parser, "module or namespace declaration", start);
				return parseTSModuleOrNamespaceDeclaration(parser, start);
			} else {
				break;
			}

		case "namespace": {
			if (!parser.match(tt.name)) {
				return undefined;
			}

			expectTSEnabled(parser, "module or namespace declaration", start);
			return parseTSModuleOrNamespaceDeclaration(parser, start);
		}

		// TODO abstract this into typescript.js
		case "global":
			// `global { }` (with no `declare`) may appear inside an ambient module declaration.
			// Would like to use parseTSAmbientExternalModuleDeclaration here, but already ran past 'global'.
			if (parser.match(tt.braceL)) {
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

	parser.pushScope("TYPE", true);

	const params = tsInNoContext(
		parser,
		() => {
			parser.expectRelational("<");
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
	parser.popScope("TYPE");
	parser.expectRelational(">");

	return parser.finishNode(
		start,
		{
			type: "TSTypeParameterInstantiation",
			params,
		},
	);
}

export function isTSDeclarationStart(parser: JSParser): boolean {
	if (parser.match(tt.name)) {
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
		parser.isContextual("abstract") &&
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
	if (!parser.isSyntaxEnabled("ts")) {
		return undefined;
	}

	if (parser.match(tt._import)) {
		// `export const A =B;`
		parser.expect(tt._import);
		return parseTSImportEqualsDeclaration(parser, start, /* isExport */ true);
	}

	if (parser.eat(tt.eq)) {
		// `export = x;`
		const expression = parseExpression(parser, "ts export assignment");
		parser.semicolon();
		return parser.finishNode(
			start,
			{
				type: "TSExportAssignment",
				expression,
			},
		);
	}

	if (parser.eatContextual("as")) {
		// `export as namespace A;`
		// See `parseNamespaceExportDeclaration` in TypeScript's own parser
		parser.expectContextual("namespace");
		const id = parseIdentifier(parser);
		parser.semicolon();
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
