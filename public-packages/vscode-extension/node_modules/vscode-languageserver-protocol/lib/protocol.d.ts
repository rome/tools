import { RequestType, RequestType0, NotificationType, NotificationType0 } from 'vscode-jsonrpc';
import { TextDocumentContentChangeEvent, Position, Range, Location, LocationLink, Diagnostic, Command, TextEdit, WorkspaceEdit, WorkspaceSymbolParams, TextDocumentIdentifier, VersionedTextDocumentIdentifier, TextDocumentItem, TextDocumentSaveReason, CompletionItem, CompletionList, Hover, SignatureHelp, ReferenceContext, DocumentHighlight, DocumentSymbolParams, SymbolInformation, CodeLens, CodeActionContext, FormattingOptions, DocumentLink, MarkupKind, SymbolKind, CompletionItemKind, CodeAction, CodeActionKind, DocumentSymbol } from 'vscode-languageserver-types';
import { ImplementationRequest, ImplementationClientCapabilities, ImplementationServerCapabilities } from './protocol.implementation';
import { TypeDefinitionRequest, TypeDefinitionClientCapabilities, TypeDefinitionServerCapabilities } from './protocol.typeDefinition';
import { WorkspaceFoldersRequest, DidChangeWorkspaceFoldersNotification, DidChangeWorkspaceFoldersParams, WorkspaceFolder, WorkspaceFoldersChangeEvent, WorkspaceFoldersInitializeParams, WorkspaceFoldersClientCapabilities, WorkspaceFoldersServerCapabilities } from './protocol.workspaceFolders';
import { ConfigurationRequest, ConfigurationParams, ConfigurationItem, ConfigurationClientCapabilities } from './protocol.configuration';
import { DocumentColorRequest, ColorPresentationRequest, ColorProviderOptions, DocumentColorParams, ColorPresentationParams, ColorServerCapabilities, ColorClientCapabilities } from './protocol.colorProvider';
import { FoldingRangeClientCapabilities, FoldingRangeProviderOptions, FoldingRangeRequest, FoldingRangeParams, FoldingRangeServerCapabilities } from './protocol.foldingRange';
import { DeclarationClientCapabilities, DeclarationRequest, DeclarationServerCapabilities } from './protocol.declaration';
/**
 * A document filter denotes a document by different properties like
 * the [language](#TextDocument.languageId), the [scheme](#Uri.scheme) of
 * its resource, or a glob-pattern that is applied to the [path](#TextDocument.fileName).
 *
 * @sample A language filter that applies to typescript files on disk: `{ language: 'typescript', scheme: 'file' }`
 * @sample A language filter that applies to all package.json paths: `{ language: 'json', pattern: '**package.json' }`
 */
export declare type DocumentFilter = {
    /** A language id, like `typescript`. */
    language: string;
    /** A Uri [scheme](#Uri.scheme), like `file` or `untitled`. */
    scheme?: string;
    /** A glob pattern, like `*.{ts,js}`. */
    pattern?: string;
} | {
    /** A language id, like `typescript`. */
    language?: string;
    /** A Uri [scheme](#Uri.scheme), like `file` or `untitled`. */
    scheme: string;
    /** A glob pattern, like `*.{ts,js}`. */
    pattern?: string;
} | {
    /** A language id, like `typescript`. */
    language?: string;
    /** A Uri [scheme](#Uri.scheme), like `file` or `untitled`. */
    scheme?: string;
    /** A glob pattern, like `*.{ts,js}`. */
    pattern: string;
};
export declare namespace DocumentFilter {
    function is(value: any): value is DocumentFilter;
}
/**
 * A document selector is the combination of one or many document filters.
 *
 * @sample `let sel:DocumentSelector = [{ language: 'typescript' }, { language: 'json', pattern: '**∕tsconfig.json' }]`;
 */
export declare type DocumentSelector = (string | DocumentFilter)[];
/**
 * General parameters to to register for an notification or to register a provider.
 */
export interface Registration {
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again.
     */
    id: string;
    /**
     * The method to register for.
     */
    method: string;
    /**
     * Options necessary for the registration.
     */
    registerOptions?: any;
}
export interface RegistrationParams {
    registrations: Registration[];
}
/**
 * The `client/registerCapability` request is sent from the server to the client to register a new capability
 * handler on the client side.
 */
export declare namespace RegistrationRequest {
    const type: RequestType<RegistrationParams, void, void, void>;
}
/**
 * General parameters to unregister a request or notification.
 */
export interface Unregistration {
    /**
     * The id used to unregister the request or notification. Usually an id
     * provided during the register request.
     */
    id: string;
    /**
     * The method to unregister for.
     */
    method: string;
}
export interface UnregistrationParams {
    unregisterations: Unregistration[];
}
/**
 * The `client/unregisterCapability` request is sent from the server to the client to unregister a previously registered capability
 * handler on the client side.
 */
export declare namespace UnregistrationRequest {
    const type: RequestType<UnregistrationParams, void, void, void>;
}
/**
 * A parameter literal used in requests to pass a text document and a position inside that
 * document.
 */
export interface TextDocumentPositionParams {
    /**
     * The text document.
     */
    textDocument: TextDocumentIdentifier;
    /**
     * The position inside the text document.
     */
    position: Position;
}
/**
 * The kind of resource operations supported by the client.
 */
