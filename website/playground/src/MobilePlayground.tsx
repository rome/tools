import { Tab, TabList, TabPanel, Tabs } from "react-tabs";
import CodeEditor from "@uiw/react-textarea-code-editor";
import { formatWithPrettier, getLanguage } from "./utils";
import { PlaygroundProps } from "./types";
import { PlaygroundSettings } from "./PlaygroundSettings";

export function MobilePlayground(
	{
		isTypeScript,
		setIsTypeScript,
		isJsx,
		setIsJsx,
		sourceType,
		setSourceType,
		indentWidth,
		setIndentWidth,
		indentStyle,
		setIndentStyle,
		quoteStyle,
		setQuoteStyle,
		lineWidth,
		setLineWidth,
		code,
		setCode,
		romeOutput,
	}: PlaygroundProps,
) {
	const language = getLanguage(isJsx, isTypeScript);
	const { cst, ast, formatted_code, formatter_ir, errors } = romeOutput;
	return (
		<div className="p-1">
			<h1 className="p-3 text-xl pb-5">Rome Playground</h1>
			<Tabs>
				<TabList>
					<Tab selectedClassName="bg-slate-300">Input</Tab>
					<Tab selectedClassName="bg-slate-300">Settings</Tab>
					<Tab selectedClassName="bg-slate-300">Formatter Output</Tab>
					<Tab selectedClassName="bg-slate-300">CST</Tab>
					<Tab selectedClassName="bg-slate-300">AST</Tab>
					<Tab selectedClassName="bg-slate-300">Formatter IR</Tab>
					<Tab disabled={errors === ""} selectedClassName="bg-slate-300">
						Errors
					</Tab>
				</TabList>
				<TabPanel>
					<CodeEditor
						value={code}
						language={language}
						placeholder="Enter some code here"
						onChange={(evn) => {
							setCode(evn.target.value);
						}}
						style={{
							fontSize: 12,
							height: "100vh",
							fontFamily:
								"ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
						}}
					/>
				</TabPanel>
				<TabPanel>
					<PlaygroundSettings
						lineWidth={lineWidth}
						setLineWidth={setLineWidth}
						indentStyle={indentStyle}
						setIndentStyle={setIndentStyle}
						indentWidth={indentWidth}
						setIndentWidth={setIndentWidth}
						quoteStyle={quoteStyle}
						setQuoteStyle={setQuoteStyle}
						sourceType={sourceType}
						setSourceType={setSourceType}
						isTypeScript={isTypeScript}
						setIsTypeScript={setIsTypeScript}
						isJsx={isJsx}
						setIsJsx={setIsJsx}
					/>
				</TabPanel>
				<TabPanel>
					<h1>Rome</h1>
					<CodeEditor
						value={formatted_code}
						language={language}
						placeholder="Rome Output"
						style={{
							fontSize: 12,
							height: "40vh",
							overflowY: "scroll",
							fontFamily:
								"ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
						}}
					/>
					<h1>Prettier</h1>
					<CodeEditor
						value={formatWithPrettier(code, {
							lineWidth,
							indentStyle,
							indentWidth,
							language: isTypeScript ? "ts" : "js",
							quoteStyle,
						})}
						key={
							code +
							lineWidth +
							indentStyle +
							indentWidth +
							language +
							quoteStyle
						}
						language={language}
						placeholder="Prettier Output"
						style={{
							fontSize: 12,
							height: "50vh",
							overflowY: "scroll",
							fontFamily:
								"ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
						}}
					/>
				</TabPanel>
				<TabPanel>
					<pre className="h-screen overflow-y-scroll">{cst}</pre>
				</TabPanel>
				<TabPanel>
					<pre className="h-screen overflow-y-scroll">{ast}</pre>
				</TabPanel>
				<TabPanel>
					<pre className="h-screen overflow-y-scroll">{formatter_ir}</pre>
				</TabPanel>
				<TabPanel>
					<pre className="h-screen overflow-y-scroll whitespace-pre-wrap text-red-500 text-xs">
						{errors}
					</pre>
				</TabPanel>
			</Tabs>
		</div>
	);
}
