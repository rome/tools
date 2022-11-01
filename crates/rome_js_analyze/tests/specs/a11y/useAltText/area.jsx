// invalid

<>
	<area alt={undefined} />
	<area alt={null} />
	<area />
	<area alt />
	<area {...spread} alt />
</>

//valid

<>
	<area alt {...spread} />
	<area aria-label="foo" />
	<area aria-label {...spread} />
	<area aria-labelledby="id1" />
	<area aria-labelledby {...spread} />
	<area alt="This is descriptive!" />
</>
