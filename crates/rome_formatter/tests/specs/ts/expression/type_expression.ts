import * as assert from "assert";

type A = string;
type B = number;
type C = null;
type D = undefined;
type E = never;
type F = unknown;
type G = object;
type H = any;
type I = boolean;
type J = bigint;
type K = symbol;
type L = void;
// @ts-ignore
type M = this;
type N = "foo";
type O = true;
type P = false;
type Q = Function;
let a = 2;
type R = typeof a;
type S = 15n;
type T = -15n;
type U = 15;
// @ts-ignore
type V = infer U;
type W = { a: string; b: symbol; c: symbol;d: symbol;e: symbol;f: symbol;g: symbol; };
type X = { a: string; b: symbol; }
type Z = {
    a: string
    b: symbol
}

type OptionsFlags
    <Type> =
    {
  +
        readonly [Property
        in
        keyof
            Type
        as              string]
        -?: boolean;
};

type OptionsFlag
    <A
        extends OptionsFlags<any>
        = any>
    = string;


type TupleA
    = [     string      ]

type TupleB = [   ...string[  ]     ]

type TupleC = [ surname  ?:
    string[],
    ...name: string[],  ]

type TupleD = [
    address: string,
    address2: string,
    address3: string,
    address4: string,
    address5: string,
    surname  ?:
    string[],
    ...name: string[],  ]

type PA = (
    string
    )


type FunctionType = <Aaaaaaaaaaaaaaaaaaaaa,bvvvvvvvvvvvvvvvvvvvvvv,ceeeeeee,deeeeeeeeeeeeee,deeeeeeeeeeeeeee,deeeeeeeeeeeeeeee,deeeeeeeewweeeeee,>(Lorem: string, ipsum: symbol,  dolor: number, sit: boolean, amet: string, consectetur: symbol) => {
    Lorem: string, ipsum: symbol, dolor: number, sit: boolean, amet: string, consectetur: symbol
}

type FunctionTypeB = ( loreum: string )  => string ;

type Indexed = string[
    number
    ]

function test(a: string):
    a is  string   { return true }


type AbstractCompositeThingamabobberFactoryProvider = string;

type ConstructorType = new ( options: { a: string, b: AbstractCompositeThingamabobberFactoryProvider },
) => {};

type Constructor<T> = new(...args: any[]) => T;

function test2(a: string):
    asserts a is string   {  }


type Type01 = 0 extends
    (1 extends 2 ? 3 : 4) ? 5 : 6
    ;