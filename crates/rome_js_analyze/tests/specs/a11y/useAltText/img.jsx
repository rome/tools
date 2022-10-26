// invalid

<>
	<img src="foo" />
	<img src="foo" role="presentation" />
	<img src="foo" role="none" />
	<img alt />
	<img alt={null} />
	<img alt={undefined} />
</>

// valid

<>
	<img alt={undefined} {...spread} />
	<img src="foo" {...spread} />
	<img src="foo" alt="Foo eating a sandwich." />
	<img src="foo" alt={"Foo eating a sandwich."} />
	<img src="foo" alt={altText} />
	<img src="foo" aria-label {...spread} />
    <img src="foo" alt={<><span class="token punctuation">${</span><span class="token variable">person</span><span class="token punctuation">}</span><span class="token string"> smiling</span><span class="token string"></>} />
    <img src="foo" alt="" />

</>;
