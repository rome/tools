import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSWhitespace extends NodeBaseWithComments {
	readonly type: "CSSWhitespace";
}

export const cssWhitespace = createBuilder<CSSWhitespace>(
	"CSSWhitespace",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
