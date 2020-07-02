import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSFontFaceAtStatement = JSNodeBase & {
	type: "CSSFontFaceAtStatement";
};

export const cssFontFaceAtStatement = createBuilder<CSSFontFaceAtStatement>(
	"CSSFontFaceAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
