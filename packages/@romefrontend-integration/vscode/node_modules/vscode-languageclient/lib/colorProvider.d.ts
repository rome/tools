import { Disposable, TextDocument, ProviderResult, Range as VRange, Color as VColor, ColorPresentation as VColorPresentation, ColorInformation as VColorInformation } from 'vscode';
import { ClientCapabilities, CancellationToken, ServerCapabilities, TextDocumentRegistrationOptions, DocumentSelector } from 'vscode-languageserver-protocol';
import { TextDocumentFeature, BaseLanguageClient } from './client';
export interface ProvideDocumentColorsSignature {
    (document: TextDocument, token: CancellationToken): ProviderResult<VColorInformation[]>;
}
export interface ProvideColorPresentationSignature {
    (color: VColor, context: {
        document: TextDocument;
        range: VRange;
    }, token: CancellationToken): ProviderResult<VColorPresentation[]>;
}
export interface ColorProviderMiddleware {
    provideDocumentColors?: (this: void, document: TextDocument, token: CancellationToken, next: ProvideDocumentColorsSignature) => ProviderResult<VColorInformation[]>;
    provideColorPresentations?: (this: void, color: VColor, context: {
        document: TextDocument;
        range: VRange;
    }, token: CancellationToken, next: ProvideColorPresentationSignature) => ProviderResult<VColorPresentation[]>;
}
export declare class ColorProviderFeature extends TextDocumentFeature<TextDocumentRegistrationOptions> {
    constructor(client: BaseLanguageClient);
    fillClientCapabilities(capabilites: ClientCapabilities): void;
    initialize(capabilities: ServerCapabilities, documentSelector: DocumentSelector): void;
    protected registerLanguageProvider(options: TextDocumentRegistrationOptions): Disposable;
    private asColor;
    private asColorInformations;
    private asColorPresentations;
}
