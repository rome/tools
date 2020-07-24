import {
	AnyTSPrimary,
	JSBindingIdentifier,
	NodeBaseWithComments,
} from "../../index";
import {createBuilder} from "../../utils";

export interface TSTupleElement extends NodeBaseWithComments {
	type: "TSTupleElement";
	name?: JSBindingIdentifier;
	optional?: boolean;
	typeAnnotation: AnyTSPrimary;
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
