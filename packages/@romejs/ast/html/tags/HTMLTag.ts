import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";
import {AnyHTMLAttribute} from "@romejs/ast/html/unions";

export type HTMLTag = NodeBaseWithComments & {
	type: "HTMLTag";
	attributes: Array<AnyHTMLAttribute>;
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
