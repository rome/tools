/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import React = require("react");
import {WebServerClient, WebServerRequest} from "@internal/core";
import Spinner from "./Spinner";
import Button from "./Button";
import {ServerMarker} from "@internal/core/server/Server";
import { VoidCallback } from "@internal/typescript-helpers";

const {css} = require("emotion");
const xterm = require("xterm");
const fit = require("xterm/lib/addons/fit/fit");

xterm.Terminal.applyAddon(fit);

function SubHeading(
	props: {
		children: React.ReactNode;
	},
) {
	return <h2 className={css`
        font-size: 20px;
        font-weight: bold;
        margin: 10px 0;
      `}>
		{props.children}
	</h2>;
}

function Code(
	props: {
		children: React.ReactNode;
	},
) {
	return <span className={css`
        font-family: monospace;
        background-color: rgba(0, 0, 0, 0.5);
        padding: 5px;
      `}>
		{props.children}
	</span>;
}

function Tick(
	props: {
		value: boolean;
	},
) {
	if (props.value) {
		return <span className={css`
          color: green;
          font-weight: bold;
        `}>
			✔
		</span>;
	} else {
		return <span className={css`
          color: red;
          font-weight: bold;
        `}>
			✖
		</span>;
	}
}

function Inspect(
	props: {
		name: string;
		value: unknown;
	},
) {
	const entries = Object.entries(Object(props.value));

	if (entries.length === 0) {
		return <Ommission>
			No
			{props.name}
			specified
		</Ommission>;
	}

	return <ul className={css`
        columns: 340px;
      `}>
		{entries.sort(([a], [b]) => a.localeCompare(b)).map(([key, value]) => {
			let valueElem;

			if (typeof value === "boolean") {
				valueElem = <Tick value={value} />;
			} else {
				valueElem = <Code>
					{JSON.stringify(value)}
				</Code>;
			}

			return <li key={key}
			className={css`
                line-height: 30px;
              `}>
				<Code>
					{key}
				</Code>
				{valueElem}
			</li>;
		})}
	</ul>;
}

function Terminal(
	props: {
		value: string;
	},
) {
	const ref = React.useRef<HTMLDivElement>(null);

	React.useEffect(
		() => {
			var term = new xterm.Terminal({
				convertEol: true,
				fontFamily: "Fira Code",
				fontSize: 16,
				theme: {
					foreground: "#D6DBE5",
					background: "#131313",
					black: "#1F1F1F",
					red: "#F81118",
					green: "#2DC55E",
					yellow: "#ECBA0F",
					blue: "#2A84D2",
					magenta: "#4E5AB7",
					cyan: "#1081D6",
					white: "#D6DBE5",
					brightBlack: "#D6DBE5",
					brightRed: "#DE352E",
					brightGreen: "#1DD361",
					brightYellow: "#F3BD09",
					brightBlue: "#1081D6",
					brightMagenta: "#5350B9",
					brightCyan: "#0F7DDB",
					brightWhite: "#FFFFFF",
				},
			});
			term.open(ref.current);
			term.write(props.value);
			term.fit();
		},
		[props.value],
	);

	return <div ref={ref} />;
}

function Ommission(
	props: {
		children: React.ReactNode;
	},
) {
	return <div className={css`
        font-style: italic;
        text-align: center;
        margin-bottom: 10px;
      `}>
		{props.children}
	</div>;
}

function doesMarkerOverlap(a: ServerMarker, b: ServerMarker): boolean {
	return !(a.start >= b.end || a.start >= b.end);
}

const colors = ["#f39237", "#bf1363", "#0e79b2"];

