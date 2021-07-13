import {LSPNotificationHandler, LSPRequestHandler} from "./types";
import * as general from "./general";
import * as textDocument from "./textDocument";
import * as workspace from "./workspace";

export const notificationHandlers = new Map<string, LSPNotificationHandler>();
export const requestHandlers = new Map<string, LSPRequestHandler>();

requestHandlers.set("initialize", general.initialize);
requestHandlers.set("shutdown", general.shutdown);
requestHandlers.set("textDocument/formatting", textDocument.formatting);
requestHandlers.set("textDocument/codeAction", textDocument.codeAction);
requestHandlers.set("workspace/executeCommand", workspace.executeCommand);

notificationHandlers.set("exit", general.exit);
notificationHandlers.set("initialized", general.initialized);
notificationHandlers.set("textDocument/didOpen", textDocument.didOpen);
notificationHandlers.set("textDocument/didChange", textDocument.didChange);
notificationHandlers.set("textDocument/didClose", textDocument.didClose);
notificationHandlers.set(
	"workspace/didChangeWorkspaceFolders",
	workspace.didChangeWorkspaceFolders,
);
