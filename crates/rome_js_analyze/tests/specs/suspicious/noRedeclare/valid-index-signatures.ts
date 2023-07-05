type ValidIndexSignatures = {
	a: {
		[index: string]: string;
	};
	b: {
		[index: string]: string;
	};
};

function func1(name: string) {
  return {
    name,
  } as { [name: string]: string };
}

function func2(name: string) {
  return {
    name,
  } as { [notName: string]: string };
}
