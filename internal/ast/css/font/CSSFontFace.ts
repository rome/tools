import {CSSBlock, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSFontFace extends NodeBaseWithComments {
	readonly type: "CSSFontFace";
	readonly value: CSSBlock;
}

export const cssFontFace = createBuilder<CSSFontFace>(
	"CSSFontFace",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
