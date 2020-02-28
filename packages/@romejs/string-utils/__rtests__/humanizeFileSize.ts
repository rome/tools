/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import { humanizeFileSize } from '../humanizeFileSize';
import test from '@romejs/test';

test('humanizeFileSize', t => {
    const testCases = [
        { input: 1, expected: '1B' },
        { input: 10, expected: '10B' },
        { input: 10, expected: '100B' },

        { input: 1000, expected: '1kB' },
        { input: 10000, expected: '10kB' },
        { input: 100000, expected: '100kB' },

        { input: 1000000, expected: '1MB' },
        { input: 10000000, expected: '10MB' },
        { input: 10000000, expected: '100MB' },

        { input: 1000000000, expected: '1GB' },
        { input: 10000000000, expected: '10GB' },
        { input: 100000000000, expected: '100GB' },

        { input: 1000000000000, expected: '1TB' },
        { input: 10000000000000, expected: '10TB' },
        { input: 100000000000000, expected: '100TB' },

        { input: 1000000000000000, expected: '1PB' },
        { input: 10000000000000000, expected: '10PB' },
        { input: 100000000000000000, expected: '100PB' },

        { input: 1000000000000000000, expected: '1EB' },
        { input: 10000000000000000000, expected: '10EB' },
        { input: 100000000000000000000, expected: '100EB' },

        { input: 1000000000000000000000, expected: '1ZB' },
        { input: 10000000000000000000000, expected: '10ZB' },
        { input: 100000000000000000000000, expected: '100ZB' },

        { input: 1000000000000000000000000, expected: '1YB' },
        { input: 10000000000000000000000000, expected: '10YB' },
    ]

    testCases.forEach(td => {
        t.is(humanizeFileSize(td.input), td.expected)
    });
});
