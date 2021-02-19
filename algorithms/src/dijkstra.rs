fn main() {
    /*
    export class DirectedWeightedGraph<K, V> {
        private nodes: Map<K, Map<K, V>> = new Map();

        add(from: K, to: K, weight: V) {
            let siblings = this.nodes.get(from);
            if (siblings === undefined) {
            siblings = new Map();
            this.nodes.set(from, siblings);
            }
            siblings.set(to, weight);

            if (!this.nodes.has(to)) {
            this.nodes.set(to, new Map());
            }
        }

        getSiblings(key: K) {
            return this.nodes.get(key);
        }
    }


    // Dijkstra's Shortest Path First algorithm
    // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm

    import { DirectedWeightedGraph } from "./structs/DirectedWeightedGraph";

    (() => {
    const graph = new DirectedWeightedGraph<string, number>();

    graph.add("start", "a", 6);
    graph.add("start", "b", 2);
    graph.add("a", "fin", 1);
    graph.add("b", "a", 3);
    graph.add("b", "fin", 5);

    const path = findPath(graph, "start", "fin");

    console.log(path);
    })();

    function findPath(
    graph: DirectedWeightedGraph<string, number>,
    from: string,
    to: string
    ) {
    const startSiblings = graph.getSiblings(from)!;

    const costs = new Map(startSiblings);
    costs.set(to, Infinity);

    const parents = new Map<string, string>();
    for (const key of startSiblings.keys()) {
        parents.set(key, from);
    }

    const processed = new Set<string>();

    let node = findLowestCostNode(costs, processed);

    while (node !== undefined) {
        const cost = costs.get(node)!;
        const siblings = graph.getSiblings(node)!;
        for (const n of siblings.keys()) {
        const newCost = cost + siblings.get(n)!;
        if (!costs.has(n) || costs.get(n)! > newCost) {
            costs.set(n, newCost);
            parents.set(n, node);
        }
        }
        processed.add(node);
        node = findLowestCostNode(costs, processed);
    }

    let end = to;
    const path = [end];

    while (end !== from) {
        const n = parents.get(end)!;
        end = n;
        path.push(n);
    }

    return path.reverse();
    }

    function findLowestCostNode(
    costs: Map<string, number>,
    processed: Set<string>
    ) {
    let lowestWeight = Infinity;
    let lowestNode;

    for (const [node, weight] of costs.entries()) {
        if (weight < lowestWeight && !processed.has(node)) {
        lowestNode = node;
        lowestWeight = weight;
        }
    }

    return lowestNode;
    }

    */
}
