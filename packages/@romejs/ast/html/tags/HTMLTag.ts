import {HTMLAttribute, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type HTMLTag = NodeBaseWithComments & {
	type: "HTMLTag";
	attributes: Array<HTMLAttribute>;
	kind: "self-closing" | "open";
	childNodes: Array<HTMLTag>;
};

export const htmlTag = createBuilder<HTMLTag>(
	"HTMLTag",
	{
		bindingKeys: {},
		visitorKeys: {
			attributes: true,
			childNodes: true,
		},
	},
);
