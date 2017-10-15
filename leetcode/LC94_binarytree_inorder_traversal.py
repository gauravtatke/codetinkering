# recursive and iterative inorder traversal of a binary tree


def inorder_recursive(root):
    def traverse(node, nlist):
        if node is None:
            return
        traverse(node.left, nlist)
        nlist.append(node.val)
        traverse(node.right, nlist)
    nlist = []
    traverse(root, nlist)
    return nlist


def inorder_iterativeWithStack(root):
    stack = []
    nlist = []
    if root is None:
        return stack
    curr = root

    while curr is not None or len(stack):
        if curr is not None:
            stack.append(curr)
            curr = curr.left
        else:
            node = stack.pop()
            nlist.append(node.val)
            curr = node.right
    return nlist


def inorder_iterativeWithoutStack(root):
    # only when we have a parent pointer
    # keep a prev pointer to point to last visited node
    # if prev == curr.parent, move further left
    # if prev == curr.left, left subtree is over, print curr and move to right
    # if prev == curr.right, right subtree is over, move up
    nlist = []
    prev = root.parent
    curr = root
    if root is None:
        return nlist
    while curr is not None:
        if prev == curr.parent:
            if curr.left is not None:
                prev = curr
                curr = curr.left
            else:
                prev = curr.left
        if prev == curr.left:
            nlist.append(curr.val)
            if curr.right is not None:
                prev = curr
                curr = curr.right
            else:
                prev = curr.right
        if prev == curr.right:
            prev = curr
            curr = curr.parent
