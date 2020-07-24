import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSBasicShapeType extends NodeBaseWithComments {
	type: "CSSBasicShapeType";
	// TODO
}

export const ccssBasicShapeType = createBuilder<CSSBasicShapeType>(
	"CSSBasicShapeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
