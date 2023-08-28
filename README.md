# tree-to-dot
Simple but educative tool that I use to teach about representing trees and binary tree formed ASTs. Written in rust.

# Example
So given we insert such: [10, 5, 20, 2, 7, 15, 25] then we get this:


![tree representation](https://github.com/LLayta/tree-to-dot/blob/main/img/example.png)


# Slight issue
The graphviz API I use doesn't specialize for binary trees it specializes for something called digraphs which are in an essence just binary trees but they don't have a sense of "left and right" like binary trees do. So why is this an issue? It's an issue because for nodes that have a single edge it won't actually place the node in a direction, it'll just put it straight down. Here's an example of that:

![Skewered representation](https://github.com/LLayta/tree-to-dot/blob/main/img/skewered_tree.png)

A possible fix is to have an edge with a label that represents the direction the node would be going if there was another child node adjecent to it. But I don't feel like implementing that.
