// invalid
<>
	<input type="image" />
	<input type="image" alt />
</>;

// valid

<>
	<input type="image" {...props} alt />
	<input type="image" {...props} /> {/* Skipping*/}
	<input type="image" {...props} alt={undefined} /> {/* Skipping*/}
	<input type="image" alt="This is descriptive!" />
	<input type="image" aria-label="foo" />
	<input type="image" aria-labelledby="id1" />
</>;
