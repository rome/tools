import { Disposable, TextDocument, ProviderResult, FoldingRange as VFoldingRange, FoldingContext } from 'vscode';
import { ClientCapabilities, CancellationToken, ServerCapabilities, TextDocumentRegistrationOptions, DocumentSelector } from 'vscode-languageserver-protocol';
import { TextDocumentFeature, BaseLanguageClient } from './client';
export interface ProvideFoldingRangeSignature {
    (document: TextDocument, context: FoldingContext, token: CancellationToken): ProviderResult<VFoldingRange[]>;
}
export interface FoldingRangeProviderMiddleware {
    provideFoldingRanges?: (this: void, document: TextDocument, context: FoldingContext, token: CancellationToken, next: ProvideFoldingRangeSignature) => ProviderResult<VFoldingRange[]>;
}
export declare class FoldingRangeFeature extends TextDocumentFeature<TextDocumentRegistrationOptions> {
    constructor(client: BaseLanguageClient);
    fillClientCapabilities(capabilites: ClientCapabilities): void;
    initialize(capabilities: ServerCapabilities, documentSelector: DocumentSelector): void;
    protected registerLanguageProvider(options: TextDocumentRegistrationOptions): Disposable;
    private asFoldingRangeKind;
    private asFoldingRanges;
}
