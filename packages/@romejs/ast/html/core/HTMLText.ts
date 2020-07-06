import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type HTMLText = NodeBaseWithComments & {
	type: "HTMLText";
	value: string;
};

export const htmlText = createBuilder<HTMLText>(
	"HTMLText",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
