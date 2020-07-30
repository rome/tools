import * as code from 'vscode';
export default class ProtocolCodeLens extends code.CodeLens {
    data: any;
    constructor(range: code.Range);
}
