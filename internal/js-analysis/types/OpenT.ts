/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {HumanBuilder} from "../Utils";
import UnknownT from "./UnknownT";
import {StaticMarkup, markup} from "@internal/markup";
import T from "./T";

export default class OpenT extends T {
	public static type = "OpenT";

	public humanize(builder: HumanBuilder): StaticMarkup {
		const type = this.utils.reduce(this);
		if (type === this) {
			return markup`open`;
		} else {
			return builder.humanize(type);
		}
	}

	public reduce(): T {
		const node = this.graph.find(this);
		if (node === undefined) {
			return new UnknownT(this.scope, this.originNode);
		}

		const values = node.lines.map((line) => this.utils.reduce(line.value));
		return this.scope.createUnion(values, this.originNode);
	}
}
