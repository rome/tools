delete ident;
class TestClass {
  #member;
  test() {
    delete obj.#member;
    delete func().#member;
    delete obj?.#member;
    delete obj?.inner.#member;
  }
}
