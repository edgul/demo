#include <string>
#include <array>
#include <assert.h>

int lengthOfLongestSubstring(std::string s) {
  int max = 0;
  for (auto c = s.begin(); c != s.end(); ++c) {
      std::array<bool,127> chars{0}; // just handle ASCII chars
      auto i = c;
      int index = *i;
      int count = 0;
      while (index < 127 && !chars[index] && i!=s.end()) {
        chars[index] = 1;
        count++;
        i++;
        index = *i; 
      }
      if (count > max) max = count;
  }
  return max;
}

int main() {
  assert(lengthOfLongestSubstring("abcabc") == 3);
  assert(lengthOfLongestSubstring("aaaaa") == 1);
  assert(lengthOfLongestSubstring("aabaa") == 2);
  assert(lengthOfLongestSubstring("abcdef") == 6);
  assert(lengthOfLongestSubstring("abcdefa") == 6);
  assert(lengthOfLongestSubstring("pwwkew") == 3);

  return 0;
}
