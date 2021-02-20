import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export type Combinator =
	| "descendant"
	| "child"
	| "nextSibling"
	| "subsequentSibling";

export interface CSSCombinator extends NodeBaseWithComments {
	readonly type: "CSSCombinator";
	readonly combinator: Combinator;
}

export const cssCombinator = createBuilder<CSSCombinator>(
	"CSSCombinator",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
