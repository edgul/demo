#include <string>
#include <assert.h>
#include <iostream>

// trying to get this under leet code's timeout period by not copying
template <typename Iter>
int isPalindromeHelper(const std::string& s, Iter start, Iter end) {
  std::string sub(start, end);
  std::cout << "checking: " << sub << std::endl;
  auto i = start;
  auto j = end;
  while (i != j && i < s.end() && j >= s.begin()) {
    if (*i != *j) {
      return 1;
    }
    i++;
    j--;
  }
  return end - start;
}

int isPalindrome(const std::string& s) {
  return isPalindromeHelper(s, s.begin(), s.end());
}

std::string longestPalindrome(std::string s) {
  std::string result = "";
  for (auto c = s.begin(); c < s.end(); c++) {
    for (auto j = c; j < s.end(); j++) {
      int len = isPalindromeHelper(s, c, j+1);
      if (len && len > result.length()) {
        result = std::string(c, j+1); 
        std::cout << "new longest palindrome: " << result << std::endl;
      } 
    } 
  } 
  return result;
}

int main() {
  assert(isPalindrome("") == 0);
  assert(isPalindrome("a") == 1);
  assert(isPalindrome("ab") == 1);
  assert(isPalindrome("aa") == 2);
  assert(isPalindrome("aba") == 3);
  assert(isPalindrome("abba") == 4);
  assert(isPalindrome("abbac") == 4);
  assert(isPalindrome("cabba") == 4);
  assert(longestPalindrome("abcabc") == "a");
  assert(longestPalindrome("aaaaa") == "aaaaa");
  assert(longestPalindrome("aaaa") == "aaaa");
  assert(longestPalindrome("aabaa") == "aabaa");
  assert(longestPalindrome("abcdef") == "a");
  assert(longestPalindrome("abcdefa") == "a");
  assert(longestPalindrome("pwwkew") == "ww");
  assert(longestPalindrome("baaab") == "baaab");
  assert(longestPalindrome("abaaab") == "baaab");
  assert(longestPalindrome("baaaba") == "baaab");
  assert(longestPalindrome("baaabacdefghi") == "baaab");
  assert(longestPalindrome("cdefghibaaab") == "baaab");
  return 0;
}
