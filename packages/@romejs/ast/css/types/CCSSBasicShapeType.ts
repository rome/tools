import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CCSSBasicShapeType = JSNodeBase & {
	type: "CCSSBasicShapeType";
};

export const ccssBasicShapeType = createBuilder<CCSSBasicShapeType>(
	"CCSSBasicShapeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
