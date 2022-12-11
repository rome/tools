const number: number = 1;

function greet(): string {}

function greet2(): Array<string> {}

function greet3(): string[] {}

function greet4(): Array<Array<string>> {}

function greet5(): Array<string[]> {}

function greet6(param: Array<string>): Array<string> {}

class Greeter {
  message: string;
}

class Greeter2 {
  message: Array<string>;
}

class Greeter3 {
  message: string[];
}

class Greeter4 {
  message: Array<Array<string>>;
}

class Greeter5 {
  message: Array<string[]>;
}

interface Greeter6 {
  message: string;
}

interface Greeter7 {
  message: Array<string>;
}

interface Greeter8 {
  message: string[];
}

interface Greeter9 {
  message: Array<Array<string>>;
}

interface Greeter10 {
  message: Array<string[]>;
}

type obj = {
  message: string;
};

type obj2 = {
  message: Array<string>;
};

type obj3 = {
  message: string[];
};

type obj4 = {
  message: Array<Array<string>>;
};

type obj5 = {
  message: Array<string[]>;
};

type obj6 = {
  message: string | number;
};

type obj7 = {
  message: string | Array<string>;
};

type obj8 = {
  message: string | string[];
};

type obj9 = {
  message: string | Array<Array<string>>;
};

type obj10 = {
  message: string & number;
};

type obj11 = {
  message: string & Array<string>;
};

type obj12 = {
  message: string & string[];
};

type obj13 = {
  message: string & Array<Array<string>>;
};
