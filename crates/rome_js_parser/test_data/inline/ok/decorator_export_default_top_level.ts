@decorator
export default class Foo { }
@first.field @second @(() => decorator)()
export default class Bar {}
@before
export default @after class Foo { }
 @before
 export default abstract class Foo { }
 @before
 export default @after abstract class Foo { }
