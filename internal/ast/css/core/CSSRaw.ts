import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSRaw extends NodeBaseWithComments {
	readonly type: "CSSRaw";
	readonly value: string;
}
export const cssRaw = createBuilder<CSSRaw>(
	"CSSRaw",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
