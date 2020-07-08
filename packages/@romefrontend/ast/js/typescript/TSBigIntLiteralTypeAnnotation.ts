import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type TSBigIntLiteralTypeAnnotation = NodeBaseWithComments & {
	type: "TSBigIntLiteralTypeAnnotation";
	value: string;
};

export const tsBigIntLiteralTypeAnnotation = createBuilder<TSBigIntLiteralTypeAnnotation>(
	"TSBigIntLiteralTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
