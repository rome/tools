/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from "@internal/ast";
import {HumanBuilder} from "../Utils";
import {HydrateData} from "../Evaluator";
import {Scope} from "../scopes";
import T from "./T";
import {StaticMarkup, markup} from "@internal/markup";

export default class ImportT extends T {
	constructor(
		scope: Scope,
		originNode: undefined | AnyNode,
		opts: {
			importedName: undefined | string;
			relative?: string;
			source: string;
		},
	) {
		super(scope, originNode);
		this.importedName = opts.importedName;
		this.relative =
			opts.relative === undefined ? scope.evaluator.filename : opts.relative;
		this.source = opts.source;
		this.absolute = undefined;
		this.resolvedType = undefined;
		scope.evaluator.addImport(
			this,
			{
				importedName: this.importedName,
				relative: this.relative,
				source: this.source,
			},
		);
	}

	public static type = "ImportT";
	private importedName: undefined | string;
	private absolute: undefined | string;
	private resolvedType: undefined | T;
	private relative: string;
	private source: string;

	public setAbsolute(absolute: undefined | string) {
		this.absolute = absolute;
	}

	public setResolvedType(resolvedType: T) {
		this.resolvedType = resolvedType;
	}

	public serialize(): HydrateData {
		return {
			importedName: this.importedName,
			relative: this.relative,
			source: this.source,
		};
	}

	public static hydrate(
		scope: Scope,
		originNode: undefined | AnyNode,
		data: HydrateData,
	): T {
		return new ImportT(
			scope,
			originNode,
			{
				importedName: String(data.importedName),
				source: String(data.source),
				relative: String(data.relative),
			},
		);
	}

	public humanize(builder: HumanBuilder): StaticMarkup {
		let object;
		if (this.resolvedType !== undefined) {
			object = builder.humanize(this.resolvedType);
		} else if (this.absolute === undefined) {
			object = markup`$Exports<"${this.source}", "${this.relative}">`;
		} else {
			object = markup`$Exports<"${this.absolute}">`;
		}

		if (this.importedName === undefined) {
			return object;
		} else {
			return markup`${object}.${this.importedName}`;
		}
	}

	public reduce(): T {
		if (this.resolvedType === undefined) {
			return this;
		} else {
			return this.resolvedType;
		}
	}
}
