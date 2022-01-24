delete (ident);
delete ((ident));
delete (obj.key, ident);
class TestClass {
  #member;
  #key;
  test() {
    delete (obj.#member);
    delete (func().#member);
    delete (obj?.#member);
    delete (obj?.inner.#member);
    delete (obj.key, obj.#key);
  }
}
