/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {consumeUnknown, Consumer, consume} from '@romejs/consume';
import url = require('url');
import {number0, number1, coerce0} from '@romejs/ob1';

export type ConsumableUrl = {
  path: Consumer;
  query: Consumer;
};

export function consumeUrl(rawUrl: string): ConsumableUrl {
  const parts = url.parse(rawUrl, true);

  const query = consumeUnknown({...parts.query}, 'parse/url/query');

  const path = consume({
    value: parts.pathname,
    context: {
      category: 'parse/url',

      getDiagnosticPointer() {
        return {
          language: 'url',
          mtime: undefined,
          sourceText: rawUrl,
          filename: 'url',
          start: {
            index: number0,
            line: number1,
            column: number0,
          },
          end: {
            index: coerce0(rawUrl.length - 1),
            line: number1,
            column: coerce0(rawUrl.length - 1),
          },
        };
      },
    },
  });

  return {query, path};
}
