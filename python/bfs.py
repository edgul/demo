from Node import * 
from typing import List
from typing import TypedDict


class NodeAndDepth:
    def __init__(self, node : Node, depth : int):
        self.node = node
        self.depth = depth


def bfs(explore):
    node_and_depth = explore.pop(0)
    node = node_and_depth.node
    depth = node_and_depth.depth
    print(depth * " " + str(node.value))

    if node.left != None: 
        explore.append(NodeAndDepth(node.left, depth + 1))
    if node.right != None:
        explore.append(NodeAndDepth(node.right, depth + 1))
    if len(explore) != 0:
        bfs(explore)
    
def dfs(node, depth):
    print(depth * " " + str(node.value))

    if node.left != None: 
        dfs(node.left, depth + 1)
    if node.right != None:
        dfs(node.right, depth + 1)
    
def main():
    head = BinaryNode(1, BinaryNode(2, BinaryNode(0)), BinaryNode(3, BinaryNode(4, BinaryNode(5))))

    print("bfs:")
    bfs([NodeAndDepth(head, 0)])

    print("\ndfs:")
    dfs(head,0)

main()
