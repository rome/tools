import * as code from 'vscode';
import * as proto from 'vscode-languageserver-protocol';
export default class ProtocolCompletionItem extends code.CompletionItem {
    data: any;
    fromEdit: boolean;
    documentationFormat: string;
    originalItemKind: proto.CompletionItemKind;
    deprecated: boolean;
    constructor(label: string);
}
