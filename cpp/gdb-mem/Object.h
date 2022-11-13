class Object {
  class InnerObject {
    int z;
  };

public:
  int x = 0;
  int y = 1;

  void dd_outer() { double_delete2(); }
  void double_delete() {
    InnerObject *i = new InnerObject();
    delete i;
    delete i;
  }

  void double_delete2() {
    InnerObject *i = new InnerObject();
    InnerObject *j = i;
    delete i;
    delete j;
  }
};
