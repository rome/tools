import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSBasicShapeType = NodeBaseWithComments & {
	type: "CSSBasicShapeType";
	// TODO
};

export const ccssBasicShapeType = createBuilder<CSSBasicShapeType>(
	"CSSBasicShapeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
