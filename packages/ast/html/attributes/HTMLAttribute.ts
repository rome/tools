import {
	HTMLIdentifier,
	HTMLString,
	NodeBaseWithComments,
} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

// class="something"
export interface HTMLAttribute extends NodeBaseWithComments {
	readonly type: "HTMLAttribute";
	readonly name: HTMLIdentifier;
	readonly value: HTMLString;
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
