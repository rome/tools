// invalid
<input type="submit" accessKey="s" value="Submit" />;
<button accessKey="n">Next</button>;

// valid
<input accessKey={undefined} />;
<input accessKey={null} />;
<button>Next</button>;
<Input accessKey="s" />;
<Button accessKey="n" />;
<RadioGroup.Radio accessKey="a" />;
<context.Provider accessKey={key} />;
