import {
	AnyTSPrimary,
	JSBindingIdentifier,
	NodeBaseWithComments,
} from "../../index";
import {createBuilder} from "../../utils";

export interface TSTupleElement extends NodeBaseWithComments {
	readonly type: "TSTupleElement";
	readonly name?: JSBindingIdentifier;
	readonly optional?: boolean;
	readonly typeAnnotation: AnyTSPrimary;
}

export const tsTupleElement = createBuilder<TSTupleElement>(
	"TSTupleElement",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
			typeAnnotation: true,
		},
	},
);
