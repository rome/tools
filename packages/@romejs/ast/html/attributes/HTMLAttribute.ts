import {HTMLIdentifier, HTMLString, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

// class="something"
export type HTMLAttribute = NodeBaseWithComments & {
	type: "HTMLAttribute";
	name: HTMLIdentifier;
	value: HTMLString;
};

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
