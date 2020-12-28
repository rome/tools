import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface HTMLCdataTag extends NodeBaseWithComments {
	readonly type: "HTMLCdataTag";
	readonly value?: string;
}

export const htmlCdataTag = createBuilder<HTMLCdataTag>(
	"HTMLCdataTag",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
