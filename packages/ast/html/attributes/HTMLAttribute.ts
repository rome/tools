import {
	HTMLIdentifier,
	HTMLString,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

// class="something"
export interface HTMLAttribute extends NodeBaseWithComments {
	type: "HTMLAttribute";
	name: HTMLIdentifier;
	value: HTMLString;
}

export const htmlAttribute = createBuilder<HTMLAttribute>(
	"HTMLAttribute",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
			value: true,
		},
	},
);
