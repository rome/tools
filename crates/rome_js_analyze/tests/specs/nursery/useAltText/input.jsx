// invalid
<>
	<input type="image" {...props} />
	<input type="image" {...props} alt={undefined} />
	<input type="image" />
	<input type="image" alt />
</>;

// valid

<>
	<input type="image" {...props} alt />
	<input type="image" alt="This is descriptive!" />
	<input type="image" aria-label="foo" />
	<input type="image" aria-labelledby="id1" />
</>;
