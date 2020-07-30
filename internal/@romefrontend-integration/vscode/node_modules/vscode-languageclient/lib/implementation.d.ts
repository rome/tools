import { Disposable, TextDocument, ProviderResult, Position as VPosition, Definition as VDefinition, DefinitionLink as VDefinitionLink } from 'vscode';
import { ClientCapabilities, CancellationToken, ServerCapabilities, TextDocumentRegistrationOptions, DocumentSelector } from 'vscode-languageserver-protocol';
import { TextDocumentFeature, BaseLanguageClient } from './client';
export interface ProvideImplementationSignature {
    (document: TextDocument, position: VPosition, token: CancellationToken): ProviderResult<VDefinition | VDefinitionLink[]>;
}
export interface ImplementationMiddleware {
    provideImplementation?: (this: void, document: TextDocument, position: VPosition, token: CancellationToken, next: ProvideImplementationSignature) => ProviderResult<VDefinition | VDefinitionLink[]>;
}
export declare class ImplementationFeature extends TextDocumentFeature<TextDocumentRegistrationOptions> {
    constructor(client: BaseLanguageClient);
    fillClientCapabilities(capabilites: ClientCapabilities): void;
    initialize(capabilities: ServerCapabilities, documentSelector: DocumentSelector): void;
    protected registerLanguageProvider(options: TextDocumentRegistrationOptions): Disposable;
}
