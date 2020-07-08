import {
	AnyHTMLChildNode,
	HTMLAttribute,
	HTMLIdentifier,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type HTMLElement = NodeBaseWithComments & {
	type: "HTMLElement";
	name: HTMLIdentifier;
	selfClosing?: boolean;
	attributes: Array<HTMLAttribute>;
	children: Array<AnyHTMLChildNode>;
};

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
