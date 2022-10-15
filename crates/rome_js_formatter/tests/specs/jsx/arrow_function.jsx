function BackTopContent(props){
	return (
		<CSSMotion>
			{({ className: motionClassName }) =>
				cloneElement(className => ({
					className
				}))}
		</CSSMotion>
	);
}

function BackTopContent(props){
	return (
		<CSSMotion>
			{({ className: motionClassName }) =>
				cloneElement(className => ({
					className
				}))/*with comment*/}
		</CSSMotion>
	);
}
