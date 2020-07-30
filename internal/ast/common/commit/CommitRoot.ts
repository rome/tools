import {createBuilder} from "@internal/ast/utils";
import {NodeBaseWithComments, RootBase} from "../../index";

export interface CommitRoot extends NodeBaseWithComments,
RootBase {
	readonly type: "CommitRoot";
	readonly breaking: boolean;
	readonly commitType: string;
	readonly custom: boolean;
	readonly rawBody: string;
	readonly scope: string;
}

export const commitRoot = createBuilder<CommitRoot>(
	"CommitRoot",
	{
		bindingKeys: {},
		visitorKeys: {
			comments: true,
		},
	},
);
