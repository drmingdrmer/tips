# Inconsistent partial order pseudo time in Raft

In Raft, the pseudo time `T: (term: u64, voted_for: Option<NodeId>)` is
partially ordered. This means that within the same term, if a node has already
accepted Candidate A, it cannot accept Candidate B(handling ReqeustVote RPC):

`(term=1, voted_for=Some(A)) ≯ (term=1, voted_for=Some(B))`

However, once Candidate A finishes the election and a quorum accepts it, the
pseudo time `T` of A becomes greater(handling AppendEntries RPC):

`(term=1, voted_for=Some(A)) > (term=1, voted_for=Some(B))`

As you can see, the order of `T` is inconsistent, which is problematic. This
rule is often overlooked in Raft. To clarify, we can embed the `quorum_accepted`
status into the pseudo time, forming:

`T': (term: u64, voted_for: Option<NodeId>, quorum_accepted: bool)`.

This adjustment ensures that the partial order of pseudo time `T'` is always
consistent. And quorum accepted `T'` become totally ordered:

`T'₁ > T'₂`, `T'₁ == T'₂`, or `T'₁ < T'₂`.
