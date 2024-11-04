
def adjlist(n, edges, directed=False):
    adj = [set() for _ in range(n + 1)]

    if directed:
        for s, t in edges:
            adj[s].add(t)
    else:
        for s, t in edges:
            adj[s].add(t)
            adj[t].add(s)

    return adj


def adjcost(n, edges, directed=False):
    adj = [set() for _ in range(n + 1)]

    inf = float('inf')
    cost = [[inf] * (n + 1) for _ in range(n + 1)]

    if directed:
        for s, t, c in edges:
            adj[s].add(t)
            cost[s][t] = c
    else:
        for s, t, c in edges:
            adj[s].add(t)
            adj[t].add(s)
            cost[s][t] = c
            cost[t][s] = c

    return adj, cost


def tpsort(n, adj, origin=0):
    """トポロジカルソート
       n が大きい時は sys.setrecursionlimit で
       再帰の上限を増やしておくこと。
       import sys
       sys.setrecursionlimit(10**6)
    """
    seen = [False] * (n + 1)
    order = []

    def dfs(v):
        seen[v] = True
        for t in adj[v]:
            if not seen[t]:
                dfs(t)
        order.append(v)

    vs = range(n) if origin == 0 else range(1, n + 1)
    for v in vs:
        if not seen[v]:
            dfs(v)

    order.reverse()
    return order


def ssspbfs(n, s, adj):
    """BFS による単一始点最短路
    """
    from collections import deque
    inf = float('inf')

    dist = [inf] * (n + 1)
    dist[s] = 0
    todo = deque([s])
    while todo:
        v = todo.pop()
        for t in adj[v]:
            if dist[v] + 1 < dist[t]:
                dist[t] = dist[v] + 1
                todo.appendleft(t)
    return dist


class UnionFind:
    def __init__(self, n):
        # 0/1-indexed 両対応のため n + 1 で確保する
        self._parents = [-1] * (n + 1)
        self._sizes = [1] * (n + 1)

    def root(self, x):
        if self._parents[x] == -1:
            return x
        else:
            self._parents[x] = self.root(self._parents[x])
            return self._parents[x]

    def unite(self, x, y):
        rx = self.root(x)
        ry = self.root(y)

        if rx == ry:
            return False
        elif self._sizes[rx] < self._sizes[ry]:
            self._parents[rx] = ry
            self._sizes[ry] += self._sizes[rx]
        else:
            self._parents[ry] = rx
            self._sizes[rx] += self._sizes[ry]

        return True

    def connected(self, x, y):
        return self.root(x) == self.root(y)

    def size(self, x):
        return self._sizes[self.root(x)]

    def __repr__(self):
        bins = {}
        for i, p in enumerate(map(self.root, range(len(self._parents)))):
            if p not in bins:
                bins[p] = set()
            bins[p].add(i)
        return f"UnionFind({', '.join(map(str, bins.values()))})"
