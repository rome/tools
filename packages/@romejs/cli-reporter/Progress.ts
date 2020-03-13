/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {formatAnsi, escapes} from '@romejs/string-ansi';
import {humanizeNumber, humanizeTime} from '@romejs/string-utils';
import {Reporter} from '@romejs/cli-reporter';
import {RemoteReporterClientMessage, ReporterStream} from './types';

type BoldRanges = Array<[number, number]>;
type SplitBar = Array<[number, string]>;

// 30 columns a second
const BOUNCER_INTERVAL = 1000 / 30;
const BOUNCER_WIDTH = 20;

export type ProgressOptions = {
  name?: string;
  initDelay?: number;
  elapsed?: boolean;
  eta?: boolean;
  persistent?: boolean;
};

const DEFAULT_PROGRESS_OPTIONS: ProgressOptions = {
  name: undefined,
  initDelay: undefined,
  elapsed: true,
  eta: true,
  persistent: false,
};

export default class Progress {
  constructor(
    reporter: Reporter,
    opts: Partial<ProgressOptions> = {},
    onEnd?: () => void,
  ) {
    this.reporter = reporter;
    this.opts = {
      ...DEFAULT_PROGRESS_OPTIONS,
      ...opts,
    };

    this.textStack = [];
    this.text = undefined;
    this.title = undefined;

    this.pausedStart = undefined;
    this.pausedElapsed = 0;

    this.startTime = Date.now();
    this.lastRenderTime = Date.now();
    this.lastRenderCurrent = 0;

    this.closed = false;
    this.current = 0;
    this.approximateTotal = false;
    this.total = undefined;
    this.approximateETA = undefined;
    this.onEnd = onEnd;

    this.delay = 60;
    this.renderEvery = 0;

    this.paused = false;

    this.streamToBouncerStart = new Map();
    this.startBouncer();

    this.queueRender(opts.initDelay);
    this.initName(opts.name);
  }

  reporter: Reporter;
  opts: ProgressOptions;

  closed: boolean;
  delay: number;
  renderTimer: undefined | NodeJS.Timeout;

  streamToBouncerStart: Map<ReporterStream, number>;
  bouncerTimer: undefined | NodeJS.Timeout;

  pausedStart: undefined | number;
  pausedElapsed: number;

  onEnd: undefined | (() => void);
  renderEvery: number;
  text: undefined | string;
  title: undefined | string;
  startTime: number;
  lastRenderCurrent: number;
  lastRenderTime: number;
  approximateETA: undefined | number;
  current: number;
  total: undefined | number;
  approximateTotal: boolean;
  textStack: Array<string>;
  paused: boolean;

  initName(name: undefined | string) {
    if (name === undefined) {
      return;
    }

    // TODO fetch approximate total and eta based on `name`
  }

  processRemoteClientMessage(msg: RemoteReporterClientMessage) {
    switch (msg.type) {
      case 'PROGRESS_SET_CURRENT':
        return this.setCurrent(msg.current);

      case 'PROGRESS_SET_TOTAL':
        return this.setTotal(msg.total, msg.approximate);

      case 'PROGRESS_SET_TITLE':
        return this.setTitle(msg.title);

      case 'PROGRESS_SET_TEXT':
        return this.setText(msg.text);

      case 'PROGRESS_PUSH_TEXT':
        return this.pushText(msg.text);

      case 'PROGRESS_POP_TEXT':
        return this.popText(msg.text);

      case 'PROGRESS_SET_APPROXIMATE_ETA':
        return this.setApproximateETA(msg.duration);

      case 'PROGRESS_TICK':
        return this.tick();

      case 'PROGRESS_END':
        return this.end();

      case 'PROGRESS_RESUME':
        return this.resume();

      case 'PROGRESS_PAUSE':
        return this.pause();
    }
  }

  resume() {
    if (!this.paused || this.pausedStart === undefined) {
      return;
    }

    this.pausedElapsed += Date.now() - this.pausedStart;
    this.pausedStart = undefined;
    this.paused = false;
    this.render();
  }

  pause() {
    if (this.paused) {
      return;
    }

    this.pausedStart = Date.now();
    this.paused = true;
    this.render();
  }

  getElapsedTime(): number {
    return Date.now() - this.startTime - this.pausedElapsed;
  }

  getBouncerPosition(stream: ReporterStream): number {
    const start = this.streamToBouncerStart.get(stream);
    if (start === undefined) {
      return 0;
    } else {
      return start;
    }
  }

