/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import setProcessTitle from './utils/setProcessTitle';
import {TestWorker} from '@romejs/core';
import {parseCLIFlagsFromProcess} from '@romejs/cli-flags';
import {TestWorkerFlags} from '@romejs/core/test-worker/TestWorker';

export default async function testWorker() {
	setProcessTitle('test-worker');

	const parser = parseCLIFlagsFromProcess({
		programName: 'rome test-worker',
		defineFlags(c): TestWorkerFlags {
			return {
				inspectorPort: c.get('inspectorPort').asNumberFromString(),
			};
		},
	});
	const flags: TestWorkerFlags = await parser.init();

	const worker = new TestWorker();
	worker.init(flags);
}
