class TestClass {
  #member;
  #key;
  test() {
    delete obj.key;
    delete obj.key;
    delete obj.#member.key;
    delete obj.#member.key;
    delete func().#member.key;
    delete func().#member.key;
    delete obj?.#member.key;
    delete (obj?.#member).key;
    delete obj?.inner.#member.key;
    delete (obj?.inner.#member).key;
    delete obj[key];
    delete obj[key];
    delete obj.#member[key];
    delete obj.#member[key];
    delete func().#member[key];
    delete func().#member[key];
    delete obj?.#member[key];
    delete (obj?.#member)[key];
    delete obj?.inner.#member[key];
    delete (obj?.inner.#member)[key];
    delete (obj.#key, obj.key);
    delete (#key in obj);
  }
}
