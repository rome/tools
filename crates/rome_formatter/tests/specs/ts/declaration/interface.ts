interface A {}
interface B extends A /** comment **/ {
    something: string
}


interface C<Foo, Bar, Loreum, Ipsum, ItShouldBeLongEnoughToCoverSomeEdgeCases, But, Maybe, Not> extends B {
    something: string
}