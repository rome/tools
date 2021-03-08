import {CSSMediaFeatureBoolean, CSSMediaFeaturePlain, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export type ValidFeatures =
	| "width"
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
	| "grid";

export interface CSSMediaFeature extends NodeBaseWithComments {
	readonly type: "CSSMediaFeature";
	// TODO: this is going to be funny
	readonly value: CSSMediaFeaturePlain | CSSMediaFeatureBoolean;
}

export const cssMediaFeature = createBuilder<CSSMediaFeature>(
	"CSSMediaFeature",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true
		},
	},
);
