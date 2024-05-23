#include <string>
#include <iostream>

static std::string static_string = "some static string";

int main() {

  // static strings are the same
  std::string local_string = "some static string";
  if (static_string == local_string) {
    std::cout << "static and local are the same" << std::endl;
  }

  // these are not the same
  std::string* static_ptr = &static_string;
  std::string* local_ptr = &local_string;
  if (static_ptr == local_ptr) {
    std::cout << "static and local ptrs are the same" << std::endl; // this does not run
  }

  return 0;
}
