/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ansiEscapes} from "@internal/cli-layout";
import {AnyMarkup, StaticMarkup, concatMarkup, markup} from "@internal/markup";
import Reporter from "./Reporter";
import {
	SelectArguments,
	SelectOption,
	SelectOptions,
	SelectOptionsKeys,
} from "./types";
import {onKeypress, setRawMode} from "./util";

function formatShortcut({shortcut}: SelectOption): StaticMarkup {
	if (shortcut === undefined) {
		return markup``;
	} else {
		return markup` <dim>(shortcut ${shortcut})</dim>`;
	}
}

export default async function select<Options extends SelectOptions>(
	reporter: Reporter,
	message: AnyMarkup,
	{
		options,
		defaults = [],
		radio = false,
		yes = false,
	}: SelectArguments<Options>,
): Promise<Set<SelectOptionsKeys<Options>>> {
	const optionNames: Array<SelectOptionsKeys<Options>> = [];
	const seenShortcuts: Set<string> = new Set();

	// Verify there's no shortcut collisions and remove empty options
	for (const key in options) {
		const option: undefined | SelectOption = options[key];

		if (option !== undefined) {
			optionNames.push(key);

			const {shortcut} = option;
			if (shortcut !== undefined) {
				if (seenShortcuts.has(shortcut)) {
					throw new Error(`Multiple options have the shortcut ${shortcut}`);
				} else {
					seenShortcuts.add(shortcut);
				}
			}
		}
	}

	let optionCount = optionNames.length;
	if (optionCount === 0) {
		return new Set();
	}

	if (yes) {
		return new Set(defaults);
	}

	let prompt = markup`<dim>❯</dim> <emphasis>${message}</emphasis>`;
	reporter.log(prompt);

	if (radio) {
		reporter.info(
			markup`Use arrow keys and then <emphasis>enter</emphasis> to select an option`,
		);
	} else {
		reporter.info(
			markup`Use arrow keys and <emphasis>space</emphasis> to select or deselect options and then <emphasis>enter</emphasis> to confirm`,
		);
	}

	const selectedOptions: Set<SelectOptionsKeys<Options>> = new Set(defaults);
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
		const optionNames = (Object.keys(options) as Array<SelectOptionsKeys<Options>>);
		for (let i = 0; i < optionNames.length; i++) {
			const key = optionNames[i];
			const option = options[key]!;
			const {label} = option;
			const shortcut = formatShortcut(option);

			let formattedLabel =
				optionNames.indexOf(key) === activeOption
					? markup`<underline>${label}</underline>`
					: label;

			let symbol = "";
			if (radio) {
				symbol = selectedOptions.has(key) ? "\u25c9" : "\u25ef";
			} else {
				symbol = selectedOptions.has(key) ? "\u2611" : "\u2610";
			}

			reporter.log(
				markup`  ${symbol} ${formattedLabel}${shortcut}`,
				{
					// Don't put a newline on the last option
					noNewline: i === optionNames.length - 1,
				},
			);
		}
	}
	function cleanup() {
		for (let i = 0; i < optionCount; i++) {
			reporter.write(ansiEscapes.eraseLine);

			// Don't move above the top line
			if (i !== optionCount - 1) {
				reporter.write(ansiEscapes.cursorUp());
			}
		}
		reporter.write(ansiEscapes.cursorTo(0));
	}
	function toggleOption(optionName: SelectOptionsKeys<Options>) {
		if (selectedOptions.has(optionName)) {
			selectedOptions.delete(optionName);
		} else {
			selectedOptions.add(optionName);
		}
	}

	const stdin = reporter.getStdin();

	render();

	setRawMode(stdin, true);

	await new Promise((resolve) => {
		const keypress = onKeypress(
			reporter,
			(key) => {
				// Check if this is an option shortcut
				if (!key.ctrl) {
					for (const optionName in options) {
						const option: undefined | SelectOption = options[optionName];
						if (option === undefined) {
							continue;
						}

						const {shortcut} = option;
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
					case "up": {
						activeOption--;
						break;
					}

					case "down": {
						activeOption++;
						break;
					}

					case "space": {
						if (!radio) {
							toggleOption(optionNames[activeOption]);
						}
						break;
					}

					case "return": {
						finish();
						return;
					}

					default:
						return;
				}

				boundActive();
				cleanup();
				render();
			},
		);

		function finish() {
			cleanup();

			// Remove initial help message
			reporter.write(ansiEscapes.cursorUp());
			reporter.write(ansiEscapes.eraseLine);

			// Remove initial log message
			reporter.write(ansiEscapes.cursorUp());
			reporter.write(ansiEscapes.eraseLine);

			prompt = markup`${prompt}: `;
			if (selectedOptions.size > 0) {
				prompt = markup`${prompt}${concatMarkup(
					Array.from(selectedOptions, (key) => options[key]!.label),
					markup`, `,
				)}`;
			} else {
				prompt = markup`${prompt}<dim>none</dim>`;
			}
			reporter.log(prompt);

			// Stop listening for keypress
			keypress.finish();
			resolve();
		}
	});

	return selectedOptions;
}
