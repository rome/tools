/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";

export default createVisitor({
	name: "propertyLiterals",
	enter() {
		return signals.retain;
	},
});
