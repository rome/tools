import { Uri } from "vscode";

export class SyntaxTreeDocument {
	private _value: string;
	private readonly _uri: Uri;

	constructor(uri: Uri, result: string) {
		this._uri = uri;
		this._value = result;
	}

	set value(cst: string) {
		this._value = cst;
	}

	get value() {
		return this._value;
	}
}
