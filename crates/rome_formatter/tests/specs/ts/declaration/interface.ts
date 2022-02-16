interface A {}
interface B extends A /** comment **/ {
    something: string
}


interface C<Foo, Bar, Loreum, Ipsum, ItShouldBeLongEnoughToCoverSomeEdgeCases, But, Maybe, Not> extends B {
    something: string
}

// @ts-ignore
interface D extends B<string, symbol>, F<string, symbol>, G<string, number, symbol>, H<string, number, symbol> {

}
// @ts-ignore
interface D extends B<string, symbol>, F<string, symbol> {

}