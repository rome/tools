import {
	AnyHTMLChildNode,
	HTMLAttribute,
	HTMLIdentifier,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface HTMLElement extends NodeBaseWithComments {
	readonly type: "HTMLElement";
	readonly name: HTMLIdentifier;
	readonly selfClosing?: boolean;
	readonly attributes: Array<HTMLAttribute>;
	readonly children: Array<AnyHTMLChildNode>;
}

export const htmlElement = createBuilder<HTMLElement>(
	"HTMLElement",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
			attributes: true,
			children: true,
		},
	},
);
