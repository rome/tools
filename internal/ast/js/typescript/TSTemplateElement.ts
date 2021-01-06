import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTemplateElement extends NodeBaseWithComments {
	readonly type: "TSTemplateElement";
	readonly tail?: boolean;
	readonly cooked: string;
	readonly raw: string;
}

export const tsTemplateElement = createBuilder<TSTemplateElement>(
	"TSTemplateElement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
