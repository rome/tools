/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {HumanBuilder} from "../Utils";
import UnknownT from "./UnknownT";
import {Markup, markup} from "@romefrontend/markup";
import T from "./T";

export default class OpenT extends T {
	static type = "OpenT";

	humanize(builder: HumanBuilder): Markup {
		const type = this.utils.reduce(this);
		if (type === this) {
			return markup`open`;
		} else {
			return builder.humanize(type);
		}
	}

	reduce(): T {
		const node = this.graph.find(this);
		if (node === undefined) {
			return new UnknownT(this.scope, this.originNode);
		}

		const values = node.lines.map((line) => this.utils.reduce(line.value));
		return this.scope.createUnion(values, this.originNode);
	}
}
