type test = string;

type T1 = test extends string ? test extends number ? unknown : unknown : undefined;

type T2 = test extends string ?  unknown : test extends number ? undefined : undefined;