import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// <!-- -->
export type HTMLCommentBlock = NodeBaseWithComments & {
	type: "HTMLCommentBlock";
};

export const htmlCommentBlock = createBuilder<HTMLCommentBlock>(
	"HTMLCommentBlock",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
