
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

					let a = <test aVeryLongAttributeName={"WithAVeryLongValuethat exceeds the line width, what happens with ithis"} />;

					<test {...WithAVeryLongFunctionthat_exceeds_the_line_width_what_happens_with_ithis()} />;
					<div {...["Chungking Express", "Fallen Angels", "In the Mood for Love", "Days of Living Wild", "Happy Together"]}/>;


//  https://github.com/rome/tools/issues/2944
<div className={asdf asdf} />;
<div className={asdf
	/* comment */ asdf } />;

// Wrapping JSX in attribute
const a = <Popconfirm
		title={
			(
				<span className="popconfirm-test">
		Are you sure delete this task? let
	</span>
			)
		}
		okText="Yes"
		cancelText="No"
		mouseEnterDelay={0}
		mouseLeaveDelay={0}
		onVisibleChange={onVisibleChange}
	>
		<span>Delete</span>
	</Popconfirm>
;

<OtherComponent
	value={
		new Set(veryLongConditionZzzzzzzzzzzzzzzzzveryLongConditionZzzzzzzzzzzzzzzzzveryLongConditionZzzzzzzzzzzzzzzzz)
	}
/>
