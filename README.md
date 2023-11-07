# tree-to-dot
Simple but educative tool that I use to teach about representing trees and binary tree formed ASTs. Written in rust.

# Example
So given we insert such: [10, 5, 20, 2, 7, 15, 25] then we get this:


![tree representation](https://github.com/LLayta/tree-to-dot/blob/main/img/example.png)


# Slight issue
The API I am using has some weird edge problem whenever a subtree has a single node, it just goes downwards like a skewered tree.

![Skewered representation](https://github.com/LLayta/tree-to-dot/blob/main/img/skewered_tree.png)

A possible fix is to have an edge with a label that represents the direction the node would be going if there was another child node adjecent to it. But I don't feel like implementing that.
