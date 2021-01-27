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
