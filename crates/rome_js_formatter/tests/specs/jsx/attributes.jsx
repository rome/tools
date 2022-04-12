<div className="divide-y divide-slate-300">
	<h1 className="p-4 text-xl">Rome Playground</h1>
	<div>
		<LineWidthInput lineWidth={lineWidth} setLineWidth={setLineWidth} />
		<IndentStyleSelect indentWidth={indentWidth}
			setIndentWidth={


			setIndentWidth


		}
			indentStyle={indentStyle}

											 setIndentStyle={setIndentStyle}
		/>
		<QuoteStyleSelect
			quoteStyle={quoteStyle}

			setQuoteStyle={setQuoteStyle}
		/>
		<SourceTypeSelect
			isTypeScript={isTypeScript}
			setIsTypeScript={setIsTypeScript}
			isJsx={isJsx}
			setIsJsx={setIsJsx}
			sourceType={sourceType}
			setSourceType={setSourceType}
		/>
	</div>
	<div className="box-border flex h-screen divide-x divide-slate-300">
		<div className="w-1/2 p-5">
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
		</div>
		<div className="w-1/2 p-5 flex flex-col">
			<Tabs>
				<TabList>
					<Tab selectedClassName="bg-slate-300">Formatter</Tab>
					<Tab selectedClassName="bg-slate-300">CST</Tab>
					<Tab selectedClassName="bg-slate-300">AST</Tab>
					<Tab selectedClassName="bg-slate-300">Formatter IR</Tab>
					<Tab
						disabled={errors === ""}
						selectedClassName="bg-slate-300">
						Errors
					</Tab>
				</TabList>
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
									<pre className="h-screen overflow-y-scroll">
										{formatter_ir}
									</pre>
				</TabPanel>
				<TabPanel>
									<pre className="h-screen overflow-y-scroll whitespace-pre-wrap text-red-500 text-xs">
										{errors}
									</pre>
				</TabPanel>
			</Tabs>
		</div>
	</div>
</div>
