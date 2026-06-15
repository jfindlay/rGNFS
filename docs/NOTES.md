# rGNFS — Notes

Rolling-context durable framings, decisions, and mental models. Distinct from `docs/PLAN.md`
(current sub-track) and `docs/ROADMAP.md` (project-lifetime view). Captured framings that prove
durable graduate to the ROADMAP Discoveries log at a sub-track boundary.

---

## Transfer-style attacks shard into transfer / structure / solve (2026-06-15, E.H shard)

The small-characteristic index-calculus attack on binary-curve ECDLP decomposes into three
sub-tracks at clean contract seams: **E.H** (GHS/Weil descent) *transfers* an ECDLP on `E/GF(2^m)`
to a DLP on `Jac(C)/GF(2^l)`; **E.J** builds the Semaev summation polynomials; **E.K** runs the
index-calculus *solve*.

The load-bearing observation: E.H's correctness signal is the **homomorphism + logarithm-preservation
relation** (`D_{P+Q} = D_P + D_Q`, and `D_h = k·D_g` for known `k` via the frozen Cantor
`scalar_mul`) — the transfer is verified by *relationship-preservation*, with the solve delegated
downstream. This is the same "build the structure, delegate the attack" idiom seen across the
project:

- **E.I** built the Jacobian group law (Cantor on Mumford pairs) and left the DLP solver to its
  consumer.
- **MOV / SSA** (E.C / E.E) transferred a prime-field ECDLP to a different DLP and verified the
  transfer by the preserved log, calling a solver (NFS-DL / p-adic log) rather than re-deriving one.

Consequence for sharding: a transfer attack freezes the *relocated problem* (here C-GHSDescent)
without ever solving it; its sub-track-close KAT is relationship-preservation, not an end-to-end
break. Bundling the solve into the transfer sub-track couples two independently-delicate designs —
the coupling the inflection-juncture discipline warns against. This framing should inform E.J/E.K
sharding and the E.W cross-attack synthesis.
