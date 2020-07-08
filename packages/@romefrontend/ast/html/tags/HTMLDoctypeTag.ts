import {NodeBaseWithComments} from "@romefrontend/ast";
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