function Markers(
	{request}: {
		request: WebServerRequest;
	},
) {
	const {markers} = request;

	if (markers.length === 0) {
		return <Ommission>
			No markers created. This requests did not trigger any worker communication.
		</Ommission>;
	}

	const start = request.startTime;

	let end = request.endTime;
	if (end === undefined) {
		end = markers[markers.length - 1].end;
	}

	const rows: Array<Array<ServerMarker>> = [];

	for (const marker of markers.slice().sort((a) => a.end - a.start)) {
		let row: undefined | Array<ServerMarker>;

		// Find row without an overlapping marker
		for (const checkRow of rows) {
			let hasOverlapping = false;

			for (const checkMarker of checkRow) {
				if (doesMarkerOverlap(marker, checkMarker)) {
					hasOverlapping = true;
					break;
				}
			}

			if (!hasOverlapping) {
				row = checkRow;
				break;
			}
		}

		if (row === undefined) {
			row = [];
			rows.push(row);
		}

		row.push(marker);
	}

	rows.sort((a, b) => b.length - a.length);

	const methodToColor: Map<string, string> = new Map();
	let nextColorIndex = 0;

	return <div className={css`
        overflow: auto;
        max-height: 400px;
      `}>
		<div className={css`
          position: relative;
          width: ${end - start}px;
          height: ${rows.length * 40}px;
        `}>
			{rows.map((row, i) => {
				return row.map((marker, i2) => {
					let color = methodToColor.get(marker.facet);
					if (color === undefined) {
						color = colors[nextColorIndex];
						methodToColor.set(marker.facet, color);
						nextColorIndex++;
						if (nextColorIndex === colors.length) {
							nextColorIndex = 0;
						}
					}

					return <div key={`${i}:${i2}`}
					title={marker.label}
					className={css`
                  background-color: ${color};
                  position: absolute;
                  top: ${i * 40}px;
                  height: 30px;
                  line-height: 30px;
                  box-sizing: border-box;
                  left: ${marker.start - start}px;
                  width: ${marker.end - marker.start}px;
                  box-shadow: inset 0 0 0 1px rgba(0, 0, 0, 0.1);
                `} />;
				});
			})}
		</div>
	</div>;
}

function Request(
	{request}: {
		request: WebServerRequest;
	},
) {
	const {query} = request;

	let backgroundColor = "rgba(255, 255, 255, 0.1)";
	let responseElem;
	const {response} = request;
	if (response === undefined) {
		responseElem = <div>
			Response still pending
		</div>;
	} else if (response.type === "SUCCESS") {
		if (response.data === undefined) {
			if (query.noData && !response.hasData) {
				responseElem = <Ommission>
					Response included no data. Most likely due to the noData query option.
				</Ommission>;
			} else {
				responseElem = <Ommission>
					Response included no data
				</Ommission>;
			}
		} else {
			responseElem = <div>
				SUCCESS
			</div>;
		}
		backgroundColor = "rgb(45, 197, 94, 0.5)";
	} else if (response.type === "DIAGNOSTICS") {
		backgroundColor = "rgb(248, 17, 24, 0.5)";
		responseElem = <Ommission>
			TODO PRINT DIAGNOSTICS HERE
		</Ommission>;
	}

	return <div className={css`
        background-color: ${backgroundColor};
        border-radius: 5px;
        padding: 10px;
      `}>
		<SubHeading>
			{query.commandName}
			{request.endTime !== undefined ? <Spinner /> : null}
		</SubHeading>
		<Inspect name="request options"
		value={{
			silent: query.silent,
			noData: query.noData,
			terminateWhenIdle: query.terminateWhenIdle,
		}} />
		<SubHeading>
			Command Flags
		</SubHeading>
		<Inspect name="command flags" value={query.commandFlags} />
		<SubHeading>
			Arguments
		</SubHeading>
		<ol className={css`
          columns: 340px;
        `}>
			{query.args.map((arg, i) => {
				return <li key={i}
				className={css`
                margin-left: 20px;
                list-style: decimal;
                line-height: 30px;
              `}>
					<Code>
						{arg}
					</Code>
				</li>;
			})}
		</ol>
		<SubHeading>
			Response
		</SubHeading>
		{responseElem}
		<SubHeading>
			Markers
		</SubHeading>
		<Markers request={request} />
	</div>;
}

export default function ClientPage(
	{
		client,
		requests,
		goBack,
	}: {
		client: WebServerClient;
		requests: Array<WebServerRequest>;
		goBack: VoidCallback;
	},
) {
	return <>
		<h1 className={css`
          line-height: 42px;
          vertical-align: middle;
          margin-bottom: 20px;
        `}>
			<Button className={css`
            float: left;
          `}
			onClick={goBack}>
				Back
			</Button>
			<span className={css`
            font-size: 30px;
            font-weight: bold;
            margin-left: 20px;
          `}>
				#
				{String(client.id)}
				{client.flags.clientName}
				{" "}
				{client.endTime !== undefined ? <Spinner /> : null}
			</span>
		</h1>
		<SubHeading>
			Flags
		</SubHeading>
		<Inspect name="client flags" value={client.flags} />
		<SubHeading>
			Console
		</SubHeading>
		<Terminal value={client.stdoutAnsi} />
		<SubHeading>
			HTML
		</SubHeading>
		<Terminal value={client.stdoutHTML} />
		<SubHeading>
			Requests
		</SubHeading>
		{requests.map((request) => {
			return <Request key={request.id} request={request} />;
		})}
	</>;
}
