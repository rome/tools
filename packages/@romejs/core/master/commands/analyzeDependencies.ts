/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {commandCategories} from '../../commands';
import {createMasterCommand} from '../../commands';
import {Consumer} from '@romejs/consume';
import {createUnknownFilePath} from '@romejs/path';
import {SourceLocation} from '@romejs/parser-core';

type Flags = {
  focusSource: undefined | string;
  compact: boolean;
};

function removeLoc<T extends {loc?: SourceLocation}>(obj: T): Omit<T, 'loc'> {
  const {loc, ...locless} = obj;
  loc;
  return locless;
}

export default createMasterCommand({
  category: commandCategories.SOURCE_CODE,
  description: 'analyze and dump the dependencies of a file',

  defineFlags(c: Consumer): Flags {
    return {
      compact: c.get('compact').asBoolean(false),
      focusSource: c.get('focusSource').asStringOrVoid(),
    };
  },

  async default(req: MasterRequest, commandFlags: Flags): Promise<void> {
    const {master, reporter} = req;
    const {args} = req.query;
    req.expectArgumentLength(1);

    const filename = await master.resolver.resolveEntryAssertPath({
      ...req.getResolverOptionsFromFlags(),
      source: createUnknownFilePath(args[0]),
    }, {location: req.getDiagnosticPointerFromFlags({type: 'arg', key: 0})});

    let res = await req.requestWorkerAnalyzeDependencies(filename);

    const {focusSource} = commandFlags;
    if (focusSource !== undefined) {
      res = {
        ...res,
        importFirstUsage: res.importFirstUsage.filter((dep) => {
          return dep.source === focusSource;
        }),
        dependencies: res.dependencies.filter((dep) => {
          return dep.source === focusSource;
        }),
      };
    }

    if (commandFlags.compact) {
      res =
        {
          ...res,
          importFirstUsage: res.importFirstUsage.map((imp) => {
            return removeLoc(imp);
          }),
          exports: res.exports.map((exp) => {
            // This weird switch is because TS only returns an object with the properties common amongst all
            switch (exp.type) {
              case 'local':
                return removeLoc(exp);

              case 'external':
                return removeLoc(exp);

              case 'externalAll':
                return removeLoc(exp);
            }
          }),
          dependencies: res.dependencies.map((dep) => {
            return {
              ...removeLoc(dep),
              names: dep.names.map((name) => {
                return removeLoc(name);
              }),
            };
          }),
        };
    }

    reporter.inspect(res);
  },
});