export declare type ResourceOperationKind = 'create' | 'rename' | 'delete';
export declare namespace ResourceOperationKind {
    /**
     * Supports creating new files and folders.
     */
    const Create: ResourceOperationKind;
    /**
     * Supports renaming existing files and folders.
     */
    const Rename: ResourceOperationKind;
    /**
     * Supports deleting existing files and folders.
     */
    const Delete: ResourceOperationKind;
}
export declare type FailureHandlingKind = 'abort' | 'transactional' | 'undo' | 'textOnlyTransactional';
export declare namespace FailureHandlingKind {
    /**
     * Applying the workspace change is simply aborted if one of the changes provided
     * fails. All operations executed before the failing operation stay executed.
     */
    const Abort: FailureHandlingKind;
    /**
     * All operations are executed transactional. That means they either all
     * succeed or no changes at all are applied to the workspace.
     */
    const Transactional: FailureHandlingKind;
    /**
     * If the workspace edit contains only textual file changes they are executed transactional.
     * If resource changes (create, rename or delete file) are part of the change the failure
     * handling startegy is abort.
     */
    const TextOnlyTransactional: FailureHandlingKind;
    /**
     * The client tries to undo the operations already executed. But there is no
     * guaruntee that this is succeeding.
     */
    const Undo: FailureHandlingKind;
}
/**
 * Workspace specific client capabilities.
 */
