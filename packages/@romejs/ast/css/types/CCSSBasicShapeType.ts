import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CCSSBasicShapeType = NodeBaseWithComments & {
	type: "CCSSBasicShapeType";
};

export const ccssBasicShapeType = createBuilder<CCSSBasicShapeType>(
	"CCSSBasicShapeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
