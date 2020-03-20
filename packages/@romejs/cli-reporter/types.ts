/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProgressOptions} from './Progress';
import {Event} from '@romejs/events';

export type Package = {
  name: string;
  version?: string;
};

export type ReporterStream = {
  type: 'out' | 'error' | 'all';
  columns: number;
  format: 'ansi' | 'html' | 'none';
  write: (chunk: string) => void;
  teardown?: () => void;
};

export type ReporterDerivedStreams = {
  columnsUpdated: Event<number, void>;
  stdout: ReporterStream;
  stderr: ReporterStream;
};

export type ProgressShape = {
  render: () => void;
  setCurrent: (current: number) => void;
  setTotal: (total: number, approximate?: boolean) => void;
  setTitle: (title: string) => void;
  setText: (text: string) => void;
  pushText: (text: string) => void;
  popText: (text: string) => void;
  setApproximateETA: (duration: number) => void;
  tick: () => void;
  end: () => void;
  pause: () => void;
  resume: () => void;
};

export type RemoteReporterReceiveMessage = {
  type: 'ENDED';
  id: string;
};

export type RemoteReporterClientMessage =
  | {
    type: 'PROGRESS_CREATE';
    id: string;
    opts: undefined | Partial<ProgressOptions>;
  }
  | {
    type: 'PROGRESS_SET_CURRENT';
    current: number;
    id: string;
  }
  | {
    type: 'PROGRESS_SET_APPROXIMATE_ETA';
    duration: number;
    id: string;
  }
  | {
    type: 'PROGRESS_SET_TOTAL';
    total: number;
    id: string;
    approximate: boolean;
  }
  | {
    type: 'PROGRESS_SET_TITLE';
    title: string;
    id: string;
  }
  | {
    type: 'PROGRESS_SET_TEXT';
    text: string;
    id: string;
  }
  | {
    type: 'PROGRESS_PUSH_TEXT';
    text: string;
    id: string;
  }
  | {
    type: 'PROGRESS_POP_TEXT';
    text: string;
    id: string;
  }
  | {
    type: 'PROGRESS_TICK';
    id: string;
  }
  | {
    type: 'PROGRESS_END';
    id: string;
  }
  | {
    type: 'PROGRESS_PAUSE';
    id: string;
  }
  | {
    type: 'PROGRESS_RESUME';
    id: string;
  };
