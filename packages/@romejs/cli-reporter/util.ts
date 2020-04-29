/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ReporterProgress} from './types';

export function mergeProgresses(
  progresses: Array<ReporterProgress>,
): ReporterProgress {
  if (progresses.length === 1) {
    return progresses[0];
  }

  return {
    render: () => {
      for (const progress of progresses) {
        progress.render();
      }
    },
    setCurrent: (current: number) => {
      for (const progress of progresses) {
        progress.setCurrent(current);
      }
    },
    setTotal: (total: number, approximate?: boolean) => {
      for (const progress of progresses) {
        progress.setTotal(total, approximate);
      }
    },
    setText: (text: string) => {
      for (const progress of progresses) {
        progress.setText(text);
      }
    },
    pushText: (text: string) => {
      for (const progress of progresses) {
        progress.pushText(text);
      }
    },
    popText: (text: string) => {
      for (const progress of progresses) {
        progress.popText(text);
      }
    },
    setApproximateETA: (duration: number) => {
      for (const progress of progresses) {
        progress.setApproximateETA(duration);
      }
    },
    tick: () => {
      for (const progress of progresses) {
        progress.tick();
      }
    },
    end: () => {
      for (const progress of progresses) {
        progress.end();
      }
    },
    pause: () => {
      for (const progress of progresses) {
        progress.pause();
      }
    },
    resume: () => {
      for (const progress of progresses) {
        progress.resume();
      }
    },
  };
}
