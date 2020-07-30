/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Dict} from '@internal/typescript-helpers';

type UnknownObject = Dict<unknown>;

type ModuleID = number | string;

type ModuleFactory = (module: Module, exports: UnknownObject) => void;

type Module = {
  exports: UnknownObject;
};

type ModuleDefinition = {
  es: boolean;
  factory: ModuleFactory;
  isInitialized: boolean;
  module: Module;
};

var hasOwn = Object.prototype.hasOwnProperty;

var definitions: Dict<ModuleDefinition> = {};

function createModule(): Module {
  return {exports: {}};
}

function createDefinition(
  moduleId: ModuleID,
  es: boolean,
  factory: ModuleFactory,
) {
  if (definitions[moduleId] !== undefined) {
    // prevent repeated calls to `global.nativeRequire` to overwrite modules
    // that are already loaded
    return;
  }

  definitions[moduleId] = {
    es,
    factory,
    module: createModule(),
    isInitialized: false,
  };
}

// @ts-ignore
const Rome = (global.Rome = {
  declareCJS: function(id: ModuleID, factory: ModuleFactory) {
    createDefinition(id, false, factory);
  },

  declareES: function(id: ModuleID, factory: ModuleFactory) {
    createDefinition(id, true, factory);
  },

  requireDefault: function(moduleId: ModuleID) {
    var def = Rome.requireDefinition(moduleId);
    var exports = def.module.exports;
    return def.es ? exports.default : exports;
  },

  requireNamespace: function(moduleId: ModuleID) {
    var def = Rome.requireDefinition(moduleId);
    return def.module.exports;
  },

  requireDefinition: function(moduleId: ModuleID): ModuleDefinition {
    const def = definitions[moduleId];

    if (def === undefined || !hasOwn.call(definitions, moduleId)) {
      throw new Error('No module ' + moduleId);
    }

    if (def.isInitialized) {
      return def;
    }

    const {module} = def;

    try {
      def.isInitialized = true;
      def.factory(module, module.exports);
    } catch (err) {
      def.isInitialized = false;
      def.module = createModule();
      throw err;
    }

    // Babel and others interop
    if (module.exports && module.exports.__esModule) {
      def.es = true;
    }

    return def;
  },
});

// @ts-ignore
global.__DEV__ = true;
