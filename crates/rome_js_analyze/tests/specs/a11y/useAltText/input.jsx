// invalid
<>
	<input type="image" />
	<input type="image" alt />
	<input type="image" alt={undefined} />
	<input type="image" alt={null} />
</>;

// valid

<>
	<input type="image" alt="This is descriptive!" />
	<input type="image" aria-label="foo" />
	<input type="image" aria-labelledby="id1" />
</>;
