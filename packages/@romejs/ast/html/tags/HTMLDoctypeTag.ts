import {HTMLAttribute, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type HTMLDoctypeTag = NodeBaseWithComments & {
	type: "HTMLDoctypeTag";
	attributes: Array<HTMLAttribute>;
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
