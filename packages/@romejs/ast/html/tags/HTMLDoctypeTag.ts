import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";
import {AnyHTMLAttribute} from "@romejs/ast/html/unions";

export type HTMLDoctypeTag = NodeBaseWithComments & {
	type: "HTMLDoctypeTag";
	attributes: Array<AnyHTMLAttribute>;
};

export const htmlDoctypeTag = createBuilder<HTMLDoctypeTag>(
	"HTMLDoctypeTag",
	{
		bindingKeys: {},
		visitorKeys: {
			attributes: true,
		},
	},
);
