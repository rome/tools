/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// Project Management
import run from './run';
import publish from './publish';

// Process Management
import status from './status';
import stop from './stop';
import develop from './develop';
import config from './config';

// Source Code
import compile from './compile';
import resolve from './resolve';
import analyzeDependencies from './analyzeDependencies';
import parse from './parse';
import bundle from './bundle';
import format from './format';

// Code Quality
import lint from './lint';
import ci from './ci';
import test from './test';

// Hidden commands, useful for internal debugging but not much else
import evict from './_evict';
import moduleSignature from './_moduleSignature';
import noop from './noop';

//
import {MasterCommand} from '../../commands';
export const masterCommands: Map<string, MasterCommand<any>> = new Map();
masterCommands.set('_moduleSignature', moduleSignature);
masterCommands.set('evict', evict);
masterCommands.set('test', test);
masterCommands.set('lint', lint);
masterCommands.set('config', config);
masterCommands.set('bundle', bundle);
masterCommands.set('parse', parse);
masterCommands.set('analyzeDependencies', analyzeDependencies);
masterCommands.set('resolve', resolve);
masterCommands.set('compile', compile);
masterCommands.set('stop', stop);
masterCommands.set('status', status);
masterCommands.set('run', run);
masterCommands.set('publish', publish);
masterCommands.set('ci', ci);
masterCommands.set('develop', develop);
masterCommands.set('format', format);
masterCommands.set('_noop', noop);
