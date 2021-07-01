import {JSONValue} from "@internal/codec-config";
import {Consumer} from "@internal/consume";
import {AsyncCallback, AsyncVoidCallback} from "@internal/typescript-helpers";
import LSPServer from "../LSPServer";

export type LSPNotificationHandler = AsyncVoidCallback<[
	LSPServer,
	Consumer,
	string
]>;

export type LSPRequestHandler = AsyncCallback<
	[LSPServer, Consumer, string],
	JSONValue
>;