export interface WorkspaceClientCapabilities {
    /**
     * The client supports applying batch edits
     * to the workspace by supporting the request
     * 'workspace/applyEdit'
     */
    applyEdit?: boolean;
    /**
     * Capabilities specific to `WorkspaceEdit`s
     */
    workspaceEdit?: {
        /**
         * The client supports versioned document changes in `WorkspaceEdit`s
         */
        documentChanges?: boolean;
        /**
         * The resource operations the client supports. Clients should at least
         * support 'create', 'rename' and 'delete' files and folders.
         */
        resourceOperations?: ResourceOperationKind[];
        /**
         * The failure handling strategy of a client if applying the workspace edit
         * failes.
         */
        failureHandling?: FailureHandlingKind;
    };
    /**
     * Capabilities specific to the `workspace/didChangeConfiguration` notification.
     */
    didChangeConfiguration?: {
        /**
         * Did change configuration notification supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
    /**
     * Capabilities specific to the `workspace/didChangeWatchedFiles` notification.
     */
    didChangeWatchedFiles?: {
        /**
         * Did change watched files notification supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
    /**
     * Capabilities specific to the `workspace/symbol` request.
     */
    symbol?: {
        /**
         * Symbol request supports dynamic registration.
         */
        dynamicRegistration?: boolean;
        /**
         * Specific capabilities for the `SymbolKind` in the `workspace/symbol` request.
         */
        symbolKind?: {
            /**
             * The symbol kind values the client supports. When this
             * property exists the client also guarantees that it will
             * handle values outside its set gracefully and falls back
             * to a default value when unknown.
             *
             * If this property is not present the client only supports
             * the symbol kinds from `File` to `Array` as defined in
             * the initial version of the protocol.
             */
            valueSet?: SymbolKind[];
        };
    };
    /**
     * Capabilities specific to the `workspace/executeCommand` request.
     */
    executeCommand?: {
        /**
         * Execute command supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
}
/**
 * Text document specific client capabilities.
 */
export interface TextDocumentClientCapabilities {
    /**
     * Defines which synchronization capabilities the client supports.
     */
    synchronization?: {
        /**
         * Whether text document synchronization supports dynamic registration.
         */
        dynamicRegistration?: boolean;
        /**
         * The client supports sending will save notifications.
         */
        willSave?: boolean;
        /**
         * The client supports sending a will save request and
         * waits for a response providing text edits which will
         * be applied to the document before it is saved.
         */
        willSaveWaitUntil?: boolean;
        /**
         * The client supports did save notifications.
         */
        didSave?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/completion`
     */
    completion?: {
        /**
         * Whether completion supports dynamic registration.
         */
        dynamicRegistration?: boolean;
        /**
         * The client supports the following `CompletionItem` specific
         * capabilities.
         */
        completionItem?: {
            /**
             * Client supports snippets as insert text.
             *
             * A snippet can define tab stops and placeholders with `$1`, `$2`
             * and `${3:foo}`. `$0` defines the final tab stop, it defaults to
             * the end of the snippet. Placeholders with equal identifiers are linked,
             * that is typing in one will update others too.
             */
            snippetSupport?: boolean;
            /**
             * Client supports commit characters on a completion item.
             */
            commitCharactersSupport?: boolean;
            /**
             * Client supports the follow content formats for the documentation
             * property. The order describes the preferred format of the client.
             */
            documentationFormat?: MarkupKind[];
            /**
             * Client supports the deprecated property on a completion item.
             */
            deprecatedSupport?: boolean;
            /**
             * Client supports the preselect property on a completion item.
             */
            preselectSupport?: boolean;
        };
        completionItemKind?: {
            /**
             * The completion item kind values the client supports. When this
             * property exists the client also guarantees that it will
             * handle values outside its set gracefully and falls back
             * to a default value when unknown.
             *
             * If this property is not present the client only supports
             * the completion items kinds from `Text` to `Reference` as defined in
             * the initial version of the protocol.
             */
            valueSet?: CompletionItemKind[];
        };
        /**
         * The client supports to send additional context information for a
         * `textDocument/completion` requestion.
         */
        contextSupport?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/hover`
     */
    hover?: {
        /**
         * Whether hover supports dynamic registration.
         */
        dynamicRegistration?: boolean;
        /**
         * Client supports the follow content formats for the content
         * property. The order describes the preferred format of the client.
         */
        contentFormat?: MarkupKind[];
    };
    /**
     * Capabilities specific to the `textDocument/signatureHelp`
     */
    signatureHelp?: {
        /**
         * Whether signature help supports dynamic registration.
         */
        dynamicRegistration?: boolean;
        /**
         * The client supports the following `SignatureInformation`
         * specific properties.
         */
        signatureInformation?: {
            /**
             * Client supports the follow content formats for the documentation
             * property. The order describes the preferred format of the client.
             */
            documentationFormat?: MarkupKind[];
            /**
             * Client capabilities specific to parameter information.
             */
            parameterInformation?: {
                /**
                 * The client supports processing label offsets instead of a
                 * simple label string.
                 */
                labelOffsetSupport?: boolean;
            };
        };
    };
    /**
     * Capabilities specific to the `textDocument/references`
     */
    references?: {
        /**
         * Whether references supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/documentHighlight`
     */
    documentHighlight?: {
        /**
         * Whether document highlight supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/documentSymbol`
     */
    documentSymbol?: {
        /**
         * Whether document symbol supports dynamic registration.
         */
        dynamicRegistration?: boolean;
        /**
         * Specific capabilities for the `SymbolKind`.
         */
        symbolKind?: {
            /**
             * The symbol kind values the client supports. When this
             * property exists the client also guarantees that it will
             * handle values outside its set gracefully and falls back
             * to a default value when unknown.
             *
             * If this property is not present the client only supports
             * the symbol kinds from `File` to `Array` as defined in
             * the initial version of the protocol.
             */
            valueSet?: SymbolKind[];
        };
        /**
         * The client support hierarchical document symbols.
         */
        hierarchicalDocumentSymbolSupport?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/formatting`
     */
    formatting?: {
        /**
         * Whether formatting supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/rangeFormatting`
     */
    rangeFormatting?: {
        /**
         * Whether range formatting supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/onTypeFormatting`
     */
    onTypeFormatting?: {
        /**
         * Whether on type formatting supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/definition`
     */
    definition?: {
        /**
         * Whether definition supports dynamic registration.
         */
        dynamicRegistration?: boolean;
        /**
         * The client supports additional metadata in the form of definition links.
         */
        linkSupport?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/codeAction`
     */
    codeAction?: {
        /**
         * Whether code action supports dynamic registration.
         */
        dynamicRegistration?: boolean;
        /**
         * The client support code action literals as a valid
         * response of the `textDocument/codeAction` request.
         */
        codeActionLiteralSupport?: {
            /**
             * The code action kind is support with the following value
             * set.
             */
            codeActionKind: {
                /**
                 * The code action kind values the client supports. When this
                 * property exists the client also guarantees that it will
                 * handle values outside its set gracefully and falls back
                 * to a default value when unknown.
                 */
                valueSet: CodeActionKind[];
            };
        };
    };
    /**
     * Capabilities specific to the `textDocument/codeLens`
     */
    codeLens?: {
        /**
         * Whether code lens supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/documentLink`
     */
    documentLink?: {
        /**
         * Whether document link supports dynamic registration.
         */
        dynamicRegistration?: boolean;
    };
    /**
     * Capabilities specific to the `textDocument/rename`
     */
    rename?: {
        /**
         * Whether rename supports dynamic registration.
         */
        dynamicRegistration?: boolean;
        /**
         * Client supports testing for validity of rename operations
         * before execution.
         */
        prepareSupport?: boolean;
    };
    /**
     * Capabilities specific to `textDocument/publishDiagnostics`.
     */
    publishDiagnostics?: {
        /**
         * Whether the clients accepts diagnostics with related information.
         */
        relatedInformation?: boolean;
    };
}
/**
 * Defines the capabilities provided by the client.
 */
export interface _ClientCapabilities {
    /**
     * Workspace specific client capabilities.
     */
    workspace?: WorkspaceClientCapabilities;
    /**
     * Text document specific client capabilities.
     */
    textDocument?: TextDocumentClientCapabilities;
    /**
     * Experimental client capabilities.
     */
    experimental?: any;
}
export declare type ClientCapabilities = _ClientCapabilities & ImplementationClientCapabilities & TypeDefinitionClientCapabilities & WorkspaceFoldersClientCapabilities & ConfigurationClientCapabilities & ColorClientCapabilities & FoldingRangeClientCapabilities & DeclarationClientCapabilities;
/**
 * Defines how the host (editor) should sync
 * document changes to the language server.
 */
export declare namespace TextDocumentSyncKind {
    /**
     * Documents should not be synced at all.
     */
    const None = 0;
    /**
     * Documents are synced by always sending the full content
     * of the document.
     */
    const Full = 1;
    /**
     * Documents are synced by sending the full content on open.
     * After that only incremental updates to the document are
     * send.
     */
    const Incremental = 2;
}
export declare type TextDocumentSyncKind = 0 | 1 | 2;
/**
 * Static registration options to be returned in the initialize
 * request.
 */
export interface StaticRegistrationOptions {
    /**
     * The id used to register the request. The id can be used to deregister
     * the request again. See also Registration#id.
     */
    id?: string;
}
/**
 * General text document registration options.
 */
export interface TextDocumentRegistrationOptions {
    /**
     * A document selector to identify the scope of the registration. If set to null
     * the document selector provided on the client side will be used.
     */
    documentSelector: DocumentSelector | null;
}
/**
 * Completion options.
 */
export interface CompletionOptions {
    /**
     * Most tools trigger completion request automatically without explicitly requesting
     * it using a keyboard shortcut (e.g. Ctrl+Space). Typically they do so when the user
     * starts to type an identifier. For example if the user types `c` in a JavaScript file
     * code complete will automatically pop up present `console` besides others as a
     * completion item. Characters that make up identifiers don't need to be listed here.
     *
     * If code complete should automatically be trigger on characters not being valid inside
     * an identifier (for example `.` in JavaScript) list them in `triggerCharacters`.
     */
    triggerCharacters?: string[];
    /**
     * The server provides support to resolve additional
     * information for a completion item.
     */
    resolveProvider?: boolean;
}
/**
 * Signature help options.
 */
export interface SignatureHelpOptions {
    /**
     * The characters that trigger signature help
     * automatically.
     */
    triggerCharacters?: string[];
}
/**
 * Code Action options.
 */
export interface CodeActionOptions {
    /**
     * CodeActionKinds that this server may return.
     *
     * The list of kinds may be generic, such as `CodeActionKind.Refactor`, or the server
     * may list out every specific kind they provide.
     */
    codeActionKinds?: CodeActionKind[];
}
/**
 * Code Lens options.
 */
export interface CodeLensOptions {
    /**
     * Code lens has a resolve provider as well.
     */
    resolveProvider?: boolean;
}
/**
 * Format document on type options
 */
export interface DocumentOnTypeFormattingOptions {
    /**
     * A character on which formatting should be triggered, like `}`.
     */
    firstTriggerCharacter: string;
    /**
     * More trigger characters.
     */
    moreTriggerCharacter?: string[];
}
/**
 * Rename options
 */
export interface RenameOptions {
    /**
     * Renames should be checked and tested before being executed.
     */
    prepareProvider?: boolean;
}
/**
 * Document link options
 */
export interface DocumentLinkOptions {
    /**
     * Document links have a resolve provider as well.
     */
    resolveProvider?: boolean;
}
/**
 * Execute command options.
 */
export interface ExecuteCommandOptions {
    /**
     * The commands to be executed on the server
     */
    commands: string[];
}
/**
 * Save options.
 */
export interface SaveOptions {
    /**
     * The client is supposed to include the content on save.
     */
    includeText?: boolean;
}
export interface TextDocumentSyncOptions {
    /**
     * Open and close notifications are sent to the server.
     */
    openClose?: boolean;
    /**
     * Change notifications are sent to the server. See TextDocumentSyncKind.None, TextDocumentSyncKind.Full
     * and TextDocumentSyncKind.Incremental.
     */
    change?: TextDocumentSyncKind;
    /**
     * Will save notifications are sent to the server.
     */
    willSave?: boolean;
    /**
     * Will save wait until requests are sent to the server.
     */
    willSaveWaitUntil?: boolean;
    /**
     * Save notifications are sent to the server.
     */
    save?: SaveOptions;
}
/**
 * Defines the capabilities provided by a language
 * server.
 */
export interface _ServerCapabilities {
    /**
     * Defines how text documents are synced. Is either a detailed structure defining each notification or
     * for backwards compatibility the TextDocumentSyncKind number.
     */
    textDocumentSync?: TextDocumentSyncOptions | TextDocumentSyncKind;
    /**
     * The server provides hover support.
     */
    hoverProvider?: boolean;
    /**
     * The server provides completion support.
     */
    completionProvider?: CompletionOptions;
    /**
     * The server provides signature help support.
     */
    signatureHelpProvider?: SignatureHelpOptions;
    /**
     * The server provides goto definition support.
     */
    definitionProvider?: boolean;
    /**
     * The server provides find references support.
     */
    referencesProvider?: boolean;
    /**
     * The server provides document highlight support.
     */
    documentHighlightProvider?: boolean;
    /**
     * The server provides document symbol support.
     */
    documentSymbolProvider?: boolean;
    /**
     * The server provides workspace symbol support.
     */
    workspaceSymbolProvider?: boolean;
    /**
     * The server provides code actions. CodeActionOptions may only be
     * specified if the client states that it supports
     * `codeActionLiteralSupport` in its initial `initialize` request.
     */
    codeActionProvider?: boolean | CodeActionOptions;
    /**
     * The server provides code lens.
     */
    codeLensProvider?: CodeLensOptions;
    /**
     * The server provides document formatting.
     */
    documentFormattingProvider?: boolean;
    /**
     * The server provides document range formatting.
     */
    documentRangeFormattingProvider?: boolean;
    /**
     * The server provides document formatting on typing.
     */
    documentOnTypeFormattingProvider?: {
        /**
         * A character on which formatting should be triggered, like `}`.
         */
        firstTriggerCharacter: string;
        /**
         * More trigger characters.
         */
        moreTriggerCharacter?: string[];
    };
    /**
     * The server provides rename support. RenameOptions may only be
     * specified if the client states that it supports
     * `prepareSupport` in its initial `initialize` request.
     */
    renameProvider?: boolean | RenameOptions;
    /**
     * The server provides document link support.
     */
    documentLinkProvider?: DocumentLinkOptions;
    /**
     * The server provides execute command support.
     */
    executeCommandProvider?: ExecuteCommandOptions;
    /**
     * Experimental server capabilities.
     */
    experimental?: any;
}
export declare type ServerCapabilities = _ServerCapabilities & ImplementationServerCapabilities & TypeDefinitionServerCapabilities & WorkspaceFoldersServerCapabilities & ColorServerCapabilities & FoldingRangeServerCapabilities & DeclarationServerCapabilities;
/**
 * The initialize request is sent from the client to the server.
 * It is sent once as the request after starting up the server.
 * The requests parameter is of type [InitializeParams](#InitializeParams)
 * the response if of type [InitializeResult](#InitializeResult) of a Thenable that
 * resolves to such.
 */
export declare namespace InitializeRequest {
    const type: RequestType<InitializeParams, InitializeResult, InitializeError, void>;
}
/**
 * The initialize parameters
 */
export interface _InitializeParams {
    /**
     * The process Id of the parent process that started
     * the server.
     */
    processId: number | null;
    /**
     * The rootPath of the workspace. Is null
     * if no folder is open.
     *
     * @deprecated in favour of rootUri.
     */
    rootPath?: string | null;
    /**
     * The rootUri of the workspace. Is null if no
     * folder is open. If both `rootPath` and `rootUri` are set
     * `rootUri` wins.
     *
     * @deprecated in favour of workspaceFolders.
     */
    rootUri: string | null;
    /**
     * The capabilities provided by the client (editor or tool)
     */
    capabilities: ClientCapabilities;
    /**
     * User provided initialization options.
     */
    initializationOptions?: any;
    /**
     * The initial trace setting. If omitted trace is disabled ('off').
     */
    trace?: 'off' | 'messages' | 'verbose';
}
export declare type InitializeParams = _InitializeParams & WorkspaceFoldersInitializeParams;
/**
 * The result returned from an initialize request.
 */
export interface InitializeResult {
    /**
     * The capabilities the language server provides.
     */
    capabilities: ServerCapabilities;
    /**
     * Custom initialization results.
     */
    [custom: string]: any;
}
/**
 * Known error codes for an `InitializeError`;
 */
export declare namespace InitializeError {
    /**
     * If the protocol version provided by the client can't be handled by the server.
     * @deprecated This initialize error got replaced by client capabilities. There is
     * no version handshake in version 3.0x
     */
    const unknownProtocolVersion: number;
}
/**
 * The data type of the ResponseError if the
 * initialize request fails.
 */
export interface InitializeError {
    /**
     * Indicates whether the client execute the following retry logic:
     * (1) show the message provided by the ResponseError to the user
     * (2) user selects retry or cancel
     * (3) if user selected retry the initialize method is sent again.
     */
    retry: boolean;
}
export interface InitializedParams {
}
/**
 * The intialized notification is sent from the client to the
 * server after the client is fully initialized and the server
 * is allowed to send requests from the server to the client.
 */
export declare namespace InitializedNotification {
    const type: NotificationType<InitializedParams, void>;
}
/**
 * A shutdown request is sent from the client to the server.
 * It is sent once when the client decides to shutdown the
 * server. The only notification that is sent after a shutdown request
 * is the exit event.
 */
export declare namespace ShutdownRequest {
    const type: RequestType0<void, void, void>;
}
/**
 * The exit event is sent from the client to the server to
 * ask the server to exit its process.
 */
export declare namespace ExitNotification {
    const type: NotificationType0<void>;
}
/**
 * The configuration change notification is sent from the client to the server
 * when the client's configuration has changed. The notification contains
 * the changed configuration as defined by the language client.
 */
export declare namespace DidChangeConfigurationNotification {
    const type: NotificationType<DidChangeConfigurationParams, DidChangeConfigurationRegistrationOptions>;
}
export interface DidChangeConfigurationRegistrationOptions {
    section?: string | string[];
}
/**
 * The parameters of a change configuration notification.
 */
export interface DidChangeConfigurationParams {
    /**
     * The actual changed settings
     */
    settings: any;
}
/**
 * The message type
 */
export declare namespace MessageType {
    /**
     * An error message.
     */
    const Error = 1;
    /**
     * A warning message.
     */
    const Warning = 2;
    /**
     * An information message.
     */
    const Info = 3;
    /**
     * A log message.
     */
    const Log = 4;
}
export declare type MessageType = 1 | 2 | 3 | 4;
/**
 * The parameters of a notification message.
 */
export interface ShowMessageParams {
    /**
     * The message type. See {@link MessageType}
     */
    type: MessageType;
    /**
     * The actual message
     */
    message: string;
}
/**
 * The show message notification is sent from a server to a client to ask
 * the client to display a particular message in the user interface.
 */
export declare namespace ShowMessageNotification {
    const type: NotificationType<ShowMessageParams, void>;
}
export interface MessageActionItem {
    /**
     * A short title like 'Retry', 'Open Log' etc.
     */
    title: string;
}
export interface ShowMessageRequestParams {
    /**
     * The message type. See {@link MessageType}
     */
    type: MessageType;
    /**
     * The actual message
     */
    message: string;
    /**
     * The message action items to present.
     */
    actions?: MessageActionItem[];
}
/**
 * The show message request is sent from the server to the client to show a message
 * and a set of options actions to the user.
 */
export declare namespace ShowMessageRequest {
    const type: RequestType<ShowMessageRequestParams, MessageActionItem | null, void, void>;
}
/**
 * The log message notification is sent from the server to the client to ask
 * the client to log a particular message.
 */
export declare namespace LogMessageNotification {
    const type: NotificationType<LogMessageParams, void>;
}
/**
 * The log message parameters.
 */
export interface LogMessageParams {
    /**
     * The message type. See {@link MessageType}
     */
    type: MessageType;
    /**
     * The actual message
     */
    message: string;
}
/**
 * The telemetry event notification is sent from the server to the client to ask
 * the client to log telemetry data.
 */
export declare namespace TelemetryEventNotification {
    const type: NotificationType<any, void>;
}
/**
 * The parameters send in a open text document notification
 */
export interface DidOpenTextDocumentParams {
    /**
     * The document that was opened.
     */
    textDocument: TextDocumentItem;
}
/**
 * The document open notification is sent from the client to the server to signal
 * newly opened text documents. The document's truth is now managed by the client
 * and the server must not try to read the document's truth using the document's
 * uri. Open in this sense means it is managed by the client. It doesn't necessarily
 * mean that its content is presented in an editor. An open notification must not
 * be sent more than once without a corresponding close notification send before.
 * This means open and close notification must be balanced and the max open count
 * is one.
 */
export declare namespace DidOpenTextDocumentNotification {
    const type: NotificationType<DidOpenTextDocumentParams, TextDocumentRegistrationOptions>;
}
/**
 * The change text document notification's parameters.
 */
export interface DidChangeTextDocumentParams {
    /**
     * The document that did change. The version number points
     * to the version after all provided content changes have
     * been applied.
     */
    textDocument: VersionedTextDocumentIdentifier;
    /**
     * The actual content changes. The content changes describe single state changes
     * to the document. So if there are two content changes c1 and c2 for a document
     * in state S then c1 move the document to S' and c2 to S''.
     */
    contentChanges: TextDocumentContentChangeEvent[];
}
/**
 * Describe options to be used when registered for text document change events.
 */
export interface TextDocumentChangeRegistrationOptions extends TextDocumentRegistrationOptions {
    /**
     * How documents are synced to the server.
     */
    syncKind: TextDocumentSyncKind;
}
/**
 * The document change notification is sent from the client to the server to signal
 * changes to a text document.
 */
export declare namespace DidChangeTextDocumentNotification {
    const type: NotificationType<DidChangeTextDocumentParams, TextDocumentChangeRegistrationOptions>;
}
/**
 * The parameters send in a close text document notification
 */
export interface DidCloseTextDocumentParams {
    /**
     * The document that was closed.
     */
    textDocument: TextDocumentIdentifier;
}
/**
 * The document close notification is sent from the client to the server when
 * the document got closed in the client. The document's truth now exists where
 * the document's uri points to (e.g. if the document's uri is a file uri the
 * truth now exists on disk). As with the open notification the close notification
 * is about managing the document's content. Receiving a close notification
 * doesn't mean that the document was open in an editor before. A close
 * notification requires a previous open notification to be sent.
 */
export declare namespace DidCloseTextDocumentNotification {
    const type: NotificationType<DidCloseTextDocumentParams, TextDocumentRegistrationOptions>;
}
/**
 * The parameters send in a save text document notification
 */
export interface DidSaveTextDocumentParams {
    /**
     * The document that was closed.
     */
    textDocument: VersionedTextDocumentIdentifier;
    /**
     * Optional the content when saved. Depends on the includeText value
     * when the save notification was requested.
     */
    text?: string;
}
/**
 * Save registration options.
 */
export interface TextDocumentSaveRegistrationOptions extends TextDocumentRegistrationOptions, SaveOptions {
}
/**
 * The document save notification is sent from the client to the server when
 * the document got saved in the client.
 */
export declare namespace DidSaveTextDocumentNotification {
    const type: NotificationType<DidSaveTextDocumentParams, TextDocumentSaveRegistrationOptions>;
}
/**
 * The parameters send in a will save text document notification.
 */
export interface WillSaveTextDocumentParams {
    /**
     * The document that will be saved.
     */
    textDocument: TextDocumentIdentifier;
    /**
     * The 'TextDocumentSaveReason'.
     */
    reason: TextDocumentSaveReason;
}
/**
 * A document will save notification is sent from the client to the server before
 * the document is actually saved.
 */
export declare namespace WillSaveTextDocumentNotification {
    const type: NotificationType<WillSaveTextDocumentParams, TextDocumentRegistrationOptions>;
}
/**
 * A document will save request is sent from the client to the server before
 * the document is actually saved. The request can return an array of TextEdits
 * which will be applied to the text document before it is saved. Please note that
 * clients might drop results if computing the text edits took too long or if a
 * server constantly fails on this request. This is done to keep the save fast and
 * reliable.
 */
export declare namespace WillSaveTextDocumentWaitUntilRequest {
    const type: RequestType<WillSaveTextDocumentParams, TextEdit[] | null, void, TextDocumentRegistrationOptions>;
}
/**
 * The watched files notification is sent from the client to the server when
 * the client detects changes to file watched by the language client.
 */
export declare namespace DidChangeWatchedFilesNotification {
    const type: NotificationType<DidChangeWatchedFilesParams, DidChangeWatchedFilesRegistrationOptions>;
}
/**
 * The watched files change notification's parameters.
 */
export interface DidChangeWatchedFilesParams {
    /**
     * The actual file events.
     */
    changes: FileEvent[];
}
/**
 * The file event type
 */
export declare namespace FileChangeType {
    /**
     * The file got created.
     */
    const Created = 1;
    /**
     * The file got changed.
     */
    const Changed = 2;
    /**
     * The file got deleted.
     */
    const Deleted = 3;
}
export declare type FileChangeType = 1 | 2 | 3;
/**
 * An event describing a file change.
 */
export interface FileEvent {
    /**
     * The file's uri.
     */
    uri: string;
    /**
     * The change type.
     */
    type: FileChangeType;
}
/**
 * Describe options to be used when registered for text document change events.
 */
export interface DidChangeWatchedFilesRegistrationOptions {
    /**
     * The watchers to register.
     */
    watchers: FileSystemWatcher[];
}
export interface FileSystemWatcher {
    /**
     * The  glob pattern to watch. Glob patterns can have the following syntax:
     * - `*` to match one or more characters in a path segment
     * - `?` to match on one character in a path segment
     * - `**` to match any number of path segments, including none
     * - `{}` to group conditions (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
     * - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
     * - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
     */
    globPattern: string;
    /**
     * The kind of events of interest. If omitted it defaults
     * to WatchKind.Create | WatchKind.Change | WatchKind.Delete
     * which is 7.
     */
    kind?: number;
}
export declare namespace WatchKind {
    /**
     * Interested in create events.
     */
    const Create = 1;
    /**
     * Interested in change events
     */
    const Change = 2;
    /**
     * Interested in delete events
     */
    const Delete = 4;
}
/**
 * Diagnostics notification are sent from the server to the client to signal
 * results of validation runs.
 */
export declare namespace PublishDiagnosticsNotification {
    const type: NotificationType<PublishDiagnosticsParams, void>;
}
/**
 * The publish diagnostic notification's parameters.
 */
export interface PublishDiagnosticsParams {
    /**
     * The URI for which diagnostic information is reported.
     */
    uri: string;
    /**
     * An array of diagnostic information items.
     */
    diagnostics: Diagnostic[];
}
/**
 * Completion registration options.
 */
export interface CompletionRegistrationOptions extends TextDocumentRegistrationOptions, CompletionOptions {
}
/**
 * How a completion was triggered
 */
export declare namespace CompletionTriggerKind {
    /**
     * Completion was triggered by typing an identifier (24x7 code
     * complete), manual invocation (e.g Ctrl+Space) or via API.
     */
    const Invoked: 1;
    /**
     * Completion was triggered by a trigger character specified by
     * the `triggerCharacters` properties of the `CompletionRegistrationOptions`.
     */
    const TriggerCharacter: 2;
    /**
     * Completion was re-triggered as current completion list is incomplete
     */
    const TriggerForIncompleteCompletions: 3;
}
export declare type CompletionTriggerKind = 1 | 2 | 3;
/**
 * Contains additional information about the context in which a completion request is triggered.
 */
export interface CompletionContext {
    /**
     * How the completion was triggered.
     */
    triggerKind: CompletionTriggerKind;
    /**
     * The trigger character (a single character) that has trigger code complete.
     * Is undefined if `triggerKind !== CompletionTriggerKind.TriggerCharacter`
     */
    triggerCharacter?: string;
}
/**
 * Completion parameters
 */
export interface CompletionParams extends TextDocumentPositionParams {
    /**
     * The completion context. This is only available it the client specifies
     * to send this using `ClientCapabilities.textDocument.completion.contextSupport === true`
     */
    context?: CompletionContext;
}
/**
 * Request to request completion at a given text document position. The request's
 * parameter is of type [TextDocumentPosition](#TextDocumentPosition) the response
 * is of type [CompletionItem[]](#CompletionItem) or [CompletionList](#CompletionList)
 * or a Thenable that resolves to such.
 *
 * The request can delay the computation of the [`detail`](#CompletionItem.detail)
 * and [`documentation`](#CompletionItem.documentation) properties to the `completionItem/resolve`
 * request. However, properties that are needed for the initial sorting and filtering, like `sortText`,
 * `filterText`, `insertText`, and `textEdit`, must not be changed during resolve.
 */
export declare namespace CompletionRequest {
    const type: RequestType<CompletionParams, CompletionList | CompletionItem[] | null, void, CompletionRegistrationOptions>;
}
/**
 * Request to resolve additional information for a given completion item.The request's
 * parameter is of type [CompletionItem](#CompletionItem) the response
 * is of type [CompletionItem](#CompletionItem) or a Thenable that resolves to such.
 */
export declare namespace CompletionResolveRequest {
    const type: RequestType<CompletionItem, CompletionItem, void, void>;
}
/**
 * Request to request hover information at a given text document position. The request's
 * parameter is of type [TextDocumentPosition](#TextDocumentPosition) the response is of
 * type [Hover](#Hover) or a Thenable that resolves to such.
 */
export declare namespace HoverRequest {
    const type: RequestType<TextDocumentPositionParams, Hover | null, void, TextDocumentRegistrationOptions>;
}
/**
 * Signature help registration options.
 */
export interface SignatureHelpRegistrationOptions extends TextDocumentRegistrationOptions, SignatureHelpOptions {
}
export declare namespace SignatureHelpRequest {
    const type: RequestType<TextDocumentPositionParams, SignatureHelp | null, void, SignatureHelpRegistrationOptions>;
}
/**
 * A request to resolve the definition location of a symbol at a given text
 * document position. The request's parameter is of type [TextDocumentPosition]
 * (#TextDocumentPosition) the response is of either type [Definition](#Definition)
 * or a typed array of [DefinitionLink](#DefinitionLink) or a Thenable that resolves
 * to such.
 */
export declare namespace DefinitionRequest {
    const type: RequestType<TextDocumentPositionParams, Location | Location[] | LocationLink[] | null, void, TextDocumentRegistrationOptions>;
}
/**
 * Parameters for a [ReferencesRequest](#ReferencesRequest).
 */
export interface ReferenceParams extends TextDocumentPositionParams {
    context: ReferenceContext;
}
/**
 * A request to resolve project-wide references for the symbol denoted
 * by the given text document position. The request's parameter is of
 * type [ReferenceParams](#ReferenceParams) the response is of type
 * [Location[]](#Location) or a Thenable that resolves to such.
 */
export declare namespace ReferencesRequest {
    const type: RequestType<ReferenceParams, Location[] | null, void, TextDocumentRegistrationOptions>;
}
/**
 * Request to resolve a [DocumentHighlight](#DocumentHighlight) for a given
 * text document position. The request's parameter is of type [TextDocumentPosition]
 * (#TextDocumentPosition) the request response is of type [DocumentHighlight[]]
 * (#DocumentHighlight) or a Thenable that resolves to such.
 */
export declare namespace DocumentHighlightRequest {
    const type: RequestType<TextDocumentPositionParams, DocumentHighlight[] | null, void, TextDocumentRegistrationOptions>;
}
/**
 * A request to list all symbols found in a given text document. The request's
 * parameter is of type [TextDocumentIdentifier](#TextDocumentIdentifier) the
 * response is of type [SymbolInformation[]](#SymbolInformation) or a Thenable
 * that resolves to such.
 */
export declare namespace DocumentSymbolRequest {
    const type: RequestType<DocumentSymbolParams, DocumentSymbol[] | SymbolInformation[] | null, void, TextDocumentRegistrationOptions>;
}
/**
 * A request to list project-wide symbols matching the query string given
 * by the [WorkspaceSymbolParams](#WorkspaceSymbolParams). The response is
 * of type [SymbolInformation[]](#SymbolInformation) or a Thenable that
 * resolves to such.
 */
export declare namespace WorkspaceSymbolRequest {
    const type: RequestType<WorkspaceSymbolParams, SymbolInformation[] | null, void, void>;
}
/**
 * Params for the CodeActionRequest
 */
export interface CodeActionParams {
    /**
     * The document in which the command was invoked.
     */
    textDocument: TextDocumentIdentifier;
    /**
     * The range for which the command was invoked.
     */
    range: Range;
    /**
     * Context carrying additional information.
     */
    context: CodeActionContext;
}
export interface CodeActionRegistrationOptions extends TextDocumentRegistrationOptions, CodeActionOptions {
}
/**
 * A request to provide commands for the given text document and range.
 */
export declare namespace CodeActionRequest {
    const type: RequestType<CodeActionParams, (Command | CodeAction)[] | null, void, CodeActionRegistrationOptions>;
}
/**
 * Params for the Code Lens request.
 */
export interface CodeLensParams {
    /**
     * The document to request code lens for.
     */
    textDocument: TextDocumentIdentifier;
}
/**
 * Code Lens registration options.
 */
export interface CodeLensRegistrationOptions extends TextDocumentRegistrationOptions, CodeLensOptions {
}
/**
 * A request to provide code lens for the given text document.
 */
export declare namespace CodeLensRequest {
    const type: RequestType<CodeLensParams, CodeLens[] | null, void, CodeLensRegistrationOptions>;
}
/**
 * A request to resolve a command for a given code lens.
 */
export declare namespace CodeLensResolveRequest {
    const type: RequestType<CodeLens, CodeLens, void, void>;
}
export interface DocumentFormattingParams {
    /**
     * The document to format.
     */
    textDocument: TextDocumentIdentifier;
    /**
     * The format options
     */
    options: FormattingOptions;
}
/**
 * A request to to format a whole document.
 */
export declare namespace DocumentFormattingRequest {
    const type: RequestType<DocumentFormattingParams, TextEdit[] | null, void, TextDocumentRegistrationOptions>;
}
export interface DocumentRangeFormattingParams {
    /**
     * The document to format.
     */
    textDocument: TextDocumentIdentifier;
    /**
     * The range to format
     */
    range: Range;
    /**
     * The format options
     */
    options: FormattingOptions;
}
/**
 * A request to to format a range in a document.
 */
export declare namespace DocumentRangeFormattingRequest {
    const type: RequestType<DocumentRangeFormattingParams, TextEdit[] | null, void, TextDocumentRegistrationOptions>;
}
export interface DocumentOnTypeFormattingParams {
    /**
     * The document to format.
     */
    textDocument: TextDocumentIdentifier;
    /**
     * The position at which this request was send.
     */
    position: Position;
    /**
     * The character that has been typed.
     */
    ch: string;
    /**
     * The format options.
     */
    options: FormattingOptions;
}
/**
 * Format document on type options
 */
export interface DocumentOnTypeFormattingRegistrationOptions extends TextDocumentRegistrationOptions, DocumentOnTypeFormattingOptions {
}
/**
 * A request to format a document on type.
 */
export declare namespace DocumentOnTypeFormattingRequest {
    const type: RequestType<DocumentOnTypeFormattingParams, TextEdit[] | null, void, DocumentOnTypeFormattingRegistrationOptions>;
}
export interface RenameParams {
    /**
     * The document to rename.
     */
    textDocument: TextDocumentIdentifier;
    /**
     * The position at which this request was sent.
     */
    position: Position;
    /**
     * The new name of the symbol. If the given name is not valid the
     * request must return a [ResponseError](#ResponseError) with an
     * appropriate message set.
     */
    newName: string;
}
/**
 * A request to rename a symbol.
 */
export declare namespace RenameRequest {
    const type: RequestType<RenameParams, WorkspaceEdit | null, void, RenameRegistrationOptions>;
}
/**
 * A request to test and perform the setup necessary for a rename.
 */
export declare namespace PrepareRenameRequest {
    const type: RequestType<TextDocumentPositionParams, Range | {
        range: Range;
        placeholder: string;
    } | null, void, void>;
}
/**
 * Rename registration options.
 */
export interface RenameRegistrationOptions extends TextDocumentRegistrationOptions, RenameOptions {
}
export interface DocumentLinkParams {
    /**
     * The document to provide document links for.
     */
    textDocument: TextDocumentIdentifier;
}
/**
 * Document link registration options
 */
export interface DocumentLinkRegistrationOptions extends TextDocumentRegistrationOptions, DocumentLinkOptions {
}
/**
 * A request to provide document links
 */
export declare namespace DocumentLinkRequest {
    const type: RequestType<DocumentLinkParams, DocumentLink[] | null, void, DocumentLinkRegistrationOptions>;
}
/**
 * Request to resolve additional information for a given document link. The request's
 * parameter is of type [DocumentLink](#DocumentLink) the response
 * is of type [DocumentLink](#DocumentLink) or a Thenable that resolves to such.
 */
export declare namespace DocumentLinkResolveRequest {
    const type: RequestType<DocumentLink, DocumentLink, void, void>;
}
export interface ExecuteCommandParams {
    /**
     * The identifier of the actual command handler.
     */
    command: string;
    /**
     * Arguments that the command should be invoked with.
     */
    arguments?: any[];
}
/**
 * Execute command registration options.
 */
export interface ExecuteCommandRegistrationOptions extends ExecuteCommandOptions {
}
/**
 * A request send from the client to the server to execute a command. The request might return
 * a workspace edit which the client will apply to the workspace.
 */
export declare namespace ExecuteCommandRequest {
    const type: RequestType<ExecuteCommandParams, any, void, ExecuteCommandRegistrationOptions>;
}
/**
 * The parameters passed via a apply workspace edit request.
 */
export interface ApplyWorkspaceEditParams {
    /**
     * An optional label of the workspace edit. This label is
     * presented in the user interface for example on an undo
     * stack to undo the workspace edit.
     */
    label?: string;
    /**
     * The edits to apply.
     */
    edit: WorkspaceEdit;
}
/**
 * A response returned from the apply workspace edit request.
 */
export interface ApplyWorkspaceEditResponse {
    /**
     * Indicates whether the edit was applied or not.
     */
    applied: boolean;
    /**
     * Depending on the client's failure handling strategy `failedChange` might
     * contain the index of the change that failed. This property is only available
     * if the client signals a `failureHandlingStrategy` in its client capabilities.
     */
    failedChange?: number;
}
/**
 * A request sent from the server to the client to modified certain resources.
 */
export declare namespace ApplyWorkspaceEditRequest {
    const type: RequestType<ApplyWorkspaceEditParams, ApplyWorkspaceEditResponse, void, void>;
}
export { ImplementationRequest, TypeDefinitionRequest, WorkspaceFoldersRequest, DidChangeWorkspaceFoldersNotification, DidChangeWorkspaceFoldersParams, WorkspaceFolder, WorkspaceFoldersChangeEvent, ConfigurationRequest, ConfigurationParams, ConfigurationItem, DocumentColorRequest, ColorPresentationRequest, ColorProviderOptions, DocumentColorParams, ColorPresentationParams, FoldingRangeClientCapabilities, FoldingRangeProviderOptions, FoldingRangeRequest, FoldingRangeParams, FoldingRangeServerCapabilities, DeclarationClientCapabilities, DeclarationRequest, DeclarationServerCapabilities };
