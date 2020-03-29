import * as code from 'vscode';
import * as proto from 'vscode-languageserver-protocol';
export interface Converter {
    asUri(uri: code.Uri): string;
    asTextDocumentIdentifier(textDocument: code.TextDocument): proto.TextDocumentIdentifier;
    asVersionedTextDocumentIdentifier(textDocument: code.TextDocument): proto.VersionedTextDocumentIdentifier;
    asOpenTextDocumentParams(textDocument: code.TextDocument): proto.DidOpenTextDocumentParams;
    asChangeTextDocumentParams(textDocument: code.TextDocument): proto.DidChangeTextDocumentParams;
    asChangeTextDocumentParams(event: code.TextDocumentChangeEvent): proto.DidChangeTextDocumentParams;
    asCloseTextDocumentParams(textDocument: code.TextDocument): proto.DidCloseTextDocumentParams;
    asSaveTextDocumentParams(textDocument: code.TextDocument, includeContent?: boolean): proto.DidSaveTextDocumentParams;
    asWillSaveTextDocumentParams(event: code.TextDocumentWillSaveEvent): proto.WillSaveTextDocumentParams;
    asTextDocumentPositionParams(textDocument: code.TextDocument, position: code.Position): proto.TextDocumentPositionParams;
    asCompletionParams(textDocument: code.TextDocument, position: code.Position, context: code.CompletionContext): proto.CompletionParams;
    asWorkerPosition(position: code.Position): proto.Position;
    asPosition(value: code.Position): proto.Position;
    asPosition(value: undefined): undefined;
    asPosition(value: null): null;
    asPosition(value: code.Position | undefined | null): proto.Position | undefined | null;
    asRange(value: code.Range): proto.Range;
    asRange(value: undefined): undefined;
    asRange(value: null): null;
    asRange(value: code.Range | undefined | null): proto.Range | undefined | null;
    asDiagnosticSeverity(value: code.DiagnosticSeverity): number;
    asDiagnostic(item: code.Diagnostic): proto.Diagnostic;
    asDiagnostics(items: code.Diagnostic[]): proto.Diagnostic[];
    asCompletionItem(item: code.CompletionItem): proto.CompletionItem;
    asTextEdit(edit: code.TextEdit): proto.TextEdit;
    asReferenceParams(textDocument: code.TextDocument, position: code.Position, options: {
        includeDeclaration: boolean;
    }): proto.ReferenceParams;
    asCodeActionContext(context: code.CodeActionContext): proto.CodeActionContext;
    asCommand(item: code.Command): proto.Command;
    asCodeLens(item: code.CodeLens): proto.CodeLens;
    asFormattingOptions(item: code.FormattingOptions): proto.FormattingOptions;
    asDocumentSymbolParams(textDocument: code.TextDocument): proto.DocumentSymbolParams;
    asCodeLensParams(textDocument: code.TextDocument): proto.CodeLensParams;
    asDocumentLink(item: code.DocumentLink): proto.DocumentLink;
    asDocumentLinkParams(textDocument: code.TextDocument): proto.DocumentLinkParams;
}
export interface URIConverter {
    (value: code.Uri): string;
}
export declare function createConverter(uriConverter?: URIConverter): Converter;
