import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export type ValidFeatures = "width"
| "height"
| "device-width"
| "device-height"
| "orientation"
| "aspect-ratio"
| "device-aspect-ratio"
| "color"
| "color-index"
| "monochrome"
| "resolution"
| "scan"
| "grid"

export interface CSSMediaType extends NodeBaseWithComments {
	readonly type: "CSSMediaType";
	readonly value: ValidFeatures
}

export const cssMediaType = createBuilder<CSSMediaType>(
	"CSSMediaType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
