// invalid

<>
	<area alt={undefined} />
	<area />
	<area alt />
</>

//valid

<>
	<area {...props} />
	<area {...props} alt />
	<area aria-label="foo" />
	<area aria-labelledby="id1" />
	<area alt="This is descriptive!" />
</>
