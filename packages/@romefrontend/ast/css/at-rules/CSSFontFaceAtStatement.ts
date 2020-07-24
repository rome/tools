import {CSSRuleDeclaration, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// @font-face
export interface CSSFontFaceAtStatement extends NodeBaseWithComments {
	type: "CSSFontFaceAtStatement";
	declarations: Array<CSSRuleDeclaration>;
	// TODO should we have a field for each known valid @font-face property?
}

export const cssFontFaceAtStatement = createBuilder<CSSFontFaceAtStatement>(
	"CSSFontFaceAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			declarations: true,
		},
	},
);
