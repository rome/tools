/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {descriptions} from "@internal/diagnostics";
import {Scope} from "../../scopes";
import E, {ErrorDefinition} from "./E";
import {AnyNode} from "@internal/ast";

export default class UnknownImportE extends E {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		opts: {
			possibleNames: Array<string>;
			importedName: string;
			source: string;
		},
	) {
		super(scope, originNode);
		this.possibleNames = opts.possibleNames;
		this.importedName = opts.importedName;
		this.source = opts.source;
	}

	public static type = "UnknownImportE";
	private importedName: string;
	private source: string;
	private possibleNames: Array<string>;

	public getError(): ErrorDefinition {
		return {
			description: descriptions.TYPE_CHECK.UNKNOWN_IMPORT(
				this.importedName,
				this.source,
				this.possibleNames,
			),
			lowerTarget: this,
		};
	}
}
