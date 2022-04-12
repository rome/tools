
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
					/>;

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
					/>;

