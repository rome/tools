import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSTimeType = JSNodeBase & {
	type: "CSSTimeType";
};

export const cssTimeType = createBuilder<CSSTimeType>(
	"CSSTimeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
