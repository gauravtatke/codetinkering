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


def morris_traversal(root):
    # 1. start from curr = root.
    # 2. if curr.left is None, process curr, and move to right
    # 3. else make curr as right child of right most node in curr's left
    # subtree and move to left
    # in this algo, tree is changed but it regain its structure once the algo
    # is finished
    if root is None:
        return None
    curr = root
    nlist = []
    while curr is not None:
        if curr.left is None:
            # process curr and move to right
            nlist.append(curr)
            curr = curr.right
        else:
            # find the rightmost node in curr's left subtree
            pre = curr.left
            while pre.right is not None and pre.right != curr:
                pre = pre.right
            if pre.right is None:
                # pre is the right most tree
                pre.right = curr
                curr = curr.left
            else:
                # pre.right == curr, left subtree is already traverse
                pre.right = None  # set is back to None
                nlist.append(curr)  # process curr node
                curr = curr.right
