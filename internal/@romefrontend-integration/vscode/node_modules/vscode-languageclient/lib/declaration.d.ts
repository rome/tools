import { Disposable, TextDocument, ProviderResult, Position as VPosition, Declaration as VDeclaration } from 'vscode';
import { ClientCapabilities, CancellationToken, ServerCapabilities, TextDocumentRegistrationOptions, DocumentSelector } from 'vscode-languageserver-protocol';
import { TextDocumentFeature, BaseLanguageClient } from './client';
export interface ProvideDeclarationSignature {
    (document: TextDocument, position: VPosition, token: CancellationToken): ProviderResult<VDeclaration>;
}
export interface DeclarationMiddleware {
    provideDeclaration?: (this: void, document: TextDocument, position: VPosition, token: CancellationToken, next: ProvideDeclarationSignature) => ProviderResult<VDeclaration>;
}
export declare class DeclarationFeature extends TextDocumentFeature<TextDocumentRegistrationOptions> {
    constructor(client: BaseLanguageClient);
    fillClientCapabilities(capabilites: ClientCapabilities): void;
    initialize(capabilities: ServerCapabilities, documentSelector: DocumentSelector): void;
    protected registerLanguageProvider(options: TextDocumentRegistrationOptions): Disposable;
}
