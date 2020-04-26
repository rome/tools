/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {escapeMarkup, unescapeTextValue} from './escape';

test(
  'should properly escape and then unescape backslashes',
  (t) => {
    t.is(unescapeTextValue(escapeMarkup('\\')), '\\');
    t.is(escapeMarkup('C:\\Users\\sebmck\\'), 'C:\\\\Users\\\\sebmck\\\\');
  },
);
