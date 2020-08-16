/**
 * Portions Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// The algorithm used to determine whether a regexp can appear at a
// given point in the program is loosely based on sweet.js' approach.
// See https://github.com/mozilla/sweet.js/wiki/design
import {JSParser, inScope} from "../parser";
import {getCurContext, isBraceBlock, readTemplateToken} from "./index";
import {lineBreak} from "@internal/js-parser-utils";
import {types as tt} from "./types";
import {Dict} from "@internal/typescript-helpers";

type TokContextOverride = (p: JSParser) => void;

export class TokContext {
	constructor(
		token: string,
		isExpr?: boolean,
		preserveSpace?: boolean,
		override?: TokContextOverride,
	) {
		this.token = token;
		this.isExpr = !!isExpr;
		this.preserveSpace = !!preserveSpace;
		this.override = override;
	}

	public token: string;
	public isExpr: boolean;
	public preserveSpace: boolean;
	public override: undefined | TokContextOverride;
}

export const types: Dict<TokContext> = {
	braceStatement: new TokContext("{", false),
	braceExpression: new TokContext("{", true),
	templateQuasi: new TokContext("${", false),
	parenStatement: new TokContext("(", false),
	parenExpression: new TokContext("(", true),
	template: new TokContext("`", true, true, (p) => readTemplateToken(p)),
	functionExpression: new TokContext("function", true),
	functionStatement: new TokContext("function", false),
	// JSX
	jsxOpenTag: new TokContext("<tag", false),
	jsxCloseTag: new TokContext("</tag", false),
	jsxInner: new TokContext("<tag>...</tag>", true, true),
};

// Token-specific context update code
tt.parenR.updateContext = tt.braceR.updateContext = function(parser) {
	if (parser.state.context.length === 1) {
		parser.state.exprAllowed = true;
		return;
	}

	let out = parser.state.context.pop();
	if (out === types.braceStatement) {
		const context = getCurContext(parser);
		if (context !== undefined && context.token === "function") {
			out = parser.state.context.pop();
		}
	}

	if (out === undefined) {
		throw new Error("No context found");
	}

	parser.state.exprAllowed = !out.isExpr;
};

tt.name.updateContext = function(parser, prevType) {
	let allowed = false;
	if (prevType !== tt.dot) {
		if (
			(parser.state.tokenValue === "of" && !parser.state.exprAllowed) ||
			(parser.state.tokenValue === "yield" && inScope(parser, "GENERATOR"))
		) {
			allowed = true;
		}
	}

	parser.state.exprAllowed = allowed;

	if (parser.state.isIterator) {
		parser.state.isIterator = false;
	}
};

tt.braceL.updateContext = function(parser, prevType) {
	parser.state.context.push(
		isBraceBlock(parser, prevType)
			? types.braceStatement
			: types.braceExpression,
	);
	parser.state.exprAllowed = true;
};

tt.dollarBraceL.updateContext = function(parser) {
	parser.state.context.push(types.templateQuasi);
	parser.state.exprAllowed = true;
};

tt.parenL.updateContext = function(parser, prevType) {
	const statementParens =
		prevType === tt._if ||
		prevType === tt._for ||
		prevType === tt._with ||
		prevType === tt._while;
	parser.state.context.push(
		statementParens ? types.parenStatement : types.parenExpression,
	);
	parser.state.exprAllowed = true;
};

tt.incDec.updateContext = function() {
	// tokExprAllowed stays unchanged
};

tt._function.updateContext = function(parser, prevType) {
	if (
		prevType.beforeExpr &&
		prevType !== tt.semi &&
		prevType !== tt._else &&
		!(prevType === tt._return &&
		lineBreak.test(
			parser.getRawInput(parser.state.lastEndIndex, parser.state.startIndex),
		)) &&
		!((prevType === tt.colon || prevType === tt.braceL) &&
		getCurContext(parser) === types.bStat)
	) {
		parser.state.context.push(types.functionExpression);
	} else {
		parser.state.context.push(types.functionStatement);
	}

	parser.state.exprAllowed = false;
};

tt._class.updateContext = tt._function.updateContext;

tt.backQuote.updateContext = function(parser) {
	if (getCurContext(parser) === types.template) {
		parser.state.context.pop();
	} else {
		parser.state.context.push(types.template);
	}
	parser.state.exprAllowed = false;
};

tt.jsxTagStart.updateContext = function(parser) {
	parser.state.context.push(types.jsxInner); // treat as beginning of JSX expression
	parser.state.context.push(types.jsxOpenTag); // start opening tag context
	parser.state.exprAllowed = false;
};

tt.jsxTagEnd.updateContext = function(parser, prevType) {
	const out = parser.state.context.pop();
	if (
		(out === types.jsxOpenTag && prevType === tt.slash) ||
		out === types.jsxCloseTag
	) {
		parser.state.context.pop();
		parser.state.exprAllowed = getCurContext(parser) === types.jsxInner;
	} else {
		parser.state.exprAllowed = true;
	}
};
