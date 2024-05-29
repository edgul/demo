#include <iostream>

int main() {

  int x;
  if (true && (x = 3) > 0){ // interesting, seems to work
    // this section runs and
    // x == 3
    std::cout << "x = " << x << std::endl;
  } 

  // though like this does not
  // if (true && (int y = 3) > 0){ // interesting, seems to work
  //   std::cout << "y = " << y << std::endl; 
  // }

  return 0;
}
