#!/usr/bin/python3

from __future__ import annotations

class Node:
    def __init__(self, value : int, node : Node):
        self.value = value
        self.next = node 

    def print(node : Node):
        print(node.value)    
        if (node.next != None):
            Node.print(node.next)

class BinaryNode:
    def __init__(self, value : int, left : Node = None, right : Node = None):
        self.value = value
        self.left = left
        self.right = right

    
