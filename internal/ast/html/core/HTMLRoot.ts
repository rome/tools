import {NodeBaseWithComments, RootBase} from "../../index";
import {createBuilder} from "../../utils";
import {AnyHTMLChildNode} from "@internal/ast";

export interface HTMLRoot extends NodeBaseWithComments,
RootBase {
	readonly type: "HTMLRoot";
	readonly body: Array<AnyHTMLChildNode>;
}

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
