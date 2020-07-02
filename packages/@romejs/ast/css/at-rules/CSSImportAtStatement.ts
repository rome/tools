import {CSSStringType, CSSURLType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// @import url("fineprint.css") print;
// @import url("bluish.css") speech;
// @import 'custom.css';
// @import url("chrome://communicator/skin/");
// @import "common.css" screen;
// @import url('landscape.css') screen and (orientation:landscape);
export type CSSImportAtStatement = NodeBaseWithComments & {
	type: "CSSImportAtStatement";
	url: CSSURLType | CSSStringType;
	// TODO media queries
	// TODO supports query
};

export const cssImportAtStatement = createBuilder<CSSImportAtStatement>(
	"CSSImportAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			url: true,
		},
	},
);
