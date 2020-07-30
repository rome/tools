import * as code from 'vscode';
export default class ProtocolDocumentLink extends code.DocumentLink {
    data: any;
    constructor(range: code.Range, target?: code.Uri | undefined);
}
