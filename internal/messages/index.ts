/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

function sprintf(msg: string, ...args: Array<unknown>): string {
	return msg.replace(
		/\$(\d+)/g,
		(match, num) => {
			return String(args[num]);
		},
	);
}

type MessagesShape = {
	[key: string]: string;
};

type Factory = (...args: Array<unknown>) => string;

type FactoryObject<Messages extends MessagesShape> = {
	[P in keyof Messages]: Factory
};

export function createMessageFactory<Messages extends MessagesShape>(
	messages: Messages,
): FactoryObject<Messages> {
	// @ts-ignore: TS complains about {} not being full of the possible properties in message... which is true
	// but they will be filled it by the time we return
	const obj: FactoryObject<Messages> = {};

	for (const key in messages) {
		const msg = messages[key];

		obj[key] = (...args: Array<unknown>) => {
			return sprintf(msg, ...args);
		};
	}

	return obj;
}
