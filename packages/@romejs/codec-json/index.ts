/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {JSONParserOptions, JSONValue, PathToComments, Tokens} from './types';
import createParser from './parse';
import {Consumer, consume} from '@romejs/consume';
import {stringifyRootConsumer} from './stringify';
import {TokenValues} from '@romejs/parser-core';

export {
  JSONArray,
  JSONObject,
  JSONParserOptions,
  JSONPropertyValue,
  JSONValue,
} from './types';

export type ConsumeJSONResult = {
  hasExtensions: boolean;
  consumer: Consumer;
  comments: PathToComments;
};

export function consumeJSON(opts: JSONParserOptions): Consumer {
  return consumeJSONExtra(opts).consumer;
}

export function consumeJSONExtra(opts: JSONParserOptions): ConsumeJSONResult {
  const parser = createParser(opts);
  const {value, context} = parser.parse();

  return {
    hasExtensions: parser.hasExtensions,
    consumer: consume({
      filePath: parser.path,
      context,
      objectPath: [],
      value,
      parent: undefined,
    }),
    comments: parser.pathToComments,
  };
}

export function parseJSON(opts: JSONParserOptions): JSONValue {
  return createParser(opts).parse().value;
}

export function tokenizeJSON(
  opts: JSONParserOptions,
): Array<TokenValues<Tokens>> {
  return createParser(opts).tokenizeAll();
}

export function stringifyJSON(
  opts: {
    consumer: Consumer;
    comments: PathToComments;
  },
): string {
  return stringifyRootConsumer(opts.consumer, opts.comments);
}
