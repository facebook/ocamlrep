# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

load("@prelude//utils:utils.bzl", "expect")

def topo_sort(graph: dict[typing.Any, list[typing.Any]]) -> list[typing.Any]:
    """
    Topo-sort the given graph.
    """

    in_degrees = {node: 0 for node in graph}
    for _node, deps in graph.items():
        for dep in dedupe(deps):
            in_degrees[dep] += 1

    queue = []

    for node, in_degree in in_degrees.items():
        if in_degree == 0:
            queue.append(node)

    ordered = []

    for _ in range(len(in_degrees)):
        if len(queue) == 0:
            fail_cycle(graph)

        node = queue.pop()
        ordered.append(node)

        for dep in graph[node]:
            in_degrees[dep] -= 1
            if in_degrees[dep] == 0:
                queue.append(dep)

    expect(not queue, "finished before processing nodes: {}".format(queue))
    expect(len(ordered) == len(graph), "missing or duplicate nodes in sort")

    return ordered

def post_order_traversal(graph: dict[typing.Any, list[typing.Any]]) -> list[typing.Any]:
    """
    Performs a post-order traversal of `graph`.
    """

    out_degrees = {node: 0 for node in graph}
    rdeps = {node: [] for node in graph}
    for node, deps in graph.items():
        for dep in dedupe(deps):
            out_degrees[node] += 1
            rdeps[dep].append(node)

    queue = []

    for node, out_degree in out_degrees.items():
        if out_degree == 0:
            queue.append(node)

    ordered = []

    for _ in range(len(out_degrees)):
        if len(queue) == 0:
            fail_cycle(graph)

        node = queue.pop()
        ordered.append(node)

        for dep in rdeps[node]:
            out_degrees[dep] -= 1
            if out_degrees[dep] == 0:
                queue.append(dep)

    expect(not queue, "finished before processing nodes: {}".format(queue))
    expect(len(ordered) == len(graph), "missing or duplicate nodes in sort")

    return ordered

def fail_cycle(graph: dict[typing.Any, list[typing.Any]]) -> typing.Never:
    cycle = find_cycle(graph)
    if cycle:
        fail(
            "cycle in graph detected: {}".format(
                " -> ".join(
                    [str(c) for c in cycle],
                ),
            ),
        )
    fail("expected cycle, but found none")

def find_cycle(graph: dict[typing.Any, list[typing.Any]]) -> list[typing.Any] | None:
    visited = {}
    OUTPUT = 1
    VISIT = 2
    current_parents = []
    work = [(VISIT, n) for n in graph.keys()]
    for _ in range(2000000000):
        if not work:
            break
        kind, node = work.pop()
        if kind == VISIT:
            if node not in visited:
                visited[node] = True
                current_parents.append(node)

                work.append((OUTPUT, node))
                for dep in graph[node]:
                    if dep in current_parents:
                        return current_parents + [dep]
                    if dep not in visited:
                        work.append((VISIT, dep))
        else:
            current_parents.pop()

    return None

def breadth_first_traversal(
        graph_nodes: dict[typing.Any, list[typing.Any]],
        roots: list[typing.Any]) -> list[typing.Any]:
    """
    Like `breadth_first_traversal_by` but the nodes are stored in the graph.
    """

    def lookup(x):
        return graph_nodes[x]

    return breadth_first_traversal_by(graph_nodes, roots, lookup)

def breadth_first_traversal_by(
        graph_nodes: [dict[typing.Any, typing.Any], None],
        roots: list[typing.Any],
        get_nodes_to_traverse_func) -> list[typing.Any]:
    """
    Performs a breadth first traversal of `graph_nodes`, beginning
    with the `roots` and queuing the nodes returned by`get_nodes_to_traverse_func`.
    Returns a list of all visisted nodes.

    get_nodes_to_traverse_func(node: '_a') -> ['_a']:

    Starlark does not offer while loops, so this implementation
    must make use of a for loop. We pop from the end of the queue
    as a matter of performance.
    """

    # Dictify for O(1) lookup
    visited = {k: None for k in roots}

    queue = visited.keys()

    for _ in range(len(graph_nodes) if graph_nodes else 2000000000):
        if not queue:
            break
        node = queue.pop()
        if graph_nodes:
            expect(node in graph_nodes, "Expected node {} in graph nodes", node)
        nodes_to_visit = get_nodes_to_traverse_func(node)
        for node in nodes_to_visit:
            if node not in visited:
                visited[node] = None
                queue.append(node)

    expect(not queue, "Expected to be done with graph traversal queue.")

    return visited.keys()
