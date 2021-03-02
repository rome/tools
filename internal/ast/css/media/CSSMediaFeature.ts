import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeature extends NodeBaseWithComments {
	readonly type: "CSSMediaFeature";
	// TODO: this is going to be funny
	readonly value: "ahahah"
}

export const cssMediaFeature = createBuilder<CSSMediaFeature>(
	"CSSMediaFeature",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
