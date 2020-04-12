/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {parseMarkup} from './parse';

test(
  'should not parse string escapes',
  (t) => {
    t.snapshot(parseMarkup('<filelink target="C:\\Users\\sebmck\\file.ts" />'));
    t.snapshot(
      parseMarkup(
        '<blue>[MemoryFileSystem] Adding new project folder C:\\Users\\sebmck\\rome</blue>',
      ),
    );

    t.snapshot(
      parseMarkup(
        '  \\<blue>[MemoryFileSystem] Adding new project folder C:\\\\Users\\\\Sebastian\\\\rome\\\\\\</blue>\n        <red><emphasis>^</emphasis></red> ',
      ),
    );
  },
);
