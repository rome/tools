import {createBuilder} from "@romefrontend/ast/utils";
import {NodeBaseWithComments, RootBase} from "..";

export type CommitRoot = NodeBaseWithComments &
	RootBase & {
		type: "CommitRoot";
		breaking: boolean;
		commitType: string;
		custom: boolean;
		rawBody: string;
		scope: string;
	};

export const commitRoot = createBuilder<CommitRoot>(
	"CommitRoot",
	{
		bindingKeys: {},
		visitorKeys: {
			comments: true,
		},
	},
);
