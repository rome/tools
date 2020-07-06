import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type HTMLDoctypeTag = NodeBaseWithComments & {
	type: "HTMLDoctypeTag";
	value: string;
};

export const htmlDoctypeTag = createBuilder<HTMLDoctypeTag>(
	"HTMLDoctypeTag",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
