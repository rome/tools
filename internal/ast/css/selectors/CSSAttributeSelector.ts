import {CSSIdentifier, NodeBaseWithComments} from "@internal/ast";
import {AnyCSSValue} from "@internal/css-parser/types";
import {createBuilder} from "../../utils";

export interface CSSAttributeSelector extends NodeBaseWithComments {
	readonly type: "CSSAttributeSelector";
	readonly attribute: CSSIdentifier;
	readonly value?: AnyCSSValue;
	readonly matcher?: AttributeMatcher;
	readonly modifier?: AttributeModifier;
}

export type AttributeMatcher = "~=" | "|=" | "^=" | "$=" | "*=" | "=";

export type AttributeModifier = "i" | "s";

export const cssAttributeSelector = createBuilder<CSSAttributeSelector>(
	"CSSAttributeSelector",
	{
		bindingKeys: {},
		visitorKeys: {
			attribute: true,
			value: true,
		},
	},
);
