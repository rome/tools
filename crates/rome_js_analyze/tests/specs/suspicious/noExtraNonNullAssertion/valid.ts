const bar = case1!.bar;

function case2(bar: number | undefined) {
  const bar1: number = bar!;
}

function case3(bar?: { n: number }) {
  return bar?.n;
}

checksCounter?.case4!.trim();

function case5(key: string | null) {
  const obj = {};
  return obj?.[key!];
}
