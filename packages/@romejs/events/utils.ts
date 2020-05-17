/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {EventSubscription} from "./types";

export function mergeEventSubscriptions(
	subs: Array<EventSubscription>,
): EventSubscription {
	return {
		unsubscribe() {
			for (const sub of subs) {
				sub.unsubscribe();
			}
		},
	};
}
