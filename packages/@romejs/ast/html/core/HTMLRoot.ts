import {NodeBaseWithComments, RootBase} from "../../index";
import {createBuilder} from "../../utils";
import {HTMLAnyNode} from "@romejs/ast/html/unions";

export type HTMLRoot = NodeBaseWithComments &
	RootBase & {
		type: "HTMLRoot";
		body: Array<HTMLAnyNode>;
	};

export const htmlRoot = createBuilder<HTMLRoot>(
	"HTMLRoot",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
			comments: true,
		},
	},
);
