#include <vector>
#include <iostream>

int SomeNumber{5};

int GetNumber() { return 41; };

int& GetOther() {
  return SomeNumber;
}

struct SomeType {
  SomeType operator+(int x) {
    return SomeType {Value + x};
  }

  SomeType& operator++() {
    ++Value;
    return *this;
  }

  int Value;
};

struct Resource {
  int _x;
  int x() { return _x; }

  // Default constructor
  Resource(int x) { _x = x;}

  // l-value reference
  Resource(const Resource& Source) {
    std::cout << "Copying resource\n";
    _x = Source._x;
  }

  // r-value reference
  Resource(Resource&& Source) {
    std::cout << "Moving resource\n";
    _x = Source._x;
  }
};

// takes an l-value reference
void takesReference(int &x) {}
void takesConstReference(const int& x) {}

// takes an r-value
void takesDoubleRef(int&& x) {
  std::cout << "That was an r-value\n";
}

int main() {
  std::cout << "l-value and r-value tinkering" << std::endl;
  // r-values non-addressable or temporary 
  // l-values are addressable containers
  int x { 5 }; // x is l-value, 5 is r-value
  int y = 1 + 5; // 1 + 5 is an r-value

  //the compiler can implicitly convert an l-value to an r-value
  int z = y; // y is implicitly converted to an r-value

  // we cannot use an r-value in a place where an l-value is expected
  // 5 = y;

  SomeType Object;

  // l-values can be addressed
  int* xptr = &x;
  std::cout << &GetNumber  << std::endl; // function name is an r-value
  std::cout << &GetOther() << std::endl; // function call depends on return type
  std::cout << &(++Object) << std::endl;

  // r-values cannot be addressed (always true?)
  // std::cout << &5;
  // std::cout << &GetNumber();
  // std::cout << &(Object + 42);

  // cannot convert r -> l, unless we declare const
  // takesReference(5);
  takesConstReference(5);
  takesDoubleRef(5);
  
  // This calls the l-value variation
  takesReference(x);
  
  // This calls the r-value variation
  // std::move is syntactic sugar for static_cast<T&&>
  takesDoubleRef(std::move(x)); // x isn't actually "moved"
  std::cout << "x: " << x << std::endl;

  Resource r1(4);
  Resource r2{r1}; // uses copy-constructor (l-value parameter)

  // if we don't have the r-value definition, this will use copy-constructor
  // as-is we use the move-constructor
  Resource r3{static_cast<Resource&&>(r1)};
}
