auto is_bipartite() -> bool {
    memset(side, -1, sizeof side);
    queue<int> q;
    for (int st = 1; st <= n; ++st) {
        if (side[st] == -1) {
            q.push(st);
            side[st] = 0;
            while (!q.empty()) {
                int v = q.front();
                q.pop();
                for (auto& u : g[v]) {
                    if (side[u] == -1) {
                        side[u] = side[v] ^ 1;
                        q.emplace(u);
                    } else if (side[u] == side[v]) {
                        cout << "NO\n";
                        return 0;
                    }
                }
            }
        }
    }
    return true;
}