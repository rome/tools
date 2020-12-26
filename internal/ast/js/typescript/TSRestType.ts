import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyTSPrimary} from "../unions";

export interface TSRestType extends NodeBaseWithComments {
	readonly type: "TSRestType";
	readonly argument: AnyTSPrimary;
}

export const tsRestType = createBuilder<TSRestType>(
	"TSRestType",
	{
		bindingKeys: {},
		visitorKeys: {
			argument: true,
		},
	},
);
