class Node:
    def __init__(self, value, left = None, right = None):
        self.value = value
        self.left = left
        self.right = right

def boustrophedon(node):
    left_to_right = True
    output = []
    layer = [node]

    while layer:
        buffer = []
        next_layer = []
        for n in layer:
            if n: buffer.append(n.value)
            if n.left: next_layer.append(n.left)
            if n.right: next_layer.append(n.right)
        buffer.reverse() if not left_to_right else None
        output.extend(buffer)
        layer = next_layer
        left_to_right = not left_to_right

    return output

if __name__ == "__main__":
    tree_one = Node('a', 
                    Node('b', Node('d'), Node('e')),
                    Node('c', Node('f'), Node('g')))
    assert boustrophedon(tree_one) == list('acbdefg')

    tree_two = Node('p',
                    Node('o',
                         Node('g', Node('n'), Node('o')),
                         Node('n', Node('i'), Node('t'))),
                    Node('r',
                         Node('o', Node('a'), Node('c')),
                         Node('s', Node('i'), Node('t'))))
    assert boustrophedon(tree_two) == list('prognostication')