  startBouncer() {
    const queueTick = () => {
      this.bouncerTimer = setTimeout(tick, BOUNCER_INTERVAL);
    };

    const tick = this.reporter.wrapCallback(() => {
      if (this.paused) {
        queueTick();
        return;
      }

      const elapsedTime = this.getElapsedTime();
      const elapsedFrames = Math.round(elapsedTime / BOUNCER_INTERVAL);

      for (const stream of this.reporter.streams) {
        // We remove the bouncer width from the total columns since we'll append it
        const width = stream.columns - BOUNCER_WIDTH;

        // Position to place the bouncer
        let position = elapsedFrames % width;

        // Every odd complete bounce should reverse direction
        const totalBounces = Math.floor(elapsedFrames / width);
        if (totalBounces % 2 === 1) {
          position = width - position;
        }

        this.streamToBouncerStart.set(stream, position);
      }

      queueTick();
      this.render();
    });

    queueTick();
  }

  setCurrent(current: number) {
    if (this.closed) {
      return;
    }

    this.current = current;

    // Schedule render
    if (this.renderTimer === undefined) {
      this.queueRender();
    }

    if (this.isRenderDue()) {
      this.render();
    }

    // Progress complete
    if (
      this.total !== undefined &&
      this.current >= this.total &&
      !this.opts.persistent
    ) {
      this.end();
    }
  }

  setApproximateETA(duration: number) {
    this.approximateETA = duration;
  }

  setTotal(total: number, approximate: boolean = false) {
    this.total = total;
    this.approximateTotal = approximate;
    this.renderEvery = Math.round(total / 100);
    this.endBouncer();
    this.queueRender();
  }

  setTitle(title: string) {
    this.title = this.reporter.stripMarkup(title);
    this.queueRender();
  }

  setText(text: string) {
    if (this.closed) {
      return;
    }

    this.text = this.reporter.stripMarkup(text);
    this.queueRender();
  }

  pushText(text: string) {
    this.setText(text);
    this.textStack.push(text);
  }

  popText(text: string) {
    // Find
    const {textStack} = this;
    const index = textStack.indexOf(text);
    if (index === -1) {
      throw new Error(`No pushed text: ${text}`);
    }

    // Remove
    textStack.splice(index, 1);

    // Set last
    const last: undefined | string = textStack[textStack.length - 1];
    this.setText(last === undefined ? '' : last);
  }

  tick() {
    this.setCurrent(this.current + 1);
  }

  queueRender(delay: number = this.delay) {
    if (this.closed) {
      // Progress bar has been removed
      return;
    }

    if (this.renderTimer !== undefined) {
      // Render already queued
      return;
    }

    this.renderTimer = setTimeout(
      this.reporter.wrapCallback(() => {
        this.render();
      }),
      delay,
    );
  }

  endBouncer() {
    if (this.bouncerTimer !== undefined) {
      clearTimeout(this.bouncerTimer);
    }
    this.bouncerTimer = undefined;
  }

  endRender() {
    if (this.renderTimer !== undefined) {
      clearTimeout(this.renderTimer);
    }
    this.renderTimer = undefined;
  }

  end() {
    this.closed = true;
    this.endBouncer();
    this.endRender();
    this.reporter.clearLineAll();

    if (this.onEnd !== undefined) {
      this.onEnd();
    }
  }

  // Ensure that we update the progress bar after a certain amount of ticks
  // This allows us to use the progress bar for sync work where the event loop is always blocked
  isRenderDue(): boolean {
    const isDue: boolean =
      this.current > this.lastRenderCurrent + this.renderEvery;
    if (isDue) {
      // We also make sure that we never force update more often than once a second
      // This is to ensure that the progress bar isn't negatively effecting performance
      const timeSinceLastRender: number = Date.now() - this.lastRenderTime;
      return timeSinceLastRender > 1000;
    } else {
      return false;
    }
  }

  isBoldCharacter(i: number, ranges: BoldRanges): boolean {
    for (const [start, end] of ranges) {
      if (start >= i && end <= i) {
        return true;
      }
    }

    return false;
  }

  splitCharacters(str: string, boldRanges: BoldRanges): SplitBar {
    return str.split('').map((char, i) => {
      if (this.isBoldCharacter(i, boldRanges)) {
        return [i, formatAnsi.bold(char)];
      } else {
        return [i, char];
      }
    });
  }

