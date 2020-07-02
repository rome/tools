import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSFontFaceAtStatement = NodeBaseWithComments & {
	type: "CSSFontFaceAtStatement";
};

export const cssFontFaceAtStatement = createBuilder<CSSFontFaceAtStatement>(
	"CSSFontFaceAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
