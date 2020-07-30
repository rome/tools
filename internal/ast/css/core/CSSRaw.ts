import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";

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
