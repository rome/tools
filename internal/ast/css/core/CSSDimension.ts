import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSDimension extends NodeBaseWithComments {
	readonly type: "CSSDimension";
	readonly value: number;
	readonly unit: string;
}
export const cssDimension = createBuilder<CSSDimension>(
	"CSSDimension",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
