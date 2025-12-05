#pragma once
#include <fstream>
#include <iostream>
#include <string>
#include "binary-tree.h"
#include "helpers.h"
#include "stack.h"

void writeToFile(std::string& s, std::string& filename) {
  std::ofstream out(filename);
  out << s;
  out.close();
}

void writeStackToFile() {
  Stack s;
  s.push(1);
  s.push(2);
  s.push(3);
  Node* n = convertStackToTree(s);
  std::string serialization = serialize(n);
  std::string filename = std::string("tree.txt");
  writeToFile(serialization, filename);
}

std::string readFromFile(std::string& filename) {
  std::ifstream fileStream(filename);
  std::string line("");
  std::string result("");
  while (std::getline(fileStream, line)) {
    result += line; 
  }
  return result;
}

void testReadFromTrees() {
  std::string filename = std::string("test-data/tree.txt");
  std::string s = readFromFile(filename);
  size_t index = 0;
  Node* n = deserialize(s, index);
  assert(n->value == 1);
  assert(n->left->value == 2);
  assert(n->right == nullptr);
  assert(n->left->left->value == 3);
  assert(n->left->right == nullptr);
  assert(n->left->left->right == nullptr);
}

void testReadFromTrees1() {
  std::string filename = std::string("test-data/tree1.txt");
  std::string s = readFromFile(filename);
  size_t index = 0;
  Node* n = deserialize(s, index);
  assert(n->value == 1);
  assert(n->left == nullptr);
  assert(n->right->value == 2);
  assert(n->right->left->value == 3);
  assert(n->right->right == nullptr);
  assert(n->right->left->right == nullptr);
}

void testReadFromTrees2() {
  std::string filename = std::string("test-data/tree2.txt");
  std::string s = readFromFile(filename);
  size_t index = 0;
  Node* n = deserialize(s, index);
  assert(n->value == 1);
  assert(n->left->value == 2);

  assert(n->left->left->value == 3);
  assert(n->left->left->right == nullptr);
  assert(n->left->left->left == nullptr);

  assert(n->left->right->value == 4);
  assert(n->left->right->right == nullptr);
  assert(n->left->right->left == nullptr);

  assert(n->right->value == 5);
  assert(n->right->left == nullptr);
  assert(n->right->right == nullptr);

  assert(n->left->right->value == 4);
}
