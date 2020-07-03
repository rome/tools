import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// class="something"
export type HTMLAttribute = NodeBaseWithComments & {
	type: "HTMLAttribute";
	value: string;
};

export const htmlAttribute = createBuilder<HTMLAttribute>(
	"HTMLAttribute",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
