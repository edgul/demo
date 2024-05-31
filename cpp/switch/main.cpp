#include <iostream>

void print_case(int i) {
      std::cout << "case " << i << std::endl;
}

int main() {
  auto var = 2;
  switch (var) {
    case 1:
    {
      print_case(1);
    }
    case 2:
    {
      print_case(2);
    }
    case 3:
    {
      print_case(3);
      break;
    }
    case 4:
    {
      print_case(4);
    }

      
  }
  return 0;
}
