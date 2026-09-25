// TODO: We can easily improve on the actual number of local cliffords that will be
// required, e.g., if a node is in the neighbourhood of two subsequent LC operations, then
// this node gets two subsequent HSH gates, which becomes HSH^2 = HZH = X.

enum Clifford {
    X,
    Y,
    Z,
    H,
    S,
}

struct CliffordStack {
    stack: Vec<Clifford>,
}

// ...
