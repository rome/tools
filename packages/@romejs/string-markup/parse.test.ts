/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {parseMarkup} from './parse';

test('should not parse string escapes', (t) => {
  t.snapshot(parseMarkup('<filelink target="C:\\Users\\sebmck\\file.ts" />'));
  t.snapshot(
    parseMarkup(
      '<info>[MemoryFileSystem] Adding new project folder C:\\Users\\sebmck\\rome</info>',
    ),
  );

  t.snapshot(
    parseMarkup(
      '  \\<info>[MemoryFileSystem] Adding new project folder C:\\\\Users\\\\Sebastian\\\\rome\\\\\\</info>\n        <error><emphasis>^</emphasis></error> ',
    ),
  );
});
