import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSBigIntLiteralTypeAnnotation extends NodeBaseWithComments {
	readonly type: "TSBigIntLiteralTypeAnnotation";
	readonly value: string;
}

export const tsBigIntLiteralTypeAnnotation = createBuilder<TSBigIntLiteralTypeAnnotation>(
	"TSBigIntLiteralTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
