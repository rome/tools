import {CSSQualifiedName, NodeBaseWithComments} from "@internal/ast";
import {AnyCSSValue} from "../unions";
import {createBuilder} from "../../utils";

export interface CSSAttributeSelector extends NodeBaseWithComments {
	readonly type: "CSSAttributeSelector";
	readonly attribute: CSSQualifiedName;
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
