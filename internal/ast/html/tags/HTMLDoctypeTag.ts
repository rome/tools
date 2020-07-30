import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface HTMLDoctypeTag extends NodeBaseWithComments {
	readonly type: "HTMLDoctypeTag";
	readonly value: string;
}

export const htmlDoctypeTag = createBuilder<HTMLDoctypeTag>(
	"HTMLDoctypeTag",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