  buildProgressBouncer(stream: ReporterStream, bar: SplitBar): string {
    let start = this.getBouncerPosition(stream);
    let fullBar = '';
    for (const [i, char] of bar) {
      const isBounce = i >= start && i < start + BOUNCER_WIDTH;

      if (isBounce) {
        if (this.paused) {
          fullBar += formatAnsi.inverse(char);
        } else {
          fullBar += formatAnsi.white(formatAnsi.bgYellow(char));
        }
      } else {
        fullBar += char;
      }
    }
    return fullBar;
  }

  buildProgressBar(
    stream: ReporterStream,
    bar: SplitBar,
    total: number,
  ): string {
    const ratio = Math.min(Math.max(this.current / total, 0), 1);

    const completeLength = Math.round(stream.columns * ratio);
    let fullBar = '';
    for (const [i, char] of bar) {
      if (i < completeLength) {
        if (this.paused) {
          fullBar += formatAnsi.inverse(char);
        } else {
          fullBar += formatAnsi.white(formatAnsi.bgGreen(char));
        }
      } else {
        fullBar += char;
      }
    }
    return fullBar;
  }

  buildBar(stream: ReporterStream) {
    const {total, current, text, title} = this;

    // Text ranges that we should make bold
    const boldRanges: BoldRanges = [];

    // Text to prefix to the bar
    let prefix = '';
    if (title !== undefined) {
      prefix += title;

      // Only the title should be bold, not the subtext
      boldRanges.push([0, prefix.length - 1]);
    }
    if (text !== undefined) {
      // Separate a title and it's text with a colon
      if (title !== undefined) {
        prefix += ': ';
      }
      prefix += text;
    }

    // Text to put at the end of the bar
    let suffix = '';

    // Total time since the progress bar was created
    const elapsed = this.getElapsedTime();

    // Time elapsed eg: elapsed 1m5s
    if (this.opts.elapsed) {
      suffix += `elapsed ${humanizeTime(elapsed)} `;
    }

    // Don't bother with a suffix if we haven't completed a single item
    if (current > 0) {
      // How many milliseconds spent per total items
      const averagePerItem = elapsed / current;

      // ETA eg: 1m5s
      if (this.opts.eta) {
        if (
          this.approximateETA !== undefined &&
          elapsed < this.approximateETA
        ) {
          // Approximate ETA
          const left = elapsed - this.approximateETA;
          suffix += `eta ~${humanizeTime(left)} `;
        } else if (total !== undefined) {
          // How many items we have left
          const itemsLeft = total - current;

          // Total estimated time left
          const eta = itemsLeft * averagePerItem;
          suffix += `eta ${humanizeTime(eta)} `;
        } else {
          const ops = Math.round(1000 / averagePerItem);
          suffix += `${humanizeNumber(ops)} op/s `;
        }
      }

      // Counter eg: 5/100
      suffix += `${humanizeNumber(current)}`;
      if (total !== undefined) {
        suffix += '/';
        if (this.approximateTotal) {
          suffix += '~';
        }
        suffix += humanizeNumber(total);
      }
    }

    // Get the full width of the bar. We take off 3 for padding.
    const width = stream.columns - 3;

    // The amount of spaces to put between the title and counter
    const spacerLength = Math.max(0, width - prefix.length - suffix.length);
    const spacer = ' '.repeat(spacerLength);

    // Trim the prefix if it will overflow
    prefix = prefix.slice(0, width - spacerLength - suffix.length);

    // The full raw bar without any coloring
    const raw = ` ${prefix}${spacer} ${suffix}`;

    // Make sure the counter is bold
    boldRanges.push([raw.length - suffix.length, raw.length - 1]);

    // Split the raw bar into an array of formatted characters
    const chars = this.splitCharacters(raw, boldRanges);

    if (total === undefined) {
      return this.buildProgressBouncer(stream, chars);
    } else {
      return this.buildProgressBar(stream, chars, total);
    }
  }

  render() {
    if (this.closed) {
      return;
    }

    this.endRender();

    this.lastRenderCurrent = this.current;
    this.lastRenderTime = Date.now();

    for (const stream of this.reporter.getStreams(false)) {
      if (stream.format === 'ansi') {
        stream.write(escapes.cursorTo(0));
        stream.write(this.buildBar(stream));
      }
    }
  }
}
