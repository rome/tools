/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ansiEscapes} from '@romejs/string-markup';
import Reporter from './Reporter';
import {SelectArguments, SelectOption, SelectOptions} from './types';
import readline = require('readline');

function formatShortcut({shortcut}: SelectOption): string {
  if (shortcut === undefined) {
    return '';
  } else {
    return ` <dim>(shortcut ${shortcut})</dim>`;
  }
}

export default async function select<Options extends SelectOptions>(
  reporter: Reporter,
  message: string,
  {
    options,
    defaults = [],
    radio = false,
    yes = false,
  }: SelectArguments<Options>,
): Promise<Set<keyof Options>> {
  const optionNames: Array<keyof Options> = Object.keys(options);
  let optionCount = optionNames.length;
  if (optionCount === 0) {
    return new Set();
  }

  if (yes) {
    return new Set(defaults);
  }

  let prompt = `<dim>❯</dim> <emphasis>${message}</emphasis>`;
  reporter.logAll(prompt);

  if (radio) {
    reporter.info(
      'Use arrow keys and then <emphasis>enter</emphasis> to select an option',
    );
  } else {
    reporter.info(
      'Use arrow keys and <emphasis>space</emphasis> to select or deselect options and then <emphasis>enter</emphasis> to confirm',
    );
  }

  const selectedOptions: Set<keyof Options> = new Set(defaults);
  let activeOption = 0;

  // Set first option if this is a radio
  if (radio && !defaults.length) {
    selectedOptions.add(optionNames[0]);
  }

  function boundActive() {
    activeOption = Math.min(activeOption, optionCount - 1);
    activeOption = Math.max(activeOption, 0);

    if (radio) {
      selectedOptions.clear();
      selectedOptions.add(optionNames[activeOption]);
    }
  }

  // If we aren't a radio then set the active option to the bottom of any that are enabled
  if (!radio) {
    while (selectedOptions.has(optionNames[activeOption])) {
      activeOption++;
    }
  }

  function render() {
    const optionNames = Object.keys(options);
    for (let i = 0; i < optionNames.length; i++) {
      const key = optionNames[i];
      const option = options[key];
      const {label} = option;
      const shortcut = formatShortcut(option);

      let formattedLabel =
        optionNames.indexOf(key) === activeOption
          ? `<underline>${label}</underline>`
          : label;

      let symbol = '';
      if (radio) {
        symbol = selectedOptions.has(key) ? '\u25c9' : '\u25ef';
      } else {
        symbol = selectedOptions.has(key) ? '\u2611' : '\u2610';
      }

      reporter.logAll(
        `  ${symbol} ${formattedLabel}${shortcut}`,
        {
          // Don't put a newline on the last option
          newline: i !== optionNames.length - 1,
        },
      );
    }
  }
  function cleanup() {
    for (let i = 0; i < optionCount; i++) {
      reporter.writeAll(ansiEscapes.eraseLine);

      // Don't move above the top line
      if (i !== optionCount - 1) {
        reporter.writeAll(ansiEscapes.cursorUp());
      }
    }
    reporter.writeAll(ansiEscapes.cursorTo(0));
  }
  function toggleOption(optionName: string) {
    if (selectedOptions.has(optionName)) {
      selectedOptions.delete(optionName);
    } else {
      selectedOptions.add(optionName);
    }
  }

  let onkeypress = undefined;

  const stdin = reporter.getStdin();

  render();

  readline.emitKeypressEvents(stdin);

  if (stdin.isTTY && stdin.setRawMode !== undefined) {
    stdin.setRawMode(true);
  }

  stdin.resume();

  await new Promise((resolve) => {
    function finish() {
      cleanup();

      // Remove initial help message
      reporter.writeAll(ansiEscapes.cursorUp());
      reporter.writeAll(ansiEscapes.eraseLine);

      // Remove initial log message
      reporter.writeAll(ansiEscapes.cursorUp());
      reporter.writeAll(ansiEscapes.eraseLine);

      prompt += ': ';
      if (selectedOptions.size > 0) {
        prompt += Array.from(selectedOptions, (key) => options[key].label).join(
          ', ',
        );
      } else {
        prompt += '<dim>none</dim>';
      }
      reporter.logAll(prompt);

      resolve();
    }
    onkeypress = (
      chunk: Buffer,
      key: {
        name: string;
        ctrl: boolean;
      },
    ) => {
      // Check if this is an option shortcut
      if (!key.ctrl) {
        for (const optionName in options) {
          const {shortcut} = options[optionName];
          if (shortcut === key.name) {
            if (radio) {
              selectedOptions.clear();
              selectedOptions.add(optionName);
              finish();
            } else {
              toggleOption(optionName);
            }
            return;
          }
        }
      }

      switch (key.name) {
        case 'up': {
          activeOption--;
          break;
        }

        case 'down': {
          activeOption++;
          break;
        }

        case 'space': {
          if (!radio) {
            toggleOption((optionNames[activeOption] as string));
          }
          break;
        }

        case 'c': {
          if (key.ctrl) {
            reporter.forceSpacer();
            reporter.warn('Cancelled by user');
            process.exit(1);
          }
          return;
        }

        case 'escape': {
          reporter.forceSpacer();
          reporter.warn('Cancelled by user');
          process.exit(1);
          return;
        }

        case 'return': {
          finish();
          return;
        }

        default:
          return;
      }

      boundActive();
      cleanup();
      render();
    };

    stdin.addListener('keypress', onkeypress);
  });

  if (onkeypress !== undefined) {
    stdin.removeListener('keypress', onkeypress);
  }

  if (stdin.isTTY && stdin.setRawMode !== undefined) {
    stdin.setRawMode(false);
  }

  stdin.pause();

  return selectedOptions;
}
