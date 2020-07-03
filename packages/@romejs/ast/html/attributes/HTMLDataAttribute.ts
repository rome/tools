import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// data-open
export type HTMLDataAttribute = NodeBaseWithComments & {
	type: "HTMLDataAttribute";
	value: string;
};

export const htmlDataAttribute = createBuilder<HTMLDataAttribute>(
	"HTMLDataAttribute",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
