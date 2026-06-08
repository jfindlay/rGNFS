# rGNFS — Mathematical Textbook

*A standing mathematical survey of discrete-logarithm and integer-factorisation algorithms.*
*Maths-first, code-second, learnable on its own.*

---

## C-Textbook: Documentation-Register Contract

This section states the contract every chapter of this textbook obeys. It is frozen at session T.0
and consumed by every later chapter (T.G, T.D, T.E, T.S, T.Z, and the `*.W` code-tours as a
recommendation). A later chapter that cannot be written within this register must surface the
constraint as a discovery and flex C-Textbook at an inflection review — not silently raise the level.

### Audience floor

The reader has a full undergraduate mathematics background:

- **Proofs.** Comfortable reading and writing $\varepsilon$–$\delta$ arguments, induction,
  contradiction, and direct proof. Knows what it means for a proof to be rigorous.
- **Algebra.** Groups, rings, fields, homomorphisms, quotients, ideals. Polynomial rings. The
  isomorphism theorems. Finite fields $\mathbb{F}_q$. Elliptic curves as groups (the chord-and-tangent
  law, the group axioms, the order of a point). Dedekind domains and unique factorisation of ideals
  are introduced in the Prerequisites chapter and used in the GNFS chapter.
- **Analysis.** Big-$O$ and little-$o$ notation. Limits. The prime number theorem (statement only).
  Logarithms and exponentials. The $L$-notation for subexponential functions is introduced in the
  Prerequisites chapter.
- **Probability.** Discrete probability spaces, expectation, the birthday paradox. The density of
  smooth numbers (Canfield–Erdős–Pomerance) is stated and motivated in the Prerequisites chapter.
- **Logic.** Propositional and first-order logic at the level of a first proof-writing course.
  Computability and complexity at the level of knowing what $\mathbf{P}$, $\mathbf{NP}$, and
  polynomial-time reductions are.

**Nothing beyond this floor is assumed.** Anything beyond is built up in text or cited with a
precise reference. A later chapter that genuinely requires graduate-level background (e.g. étale
cohomology for the Weil conjectures, or the theory of $p$-adic representations) must surface this
as a C-Textbook flex, not silently assume it.

### Depth

**Survey with proof-sketch depth.** Every key theorem is stated precisely and motivated; proofs are
sketches with citations to the full proof, *except* where the proof is the pedagogical payoff. The
designated payoff proofs — where the proof *is* the lesson — are given in full:

- The $L$-notation subexponentiality derivation for GNFS (T.G chapter).
- The MOV reduction (T.E chapter).

All other proofs are sketches: the key idea is stated, the main steps are named, and a citation
points to the full argument. The sketch is honest — it does not pretend to be a proof, and it does
not omit the key idea.

**Complete:** no key idea is silently omitted. If a theorem is used, it is stated. If an algorithm
depends on a non-obvious mathematical fact, that fact is named and cited.

**Academic and clinical:** the register is precise and unsentimental. Intuition leads, rigour
follows — but the rigour is there.

**Not exhaustive:** no encyclopaedic case enumeration. The goal is understanding, not a reference
manual. Edge cases are noted when they are pedagogically important; otherwise they are cited.

**Not inscrutable:** every section opens with the intuition before the formalism. A reader who
skips the proofs should still understand what is true and why it matters.

### Through-line

**Structure-based escape from search.** Every algorithm in this textbook is a story about finding
exploitable structure — a group homomorphism, a smoothness phenomenon, an endomorphism, a pairing,
a quantum period — that escapes the generic $\sqrt{n}$ or $L$-notation search bound. This
through-line is stated once in the Escape-from-Search chapter and revisited at the opening of every
subsequent chapter.

### Markup

**Markdown + MathJax.** Inline mathematics uses `$…$`; display mathematics uses `$$…$$`. This
supersedes the earlier "rST or Markdown TBD" in the ROADMAP Phase τ scope contract. Rationale:
zero migration (every artifact stays `.md`), renders on GitHub and in standard Markdown viewers,
and lets trivial inline glyphs stay as Unicode while non-trivial expressions use TeX delimiters.
The TeX source is the same notation the textbook would use under any tooling. Reopen only on a
hard MathJax limitation (a renderer the project must target that lacks MathJax).

Trivial inline glyphs ($\mathbb{Z}$, $\mathbb{F}_2$, $\equiv$, subscripts) may remain as Unicode
where they read naturally. Non-trivial expressions — fractions, limits, aligned derivations,
matrices, sub/superscript stacks — use MathJax delimiters.

### Artifact location

**`docs/MATHEMATICS.md`** (single file). Chapters are appended to this file as they are written
(T.G appends the GNFS chapter; T.D, T.E, T.S append their chapters at their respective ◆
boundaries). Promote to `docs/textbook/` only if the single file becomes unwieldy — decide at T.Z
(the textbook bind) and record the decision here.

### Chapter-pairing pattern

Each textbook chapter is the maths-first sibling of a `PEDAGOGY.md` code-tour chapter. The
pairing is:

| Textbook chapter (this file) | Code-tour chapter |
|------------------------------|-------------------|
| §Pollard Rho (ECDLP) | `docs/PEDAGOGY.md` |
| §α-Substrate | `shared/numth/docs/PEDAGOGY.md` |
| §GNFS (T.G, to be appended) | `gnfs/docs/PEDAGOGY.md` (integrative chapter) |
| §NFS-DL (T.D, to be appended) | `gnfs/docs/PEDAGOGY.md` (NFS-DL chapter) |
| §Algebraic ECDLP Attacks (T.E) | `gnfs/docs/PEDAGOGY.md` (Track E chapter) |
| §Shor + Post-Quantum (T.S) | `gnfs/docs/PEDAGOGY.md` (Track S chapter) |

The code-tour cites the textbook chapter for the mathematics; the textbook chapter cites the
code-tour for the realisation. Neither is a prerequisite for the other — they are complementary
lenses on the same material.

---

## Table of Contents

The chapter skeleton across the whole survey. Each entry is a chapter title and a one-sentence
scope statement. Later `*.W` sessions fill the chapters marked *to be appended*.

1. **C-Textbook: Documentation-Register Contract** *(this section)* — the audience, depth,
   through-line, markup, and location contract every chapter obeys.

2. **Table of Contents** *(this section)* — the chapter skeleton.

3. **Escape from Search: The Through-Line** — the conceptual spine: every attack finds exploitable
   structure that escapes the generic $\sqrt{n}$ or $L$-notation search bound, and the taxonomy of
   structures that make this possible.

4. **Prerequisites** — the undergraduate-background bridge: the specific theorems from algebra,
   analysis, probability, and logic that later chapters lean on, each stated precisely with a
   proof sketch or citation.

5. **On Scale: A Natural-Philosophy Interlude** — the three axes of scale (resource/operational,
   mathematical-dimension, structural), their couplings, and the honest science↔engineering gap
   that runs through every chapter.

6. **Pollard Rho for ECDLP** — the birthday-paradox collision argument, Floyd's cycle detection,
   the group-homomorphism structure that makes rho work on elliptic curves, and the $L$-notation
   bound; maths-first sibling to `docs/PEDAGOGY.md`.

7. **The α-Substrate: Primality, Smoothness, and ECM** — Miller–Rabin primality (Fermat witnesses,
   strong pseudoprimes), smooth-number theory (the $B$-smooth density theorem), ECM (Lenstra's
   elliptic-curve method), and Tonelli–Shanks (square roots mod $p$); maths-first sibling to
   `shared/numth/docs/PEDAGOGY.md`.

8. **GNFS: The General Number Field Sieve** *(T.G — to be appended)* — the number-field bridge,
   factor-base smoothness, linear-algebra dependency, square-root recovery, and the full
   $L_N[1/3, (64/9)^{1/3}]$ subexponentiality derivation as the payoff proof.

9. **NFS-DL: Discrete Logarithm via the Number Field Sieve** *(T.D — to be appended)* — the
   adaptation of GNFS to the discrete-logarithm problem, Schirokauer maps, and the individual
   logarithm descent.

10. **Algebraic ECDLP Attacks** *(T.E — to be appended)* — Pohlig–Hellman, the MOV/Frey–Rück
    reduction (payoff proof: the MOV bridge), Smart–Satoh–Araki, GHS/Weil descent, and
    Gaudry–Diem–Joux–Vitse index calculus.

11. **Shor's Algorithm and Post-Quantum Context** *(T.S — to be appended)* — the quantum period-
    finding algorithm, its application to factoring and ECDLP, and the post-quantum migration
    landscape.

---

## Escape from Search: The Through-Line

### The generic search bound

Every cryptographic hardness assumption rests, at bottom, on the difficulty of a *search problem*:
find $k$ such that $k \cdot G = Q$ (the discrete logarithm), or find $p$ such that $p \mid N$ (the
factoring problem). The naive approach is exhaustive search: try every candidate. For a group of
order $n$, this costs $O(n)$ operations.

The first non-trivial observation is that exhaustive search is never necessary. The **birthday
paradox** gives a generic lower bound on what any algorithm must do, and a generic algorithm that
meets it: by sampling $O(\sqrt{n})$ random elements of a group of order $n$, one expects a
collision — two samples that land on the same element. Pollard's rho algorithm (1978) exploits this
to solve the discrete logarithm in $O(\sqrt{n})$ group operations, and Shank's baby-step/giant-step
algorithm achieves the same bound with $O(\sqrt{n})$ space. For the factoring problem, Fermat's
method and its descendants give $O(N^{1/4})$ operations for a balanced semiprime.

The $\sqrt{n}$ barrier is the **generic search bound**: it is what any algorithm achieves without
exploiting the specific structure of the problem. For a 256-bit elliptic curve group ($n \approx
2^{256}$), the generic bound is $2^{128}$ operations — the security level of ECC-256.

### Escaping the bound: the taxonomy of structures

The algorithms in this textbook all escape the generic bound by finding exploitable structure. The
structures fall into five families:

**1. Group homomorphisms (index calculus, smoothness).** If the group $G$ admits a homomorphism
$\phi: G \to H$ where $H$ has a known structure (e.g. $H = \mathbb{Z}/n\mathbb{Z}$), then the
discrete logarithm in $G$ reduces to a problem in $H$. The key instance is *smoothness*: if the
group elements can be expressed as products of a small set of generators (the *factor base*), then
the discrete logarithm reduces to a system of linear equations over $\mathbb{Z}/n\mathbb{Z}$. This
is the engine of index calculus, NFS, and NFS-DL.

**2. Endomorphisms (GLV, Koblitz).** An endomorphism $\phi: E \to E$ of an elliptic curve satisfies
$\phi(P) = \lambda P$ for all $P$, where $\lambda$ is a fixed scalar. This collapses the orbit
$\{P, \phi(P), \phi^2(P), \ldots\}$ to a single representative, reducing the effective group size
and the cost of Pollard rho. The GLV decomposition (Gallant–Lambert–Vanstone) and the Koblitz
automorphism are the canonical instances.

**3. Pairings (MOV reduction).** A bilinear pairing $e: E[n] \times E[n] \to \mu_n$ maps the
discrete logarithm on an elliptic curve to the discrete logarithm in a multiplicative group
$\mathbb{F}_{p^k}^*$, where index calculus applies. The MOV/Frey–Rück reduction (Menezes–Okamoto–
Vanstone, Frey–Rück) is the canonical instance; it is the cross-track bridge of this project.

**4. Number-field structure (GNFS, NFS-DL).** The integers $\mathbb{Z}$ embed into a number field
$K = \mathbb{Q}(\alpha)$, where the norm map $N_{K/\mathbb{Q}}$ connects factorisation in $K$ to
factorisation in $\mathbb{Z}$. The GNFS exploits this by sieving for *smooth norms* — elements of
$\mathbb{Z}[\alpha]$ whose norms factor completely over a small factor base — and then using linear
algebra to find a congruence of squares $x^2 \equiv y^2 \pmod{N}$. The subexponential complexity
$L_N[1/3, (64/9)^{1/3}]$ arises from the optimal balance between the sieving cost and the
smoothness probability.

**5. Quantum period-finding (Shor).** Shor's algorithm (1994) uses the quantum Fourier transform to
find the period of the function $f(k) = g^k \bmod N$ in polynomial time. This dissolves the
$\sqrt{n}$ barrier entirely in the quantum model, reducing both factoring and discrete logarithm to
polynomial time. The structure exploited is the *periodicity* of modular exponentiation — a
structure that classical algorithms cannot access efficiently.

### The L-notation hierarchy

The complexity of the best known algorithms for the main problems forms a hierarchy:

$$L_N[\alpha, c] = \exp\!\left(c \cdot (\log N)^\alpha \cdot (\log \log N)^{1-\alpha}\right)$$

- **Fully exponential** ($\alpha = 1$): $L_N[1, c] = N^c$. Generic search (baby-step/giant-step,
  Pollard rho) for ECDLP on a generic curve.
- **Subexponential** ($0 < \alpha < 1$): $L_N[\alpha, c]$. Index calculus for DLP in
  $\mathbb{F}_p^*$ ($\alpha = 1/2$); GNFS for factoring and NFS-DL ($\alpha = 1/3$).
- **Quasi-polynomial** ($\alpha \to 0$): $L_N[0, c] = (\log N)^c$. The Barbulescu–Gaudry–Joux–
  Thomé algorithm for DLP in small-characteristic fields (2014).
- **Polynomial** ($\alpha = 0$, exactly): Shor's algorithm for both factoring and DLP.

The through-line of this textbook is the story of how each step down this hierarchy is achieved by
finding a new kind of exploitable structure.

### Why the bound matters

The $\sqrt{n}$ generic bound is not just a complexity statement — it is a *security guarantee*. A
256-bit elliptic curve group provides 128-bit security *if and only if* no algorithm beats the
generic bound on that curve. The moment a structural exploit is found (an endomorphism, a pairing
with small embedding degree, a special field structure), the security level collapses. The history
of cryptanalysis is a history of finding such structures in curves and fields that were believed to
be generic.

This is why the through-line matters: understanding *which* structures enable *which* escapes is
the key to both attacking and defending cryptographic systems.

---

## Prerequisites

This chapter collects the specific theorems from algebra, analysis, probability, and logic that
later chapters lean on. Each theorem is stated precisely; the proof is either sketched (with a
citation to the full argument) or, where the proof is short and illuminating, given in full.

A reader who is comfortable with all of these results can proceed directly to any chapter. A reader
who needs to review specific topics should consult the cited references.

### Algebra

#### Groups, rings, and fields

**Definition (group).** A *group* $(G, \cdot)$ is a set $G$ with a binary operation $\cdot$ that
is associative, has an identity element $e$, and has inverses. A group is *abelian* (or
*commutative*) if $a \cdot b = b \cdot a$ for all $a, b \in G$. The *order* of $G$, written $|G|$,
is the cardinality of $G$. The *order* of an element $g \in G$ is the smallest positive integer $k$
such that $g^k = e$.

**Theorem (Lagrange).** *If $H$ is a subgroup of a finite group $G$, then $|H|$ divides $|G|$.*

*Proof sketch.* The cosets $gH$ partition $G$ into equal-sized pieces of size $|H|$. $\square$

**Corollary.** *The order of any element $g \in G$ divides $|G|$.*

**Definition (cyclic group).** A group $G$ is *cyclic* if there exists $g \in G$ such that every
element of $G$ is a power of $g$. Such a $g$ is called a *generator* of $G$.

**Theorem (structure of cyclic groups).** *Every cyclic group of order $n$ is isomorphic to
$\mathbb{Z}/n\mathbb{Z}$. Every subgroup of a cyclic group is cyclic.*

**Definition (ring, field).** A *ring* $(R, +, \cdot)$ is an abelian group under $+$ with a
compatible multiplication that is associative and distributes over $+$. A *field* is a commutative
ring in which every non-zero element has a multiplicative inverse.

**Theorem (finite fields).** *For every prime power $q = p^k$, there exists a unique (up to
isomorphism) field $\mathbb{F}_q$ with $q$ elements. The multiplicative group $\mathbb{F}_q^*$ is
cyclic of order $q - 1$.*

*Proof sketch.* Existence: $\mathbb{F}_{p^k}$ is the splitting field of $x^{p^k} - x$ over
$\mathbb{F}_p$. Uniqueness: any two fields with $q$ elements are both splitting fields of the same
polynomial, hence isomorphic. Cyclicity of $\mathbb{F}_q^*$: a finite subgroup of the
multiplicative group of any field is cyclic (the key lemma: a polynomial of degree $d$ over a
field has at most $d$ roots, so the number of elements of order dividing $d$ is at most $d$, which
forces the group to be cyclic). $\square$

#### Ideals and factorisation in Dedekind domains

**Definition (ideal).** An *ideal* $\mathfrak{a}$ in a ring $R$ is a non-empty subset closed under
addition and under multiplication by any element of $R$. A *prime ideal* $\mathfrak{p}$ is an ideal
such that $ab \in \mathfrak{p}$ implies $a \in \mathfrak{p}$ or $b \in \mathfrak{p}$.

**Definition (Dedekind domain).** A *Dedekind domain* is an integral domain $R$ that is Noetherian,
integrally closed in its fraction field, and has Krull dimension 1 (every non-zero prime ideal is
maximal).

**Theorem (unique factorisation of ideals in Dedekind domains).** *In a Dedekind domain, every
non-zero ideal factors uniquely as a product of prime ideals.*

*Proof sketch.* The key steps are: (1) every ideal is contained in a maximal ideal; (2) the
fractional ideals form a group under multiplication; (3) the group of fractional ideals is free
abelian on the prime ideals. The proof uses the Noetherian condition to guarantee termination of
the factorisation process. See Neukirch [N99, §I.3] for the full argument. $\square$

**Why this matters for GNFS.** The ring of integers $\mathcal{O}_K$ of a number field $K =
\mathbb{Q}(\alpha)$ is a Dedekind domain. The GNFS exploits the unique factorisation of ideals in
$\mathcal{O}_K$ to connect smoothness of norms in $\mathbb{Z}$ to smoothness of ideals in
$\mathcal{O}_K$. The *bad primes* — primes dividing the index $[\mathcal{O}_K : \mathbb{Z}[\alpha]]$
— are exactly the primes where $\mathbb{Z}[\alpha]$ fails to be a Dedekind domain, and they require
special handling (the Dedekind index criterion).

#### Elliptic curves

**Definition (elliptic curve).** An *elliptic curve* over a field $k$ is a smooth projective curve
of genus 1 with a specified base point $\mathcal{O}$. In short Weierstrass form (valid when
$\mathrm{char}(k) \neq 2, 3$):

$$E: y^2 = x^3 + ax + b, \quad \Delta = -16(4a^3 + 27b^2) \neq 0.$$

The condition $\Delta \neq 0$ ensures smoothness (no cusps or self-intersections).

**Theorem (group law).** *The points of $E(k)$ (including the point at infinity $\mathcal{O}$)
form an abelian group under the chord-and-tangent law, with $\mathcal{O}$ as the identity.*

*Proof sketch.* Associativity is the non-trivial part; it follows from the Riemann–Roch theorem
applied to the divisor class group of $E$. The explicit addition formulas (slope, $x$-coordinate,
$y$-coordinate) are derived from the intersection of a line with the cubic. $\square$

**Theorem (Hasse's bound).** *For an elliptic curve $E$ over $\mathbb{F}_p$,*

$$\left| |E(\mathbb{F}_p)| - (p + 1) \right| \leq 2\sqrt{p}.$$

*Proof sketch.* The bound follows from the Riemann hypothesis for curves over finite fields
(Weil's theorem): the eigenvalues of Frobenius on the Tate module have absolute value $\sqrt{p}$.
See Silverman [S09, §V.1]. $\square$

**Why this matters.** Hasse's bound says the group order $|E(\mathbb{F}_p)|$ lies in the interval
$[p + 1 - 2\sqrt{p},\, p + 1 + 2\sqrt{p}]$. For ECM (Lenstra's method), this means that by
choosing different curves, one can vary the group order over an interval of width $4\sqrt{p}$
around $p$, and hope to find a curve whose order is smooth.

#### The Chinese Remainder Theorem

**Theorem (CRT).** *Let $m_1, \ldots, m_k$ be pairwise coprime positive integers, and let $M =
m_1 \cdots m_k$. Then the natural ring homomorphism*

$$\mathbb{Z}/M\mathbb{Z} \xrightarrow{\;\sim\;} \mathbb{Z}/m_1\mathbb{Z} \times \cdots \times
\mathbb{Z}/m_k\mathbb{Z}$$

*is an isomorphism.*

*Proof.* The map $x \mapsto (x \bmod m_1, \ldots, x \bmod m_k)$ is a ring homomorphism. Injectivity
follows from $\gcd(m_i, m_j) = 1$; surjectivity follows from Bézout's identity. $\square$

**Why this matters.** CRT is used in the GNFS square-root step (Couveignes' algorithm computes the
algebraic square root modulo many primes and lifts via CRT), in Pohlig–Hellman (reducing ECDLP in
a group of composite order to ECDLP in prime-order subgroups), and in the construction of
deterministic Miller–Rabin witness sets.

### Analysis

#### Big-$O$ and $L$-notation

**Definition (big-$O$).** $f(n) = O(g(n))$ as $n \to \infty$ if there exist constants $C > 0$ and
$n_0$ such that $|f(n)| \leq C \cdot g(n)$ for all $n \geq n_0$.

**Definition ($L$-notation).** For $0 \leq \alpha \leq 1$ and $c > 0$, define

$$L_N[\alpha, c] = \exp\!\left(c \cdot (\log N)^\alpha \cdot (\log \log N)^{1-\alpha}\right).$$

This interpolates between polynomial ($\alpha = 0$: $L_N[0, c] = (\log N)^c$) and fully
exponential ($\alpha = 1$: $L_N[1, c] = N^c$). The intermediate regime $0 < \alpha < 1$ is
*subexponential*: faster than any polynomial in $\log N$ but slower than any polynomial in $N$.

**Lemma ($L$-notation arithmetic).** *For $0 < \alpha < 1$:*

$$L_N[\alpha, c_1] \cdot L_N[\alpha, c_2] = L_N[\alpha, c_1 + c_2], \qquad
L_N[\alpha, c]^k = L_N[\alpha, kc].$$

*Moreover, $L_N[\alpha, c] = o(N^\varepsilon)$ for every $\varepsilon > 0$, and $L_N[\alpha, c] =
\omega((\log N)^M)$ for every $M > 0$.*

*Proof.* Direct computation from the definition. $\square$

**Why this matters.** The $L$-notation is the natural language for stating the complexity of
subexponential algorithms. The GNFS achieves $L_N[1/3, (64/9)^{1/3}]$; the Pohlig–Hellman
reduction achieves $L_p[1/2, 1]$ for the largest prime factor $p$ of the group order; Shor's
algorithm achieves $O((\log N)^3)$. The exponent $\alpha$ is the key invariant: it determines
whether the algorithm is polynomial, subexponential, or exponential in the input size.

#### The prime number theorem

**Theorem (prime number theorem).** *Let $\pi(x)$ denote the number of primes $\leq x$. Then*

$$\pi(x) \sim \frac{x}{\log x} \quad \text{as } x \to \infty.$$

*Equivalently, the $n$-th prime $p_n \sim n \log n$.*

*Proof sketch.* The proof uses the Riemann zeta function $\zeta(s) = \sum_{n=1}^\infty n^{-s}$ and
the fact that $\zeta(s)$ has no zeros on the line $\mathrm{Re}(s) = 1$. See Davenport [D00,
Chapter 13]. $\square$

**Why this matters.** The prime number theorem controls the size of factor bases: the number of
primes up to $B$ is approximately $B / \log B$. This enters the complexity analysis of every
sieve-based algorithm.

### Probability

#### The birthday paradox

**Theorem (birthday bound).** *If $m$ elements are chosen uniformly at random (with replacement)
from a set of size $n$, the probability that at least two are equal is approximately*

$$1 - e^{-m^2/(2n)} \approx \frac{m^2}{2n} \quad \text{for } m \ll n.$$

*In particular, the expected number of draws before a collision is $\Theta(\sqrt{n})$.*

*Proof.* The probability that all $m$ draws are distinct is $\prod_{k=0}^{m-1}(1 - k/n) \approx
e^{-m(m-1)/(2n)}$ for $m \ll n$. Setting this equal to $1/2$ and solving gives $m \approx
\sqrt{2n \log 2} = \Theta(\sqrt{n})$. $\square$

**Why this matters.** The birthday bound is the foundation of Pollard rho: the expected number of
steps before a collision in a pseudorandom walk on a group of order $n$ is $\Theta(\sqrt{n})$. It
is also the reason that a 256-bit elliptic curve group provides 128-bit security against generic
attacks.

#### Smooth-number density

**Definition ($B$-smooth).** An integer $m$ is *$B$-smooth* if all its prime factors are $\leq B$.

**Theorem (Canfield–Erdős–Pomerance, 1983).** *Let $\Psi(x, y)$ denote the number of $y$-smooth
integers in $[1, x]$. For $u = \log x / \log y$ fixed as $x \to \infty$,*

$$\Psi(x, y) = x \cdot \rho(u) \cdot (1 + o(1)),$$

*where $\rho(u)$ is the Dickman function, defined by $\rho(u) = 1$ for $0 \leq u \leq 1$ and*

$$u \rho'(u) = -\rho(u - 1) \quad \text{for } u > 1.$$

*For large $u$, $\rho(u) = u^{-u(1 + o(1))}$.*

*Proof sketch.* The key step is to count integers in $[1, x]$ whose largest prime factor is $\leq
y$, by conditioning on the largest prime factor and applying the prime number theorem. The Dickman
function arises as the continuous analogue of this recursion. See Granville [G08] for a modern
treatment. $\square$

**Corollary (smoothness probability in $L$-notation).** *If $x = N$ and $y = L_N[1/3, c]$, then*

$$\frac{\Psi(N, y)}{N} = L_N[-1/3, -1/(3c)] \cdot (1 + o(1)).$$

*This is the key estimate in the GNFS complexity analysis: the probability that a random integer
near $N$ is $L_N[1/3, c]$-smooth is $L_N[-1/3, \ldots]$, which is subexponentially small but
much larger than $N^{-\varepsilon}$ for any fixed $\varepsilon > 0$.*

**Why this matters.** The smooth-number density theorem is the engine of every sieve-based
algorithm. The GNFS complexity $L_N[1/3, (64/9)^{1/3}]$ arises from optimising the balance between
the smoothness probability (which improves as the smoothness bound $B$ grows) and the cost of the
linear algebra step (which grows with $B$). The optimal $B = L_N[1/3, c]$ for some constant $c$,
and the exponent $1/3$ is the result of this optimisation.

### Logic and complexity

#### Polynomial-time reductions

**Definition (polynomial-time reduction).** A problem $A$ *polynomial-time reduces* to a problem
$B$ (written $A \leq_P B$) if there is a polynomial-time algorithm that transforms any instance of
$A$ into an instance of $B$ such that the answer is preserved.

**Why this matters.** The MOV reduction (T.E chapter) is a polynomial-time reduction from ECDLP
on a curve with small embedding degree to DLP in $\mathbb{F}_{p^k}^*$. Pohlig–Hellman is a
polynomial-time reduction from ECDLP in a group of composite order to ECDLP in prime-order
subgroups. Understanding reductions is essential for understanding why these attacks work.

#### Fermat's little theorem and Euler's criterion

**Theorem (Fermat's little theorem).** *If $p$ is prime and $\gcd(a, p) = 1$, then $a^{p-1} \equiv
1 \pmod{p}$.*

*Proof.* The map $x \mapsto ax$ is a bijection on $(\mathbb{Z}/p\mathbb{Z})^*$, so $\prod_{x=1}^{p-1}
(ax) \equiv \prod_{x=1}^{p-1} x \pmod{p}$, giving $a^{p-1} \equiv 1$. $\square$

**Theorem (Euler's criterion).** *For an odd prime $p$ and $\gcd(a, p) = 1$,*

$$a^{(p-1)/2} \equiv \left(\frac{a}{p}\right) \pmod{p},$$

*where $\left(\frac{a}{p}\right)$ is the Legendre symbol: $+1$ if $a$ is a quadratic residue mod
$p$, $-1$ if not.*

*Proof.* The multiplicative group $(\mathbb{Z}/p\mathbb{Z})^*$ is cyclic of order $p - 1$. An
element $a$ is a square iff $a^{(p-1)/2} = 1$ (since squaring is a 2-to-1 map on the group). The
non-residues satisfy $a^{(p-1)/2} = -1$ (the only other square root of $1$ in a field). $\square$

**Why this matters.** Fermat's little theorem is the basis of Miller–Rabin primality testing and of
the Fermat primality test. Euler's criterion is the basis of the Legendre symbol computation and
the Tonelli–Shanks square root algorithm.

#### The discrete logarithm problem

**Definition (DLP).** Let $G$ be a cyclic group of order $n$ with generator $g$. Given $h \in G$,
the *discrete logarithm problem* (DLP) asks for the unique $k \in \{0, 1, \ldots, n-1\}$ such that
$g^k = h$.

**Definition (ECDLP).** The *elliptic curve discrete logarithm problem* (ECDLP) is the DLP in the
group $E(\mathbb{F}_p)$ of points on an elliptic curve over a prime field.

**Why hardness is believed.** No polynomial-time classical algorithm is known for the DLP or ECDLP
in a generic group. The best known classical algorithms for ECDLP on a generic curve run in
$\Theta(\sqrt{n})$ time (Pollard rho, baby-step/giant-step). For DLP in $\mathbb{F}_p^*$, the best
known algorithms run in $L_p[1/3, (64/9)^{1/3}]$ time (NFS-DL). The hardness of these problems is
a *computational assumption*, not a theorem — no proof that they require superpolynomial time is
known.

### References for prerequisites

- [N99] Neukirch, J. *Algebraic Number Theory*. Springer, 1999. (Dedekind domains, ideal
  factorisation, number fields.)
- [S09] Silverman, J. H. *The Arithmetic of Elliptic Curves*. 2nd ed. Springer, 2009. (Elliptic
  curves, Hasse's bound, the group law.)
- [D00] Davenport, H. *Multiplicative Number Theory*. 3rd ed. Springer, 2000. (Prime number
  theorem, Dirichlet series.)
- [G08] Granville, A. "Smooth numbers: computational number theory and beyond." In *Algorithmic
  Number Theory: Lattices, Number Fields, Curves and Cryptography*, MSRI Publications 44, 2008.
  (Smooth-number density, Dickman function.)
- [CP05] Crandall, R., and Pomerance, C. *Prime Numbers: A Computational Perspective*. 2nd ed.
  Springer, 2005. (Miller–Rabin, Tonelli–Shanks, ECM, smoothness.)

---

## On Scale: A Natural-Philosophy Interlude

*This interlude is the full exposition deferred from the ROADMAP's `## On scale` section. The
ROADMAP states the clinical summary; this chapter provides the natural-philosophy context that
belongs in a textbook register, not a planning document.*

### The word "scale" is not one thing

Every chapter in this textbook involves a comparison between "toy scale" and "real scale" — between
the parameters at which the algorithms are demonstrated here and the parameters at which they are
deployed in practice. The comparison is essential for honest pedagogy: an algorithm that is
"correct" at toy scale may be missing engineering that only matters at real scale, and a phenomenon
that is "prominent" at toy scale may wash out at real scale.

But the comparison is routinely made carelessly, as if "scale" were a single axis — a ladder from
small to large, with toy instances at the bottom and RSA-2048 at the top. It is not. At least three
distinct things travel under the word, and they do not form one ordered line.

### Axis 1: Resource/operational scale

The first axis is the one most people mean: how large an instance one can actually run. Toy scale
here means roughly 80-bit $N$ on a laptop; "NFS scale" means RSA-768 ($N \approx 2^{768}$) on a
cluster-month, or RSA-829 ($N \approx 2^{829}$) on a larger cluster. This axis *is* an unbounded
ladder — bigger $N$ always exists, up to a physical-computation ceiling.

But this axis is **phenomenologically flat**: scaling $N$ alone introduces no new mathematics. It
makes the same machinery bigger and improves the statistics. This is exactly why *over-exposed*
phenomena (bad primes, for instance) *wash out* at large $N$ rather than changing in kind: at
cryptographic scale, the contribution of bad primes is marginal and largely absorbed by polynomial
selection, but at toy scale with a hand-picked polynomial they are unavoidable and must be handled
head-on. The honest annotation is that the prominence is a toy-scale artifact, not the typical
NFS-scale picture.

The flatness of this axis is also why the GNFS complexity $L_N[1/3, (64/9)^{1/3}]$ is a
*heuristic*, not a theorem: the smoothness-probability estimates that drive the analysis are
asymptotic, and the asymptotic regime is only reached at scales far beyond what any computer can
run. The algorithm is correct at toy scale; the complexity analysis is only accurate at a scale
that is, in a precise sense, unreachable.

### Axis 2: Mathematical-dimension scale

The second axis is qualitatively different: the *degree* $d = [K:\mathbb{Q}]$ of the number field,
the *embedding degree* $k$ of a pairing, the *characteristic* vs *extension degree* shape of a
finite field $\mathbb{F}_{p^n}$. These are dimensions *of the mathematics*, not of the instance
size.

This axis is **not** a ladder in the same sense. A number field of degree $d = 6$ is not "larger"
than one of degree $d = 3$ in any operational sense — it is *richer*. The ramification structure,
the Galois group, the arithmetic of the ring of integers — all of these change qualitatively as $d$
grows, not just quantitatively. A toy instance over a degree-6 field can exhibit phenomena that a
"huge" instance over a degree-3 field never touches.

This is the axis that drives the most interesting mathematics in this textbook. The MOV reduction
works because the embedding degree $k$ of the target curve is small; the GHS descent works because
the field $\mathbb{F}_{2^m}$ has a tower structure; the quasi-polynomial DLP algorithm works
because the characteristic is small relative to the extension degree. None of these phenomena are
about instance size — they are about the *shape* of the mathematical object.

### Axis 3: Structural scale

The third axis is the most subtle: *thresholds* where scaling unlocks (or requires) a *different*
machine. These are not just quantitative changes — they are qualitative transitions.

The canonical examples:

- **Large-prime variations.** The basic NFS sieve collects only fully smooth relations. The
  large-prime variation collects *partial* relations — those with one or two large prime cofactors —
  and combines them to form full relations. This variation only pays off once the relation yield is
  large enough that the partial-relation graph has a giant component; at toy scale, the graph is
  sparse and the variation adds overhead without benefit. The threshold is a structural property of
  the relation graph, not a function of $N$ alone.

- **The quasi-polynomial DLP regime.** The Barbulescu–Gaudry–Joux–Thomé algorithm (2014) achieves
  quasi-polynomial complexity for DLP in $\mathbb{F}_{p^n}$ when the characteristic $p$ is small
  (e.g. $p = 2$). This is not a quantitative improvement on the $L[1/3]$ bound — it is a
  *structural* discovery that the small-characteristic case admits a fundamentally different
  algorithm. The threshold is the ratio $p / n$, not the size of the field.

- **The asymptotic regime of $L$-notation.** The $L$-notation complexity of GNFS is an asymptotic
  statement: it is accurate only when $N$ is large enough that the smoothness-probability estimates
  are reliable. At toy scale, the optimal parameters (smoothness bound $B$, sieve region size) are
  not the asymptotically optimal ones — they are calibrated empirically. The transition to the
  asymptotic regime is a structural threshold, not a smooth function of $N$.

### The three couplings

The three axes are sometimes independent, sometimes coupled, and sometimes structurally enabling.

**Independent:** the degree $d$ and the instance size $N$ are separate knobs. One can choose a
degree-6 number field for a 100-bit $N$, or a degree-3 field for a 1000-bit $N$. The mathematics
of the number field does not depend on $N$ directly.

**Coupled along the efficient frontier.** The *optimal* degree for GNFS is tied to $N$ by

$$d \sim \left(\frac{3 \log N}{\log \log N}\right)^{1/3},$$

so scaling $N$ "properly" (along the efficient frontier) drags $d$ along. At toy scale, the optimal
$d$ is 3 or 4 and barely moves; at RSA-768 scale, $d \approx 6$. The coupling is invisible at toy
scale, which is why toy-scale experiments with $d = 3$ are not misleading — they are just not
exploring the full frontier.

**Structural-enabling.** At certain thresholds, scaling one axis changes *which* mathematics
applies, not just how big it is. The quasi-polynomial DLP regime is the clearest example: the
algorithm is not a quantitative improvement on NFS-DL, it is a different algorithm that only
applies in a specific structural regime. The large-prime variation is another: it requires the
relation graph to have a giant component, which is a structural threshold.

### Method-convergence vs problem-openness

A natural question: is NFS asymptotically convergent? The precise answer distinguishes two things
the word "convergence" blurs.

**Convergence of the method (true).** The GNFS heuristic complexity has sat at $L_N[1/3,
(64/9)^{1/3}]$ since the early 1990s. Three decades of work have improved the constant and the
engineering — not the exponent $1/3$. Within the NFS paradigm (sieve number-field relations for
smoothness, then linear algebra), the exponent has converged. Note even this is *heuristic*,
resting on unproven smoothness assumptions, not a theorem.

**Convergence of the problem (false / open).** The true complexity of factoring and discrete
logarithm is **open**. $L[1/3]$ is a believed barrier for the *general* problem, not a proven
lower bound, and history shows that apparent convergence is repeatedly punctured by structural
discovery:

- The Barbulescu–Gaudry–Joux–Thomé quasi-polynomial algorithm (2014) collapsed small-characteristic
  DLP — a regime everyone had taken to be $L[1/3]$ — to $L[0, c]$.
- Shor's algorithm (1994) dissolves the barrier entirely in the quantum model.

The plateau is real, but it is a plateau of *one method*. It is reset, not approached, by new
mathematical structure.

### The honest science↔engineering gap

This textbook implements the mathematics of these algorithms completely. It does not implement the
engineering optimisations that make them run at cryptographic scale. The gap is real and is
annotated explicitly wherever it appears.

The gap runs in both directions:

**Under-exposed at toy scale.** Some phenomena only appear at large scale. Large-prime variations
only pay off once relation yield is in the millions. Block-Lanczos convergence behaviour that a
tiny matrix never stresses. The optimal-degree coupling between $d$ and $N$ that is invisible at
$d = 3$. These are implemented at *demonstration fidelity* — the mathematical content is present,
but the payoff is unreachable here. The annotation says: the mathematics is correct; the
engineering scale to exhibit the payoff is out of scope.

**Over-exposed at toy scale.** Some phenomena are more prominent at toy scale than at real scale.
Bad primes (primes dividing $\mathrm{disc}(f)$) are marginal at cryptographic scale but unavoidable
at toy scale with a hand-picked polynomial. The annotation says: this prominence is a toy-scale
artifact; at real scale it washes out.

The disconnect itself is pedagogical content. Part of honestly surveying how these algorithms
behave at the scales they were designed for is acknowledging what toy scale can and cannot exhibit.

---

## Pollard Rho for ECDLP

*Maths-first sibling to `docs/PEDAGOGY.md`. For the code-tour — phase-by-phase implementation
narrative, optimisation sequence, benchmarks, and CLI — see `docs/PEDAGOGY.md`. For the
prerequisites used in this chapter, see §Prerequisites above.*

### The through-line for this chapter

Pollard rho for ECDLP is the canonical example of **structure-based escape from search via group
homomorphism**. The structure exploited is the *linear representation* of walk states: every point
in the walk is a known linear combination $W = a \cdot G + b \cdot Q$ of the generator $G$ and the
target $Q$. A collision in the walk — two states that land on the same group element — immediately
yields a linear equation in the unknown discrete logarithm $k$. The birthday paradox guarantees
that such a collision occurs after $O(\sqrt{n})$ steps, where $n = |E(\mathbb{F}_p)|$.

The escape from the generic $O(n)$ exhaustive search is not a reduction in the *difficulty* of the
problem — it is a reduction in the *search space* from $n$ to $\sqrt{n}$, achieved by the birthday
paradox. The group homomorphism structure (the linear invariant $W = aG + bQ$) is what makes the
collision *useful*: without it, a collision would be a dead end.

### The birthday-paradox collision argument

Let $G$ be a cyclic group of prime order $n$, with generator $G$ and target $Q = k \cdot G$. A
*pseudorandom walk* on $G$ is a sequence $W_0, W_1, W_2, \ldots$ where each $W_{i+1} = f(W_i)$
for some deterministic function $f: G \to G$ that behaves like a random function.

**The key invariant.** The walk maintains scalars $a_i, b_i \in \mathbb{Z}/n\mathbb{Z}$ such that

$$W_i = a_i \cdot G + b_i \cdot Q$$

at every step. The function $f$ is chosen so that $a_{i+1}$ and $b_{i+1}$ can be computed from
$a_i$ and $b_i$ in $O(1)$ time.

**The collision.** Suppose $W_i = W_j$ for $i \neq j$. Then

$$a_i \cdot G + b_i \cdot Q = a_j \cdot G + b_j \cdot Q,$$

so $(a_i - a_j) \cdot G = (b_j - b_i) \cdot Q = (b_j - b_i) \cdot k \cdot G$, giving

$$k \equiv \frac{a_i - a_j}{b_j - b_i} \pmod{n}$$

provided $b_j \not\equiv b_i \pmod{n}$ (which holds with probability $1 - 1/n$; if it fails, retry
with a fresh walk).

**The birthday bound.** If $f$ behaves like a random function on $G$, the expected number of steps
before a collision is $\Theta(\sqrt{n})$ by the birthday paradox (§Prerequisites). This is the
$O(\sqrt{n})$ complexity of Pollard rho.

### The $r$-adding walk and mixing

The naive pseudorandom walk — "add a random point at each step" — has poor mixing properties: the
walk tends to cluster near its starting point, and the birthday bound is not achieved in practice.
Teske (1998) showed that an *$r$-adding walk* mixes much better.

**Definition ($r$-adding walk).** Precompute a table of $r$ random addends $R[0], \ldots, R[r-1]$,
where $R[i] = \alpha_i \cdot G + \beta_i \cdot Q$ with random scalars $\alpha_i, \beta_i$. At each
step, select the addend by $i = x \bmod r$ (where $x$ is a low word of the current point's
$x$-coordinate), then set

$$W \leftarrow W + R[i], \quad a \leftarrow a + \alpha_i \bmod n, \quad b \leftarrow b + \beta_i
\bmod n.$$

With $r \approx 20$, the walk behaves like a random function on the group, and the birthday bound
is achieved in practice.

**Why mixing matters.** The birthday bound $\Theta(\sqrt{n})$ assumes that the walk visits group
elements uniformly at random. A poorly mixing walk may visit some elements many times and others
never, increasing the expected collision time. The $r$-adding walk is designed to avoid this by
making the step function depend on the current point in a way that spreads the walk across the
group.

### Floyd's cycle detection

The walk $W_0, W_1, W_2, \ldots$ is eventually periodic: since $G$ is finite, the walk must
revisit a point, and from that point on it follows the same trajectory. The *rho* shape — a tail
leading into a cycle — gives the algorithm its name.

**Floyd's algorithm.** Run two pointers: the *tortoise* at speed 1 ($W_i$) and the *hare* at speed
2 ($W_{2i}$). When the hare catches the tortoise ($W_i = W_{2i}$), a collision has been detected.
The collision yields the discrete logarithm as above.

**Brent's improvement.** Freeze the tortoise at the start of each power-of-2 window; advance only
the hare. Compare the hare to the frozen tortoise at each step. After $2^k$ comparisons without
collision, snap the tortoise forward to the hare's position and double the window. This reduces the
number of function evaluations by approximately 24% compared to Floyd's algorithm.

**Expected complexity.** The expected number of steps before the hare catches the tortoise is
$\Theta(\sqrt{n})$, matching the birthday bound. The constant factor depends on the mixing
properties of the walk.

### The group-homomorphism structure

The reason Pollard rho works on elliptic curves — and not just on abstract groups — is the
*group-homomorphism structure* of the walk. The key point is that the elliptic curve group law is
*computable*: given two points $P$ and $Q$ on $E(\mathbb{F}_p)$, one can compute $P + Q$ in $O(1)$
field operations. This means the walk can be implemented efficiently, and the linear invariant
$W = aG + bQ$ can be maintained at each step.

More precisely: the map $(a, b) \mapsto aG + bQ$ is a group homomorphism from $\mathbb{Z}/n\mathbb{Z}
\times \mathbb{Z}/n\mathbb{Z}$ to $E(\mathbb{F}_p)$. The walk on $E(\mathbb{F}_p)$ lifts to a walk
on $\mathbb{Z}/n\mathbb{Z} \times \mathbb{Z}/n\mathbb{Z}$, and a collision in the former gives a
linear equation in the latter. The discrete logarithm is the solution to that equation.

This is the general pattern of index calculus: find a homomorphism from the group to a structure
where the discrete logarithm is easy, and use it to reduce the hard problem to an easy one. In
Pollard rho, the "easy structure" is just the linear equation $k \equiv (a_i - a_j)/(b_j - b_i)
\pmod{n}$, which is trivial to solve.

### Distinguished points and parallel collision search

The single-threaded Floyd/Brent algorithm is inherently sequential: the tortoise and hare must stay
in sync. The *distinguished-point* (DP) method (van Oorschot–Wiener, 1999) breaks this dependency
and enables linear parallelism.

**Definition (distinguished point).** A point $W \in E(\mathbb{F}_p)$ is *distinguished* if its
$x$-coordinate has at least $\theta$ low-order zero bits. The expected number of steps between
distinguished points is $2^\theta$.

**The parallel architecture.** $N$ walker threads each run an independent $r$-adding walk and emit
a record $(x, a, b)$ whenever the walk lands on a distinguished point. A coordinator thread
maintains a hash table keyed on $x$. When two records with the same $x$ but different $(a, b)$
pairs arrive, the coordinator solves for $k$.

**Why it works.** Two walks that ever visit the same point will, from that point on, follow
identical trajectories (the walk is deterministic). They will therefore hit the same distinguished
points. A collision in the DP table implies a collision in the underlying walks, which implies the
DLP can be recovered.

**Speedup.** With $N$ walkers, the expected wall-time steps to collision is $O(\sqrt{n} / N)$ —
linear speedup in the number of walkers. This is the second pedagogical moment: the speedup from
parallelism is linear, not $\sqrt{N}$.

### Orbit-collapsing maps: negation and GLV

The birthday bound $\Theta(\sqrt{n})$ applies to the *effective group size* — the number of
distinct walk states. By collapsing orbits of the group action, one can reduce the effective group
size and improve the expected collision time.

**The negation map.** On an elliptic curve, $P$ and $-P$ share the same $x$-coordinate (negation
flips only the $y$-coordinate). By collapsing $\{P, -P\}$ to a single canonical representative
(the one with the smaller $y$-coordinate), the effective group size is halved, reducing the
expected walk length by $\sqrt{2}$.

**The GLV endomorphism.** For a curve with an order-3 endomorphism $\phi$ (e.g. $y^2 = x^3 + 7$
over a field with a cube root of unity), the orbit of any generic point $P$ under $\langle \phi,
-1 \rangle$ has size 6:

$$\{P,\; \phi(P),\; \phi^2(P),\; -P,\; -\phi(P),\; -\phi^2(P)\}.$$

Collapsing this orbit to a single canonical representative reduces the effective group size by 6,
reducing the expected walk length by $\sqrt{6}$ compared to the plain walk, or $\sqrt{3}$ compared
to the negation-map-only walk.

**Scalar bookkeeping.** The orbit-collapsing maps require adjusting the scalars $(a, b)$ to
maintain the invariant $W = aG + bQ$. For the negation map: $-W = (n-a)G + (n-b)Q$. For the GLV
endomorphism: $\phi(W) = \lambda W$ where $\lambda$ is the eigenvalue of $\phi$, so $\phi(W) =
(\lambda a)G + (\lambda b)Q$.

**Fruitless cycles.** The canonical map can create *fruitless cycles*: the walk oscillates between
two orbit members that share the same canonical representative, and never hits a distinguished
point. The BKNS escape (Bos–Kleinjung–Niederhagen–Schwabe) handles this deterministically: detect
the period-2 pattern in a sliding window, then perturb by doubling the current point ($W \leftarrow
2W$, $a \leftarrow 2a$, $b \leftarrow 2b$) and re-canonicalising.

### The $L$-notation bound for ECDLP

For a generic elliptic curve over $\mathbb{F}_p$ with $|E(\mathbb{F}_p)| = n$, the best known
classical algorithm runs in $\Theta(\sqrt{n})$ group operations. In $L$-notation:

$$\text{ECDLP complexity} = L_n[1, 1/2] = n^{1/2} = \Theta(\sqrt{n}).$$

This is *fully exponential* in $\log n$ — the algorithm is not subexponential. The reason is that
no index-calculus algorithm is known for a generic elliptic curve: the group $E(\mathbb{F}_p)$ does
not have a natural "factor base" structure that would allow smooth-element collection.

**The contrast with DLP in $\mathbb{F}_p^*$.** For the multiplicative group $\mathbb{F}_p^*$, the
best known algorithm (NFS-DL) runs in $L_p[1/3, (64/9)^{1/3}]$ — subexponential. The key
difference is that $\mathbb{F}_p^*$ has a natural factor base (the small primes), while a generic
elliptic curve group does not. This is why ECC-256 provides 128-bit security while RSA-3072 is
needed for the same security level against classical attacks.

**When the bound breaks.** The $\sqrt{n}$ bound is the *generic* bound. It breaks when the curve
has exploitable structure:
- **Small embedding degree** (MOV/Frey–Rück): the ECDLP reduces to DLP in $\mathbb{F}_{p^k}^*$,
  where index calculus applies.
- **Anomalous curves** (Smart–Satoh–Araki): curves with $|E(\mathbb{F}_p)| = p$ admit a
  polynomial-time attack via $p$-adic logarithms.
- **Special field structure** (GHS descent): curves over $\mathbb{F}_{2^m}$ with a tower
  structure admit a descent to a hyperelliptic Jacobian.

These are the subjects of the T.E chapter.

### Cross-reference

For the phase-by-phase implementation of Pollard rho — the code, the optimisation sequence, the
benchmarks, and the CLI — see `docs/PEDAGOGY.md`. That chapter is the code-tour sibling to this
one: it assumes the reader knows the mathematics (this chapter) and focuses on the realisation in
Rust.

---

## The α-Substrate: Primality, Smoothness, and ECM

*Maths-first sibling to `shared/numth/docs/PEDAGOGY.md`. For the code-tour — implementation
details, code references, KAT summary, and downstream consumption — see
`shared/numth/docs/PEDAGOGY.md`. For the prerequisites used in this chapter, see §Prerequisites
above.*

### The through-line for this chapter

The α-substrate algorithms are the *infrastructure* of structure-based escape from search. They do
not themselves escape the generic bound — they provide the tools that other algorithms use to do
so. Specifically:

- **Miller–Rabin** decides primality, which is needed to build factor bases and to certify that a
  found factor is prime.
- **Smooth-number detection** identifies the elements that index-calculus algorithms collect; the
  smooth-number density theorem (§Prerequisites) is the mathematical engine of every sieve.
- **ECM** (Lenstra's elliptic-curve method) factors integers by exploiting the variability of
  elliptic-curve group orders — a direct application of the group-homomorphism structure from the
  Pollard rho chapter, now used for factoring rather than discrete logarithm.
- **Tonelli–Shanks** computes square roots modulo a prime, which is needed for curve-point recovery
  and for the GNFS square-root step.

The through-line is: these algorithms are the substrate on which the escape from search is built.
Understanding them is understanding the *tools* of the escape.

### Miller–Rabin primality testing

#### The Fermat test and its failure

The simplest primality test is the *Fermat test*: if $n$ is prime, then $a^{n-1} \equiv 1 \pmod{n}$
for all $a$ with $\gcd(a, n) = 1$ (Fermat's little theorem). A *Fermat pseudoprime* to base $a$ is
a composite $n$ that passes the Fermat test for base $a$.

The Fermat test fails catastrophically for *Carmichael numbers* — composites $n$ that pass the
Fermat test for *every* base $a$ with $\gcd(a, n) = 1$. The smallest Carmichael number is 561 =
$3 \cdot 11 \cdot 17$. There are infinitely many Carmichael numbers (Alford–Granville–Pomerance,
1994), so the Fermat test alone is not a reliable primality test.

#### The Miller–Rabin test

The Miller–Rabin test strengthens the Fermat test by exploiting the structure of square roots of 1
modulo a prime.

**Setup.** Write $n - 1 = 2^s \cdot d$ with $d$ odd. For a witness base $a$, compute $x = a^d
\bmod n$. Then square $x$ up to $s - 1$ times. If the sequence $a^d, a^{2d}, \ldots, a^{2^{s-1}d}$
ever hits $n - 1$ (or starts at 1), the witness $a$ is *satisfied* and offers no evidence of
compositeness. If it never does, $a$ is a *witness to compositeness* and $n$ is definitely
composite.

**Theorem (Miller–Rabin soundness).** *If $n$ is prime, every $a \in (1, n-1)$ is satisfied. If
$n$ is composite, at least $3/4$ of the bases $a \in \{1, \ldots, n-1\}$ are witnesses to
compositeness.*

*Proof sketch.* For a prime $n$, the only square roots of 1 modulo $n$ are $\pm 1$ (since
$\mathbb{F}_n$ is a field). The sequence $a^d, a^{2d}, \ldots$ must therefore hit $\pm 1$ before
reaching $a^{n-1} = 1$; if it hits $-1 = n-1$, the witness is satisfied. For a composite $n$,
the argument uses the structure of the group $(\mathbb{Z}/n\mathbb{Z})^*$ and the Chinese Remainder
Theorem to show that at most $1/4$ of the bases are satisfied. See Crandall–Pomerance [CP05, §3.5]
for the full argument. $\square$

**Corollary.** A composite $n$ that passes $k$ independent random bases slips through with
probability at most $4^{-k}$.

#### From probabilistic to deterministic

Miller–Rabin is probabilistic in general, but for bounded $n$ it can be made *deterministic* by
using a fixed witness set known to have no exceptions below a threshold. The key result is:

**Theorem (deterministic Miller–Rabin).** *The 13-base set $\{2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
31, 37\}$ is a deterministic primality test for all $n < 3{,}317{,}044{,}064{,}679{,}887{,}385{,}961{,}981
\approx 2^{81.6}$.*

*Proof.* Computational verification: no composite below the threshold passes all 13 bases. The
threshold and the witness set are from Sorenson–Webster (2017), refining earlier tables by
Pomerance–Selfridge–Wagstaff and Jaeschke. $\square$

**Why this matters.** The deterministic Miller–Rabin test is the primality oracle used throughout
this project. It is used to build factor bases (certifying that each base element is prime), to
certify that a found factor is prime, and inside ECM (to check that the found factor is not 1 or
$N$).

#### Strong pseudoprimes and the 2-Sylow structure

The Miller–Rabin test is more powerful than the Fermat test because it exploits the *2-Sylow
structure* of $(\mathbb{Z}/n\mathbb{Z})^*$. Writing $n - 1 = 2^s \cdot d$, the test checks not
just that $a^{n-1} \equiv 1$, but that the sequence of squarings $a^d, a^{2d}, \ldots, a^{2^{s-1}d}$
has the right structure: it must hit $-1$ before reaching $1$ (unless it starts at $1$). This is
exactly the structure that the 2-Sylow subgroup of $(\mathbb{Z}/p\mathbb{Z})^*$ forces for a prime
$p$.

A *strong pseudoprime* to base $a$ is a composite $n$ that passes the Miller–Rabin test for base
$a$. Strong pseudoprimes are much rarer than Fermat pseudoprimes: the smallest strong pseudoprime
to base 2 is 2047 = $23 \cdot 89$, and it is caught by base 23.

### Smooth-number theory

#### The $B$-smooth density theorem

The Canfield–Erdős–Pomerance theorem (§Prerequisites) gives the density of $B$-smooth integers:
for $u = \log N / \log B$ fixed,

$$\Pr[\text{random integer in } [1, N] \text{ is } B\text{-smooth}] = \rho(u) \cdot (1 + o(1)),$$

where $\rho(u)$ is the Dickman function. For large $u$, $\rho(u) \approx u^{-u}$.

**The key estimate for sieve algorithms.** If $B = L_N[1/3, c]$, then $u = \log N / \log B \sim
(\log N)^{2/3} / (c \cdot (\log \log N)^{1/3})$, and

$$\rho(u) = L_N\!\left[-\frac{1}{3},\, -\frac{1}{3c}\right] \cdot (1 + o(1)).$$

This is the smoothness probability that enters the GNFS complexity analysis. The probability is
subexponentially small, but much larger than $N^{-\varepsilon}$ for any fixed $\varepsilon > 0$.

**Why this matters.** The smooth-number density theorem is the mathematical engine of every sieve.
The GNFS sieve collects pairs $(a, b)$ such that both the rational norm $|a + bm|$ and the
algebraic norm $|N_{K/\mathbb{Q}}(a + b\alpha)|$ are $B$-smooth. The probability that a random
pair is smooth is $\rho(u)^2$ (approximately, for the two independent norms), and the optimal $B$
is chosen to balance this probability against the cost of the linear algebra step.

#### The factor base

**Definition (factor base).** A *factor base* $\mathcal{F}$ is a finite set of primes $\{p_1, p_2,
\ldots, p_k\}$ with $p_k \leq B$. An integer $m$ is $\mathcal{F}$-smooth (or $B$-smooth) if all
its prime factors are in $\mathcal{F}$.

**The smoothness witness.** A *smoothness witness* for $m$ over $\mathcal{F}$ is the factorisation
$m = \prod_{p \in \mathcal{F}} p^{e_p} \cdot r$, where $r$ is the *cofactor* — the part of $m$
not accounted for by $\mathcal{F}$. If $r = 1$, $m$ is fully $\mathcal{F}$-smooth; if $r > 1$, $m$
has a *large prime* cofactor outside the base.

**The large-prime variation.** In the basic sieve, only fully smooth relations are collected. In
the large-prime variation, partial relations (with one or two large prime cofactors) are also
collected. Two partial relations with the same large prime cofactor can be combined to form a full
relation. This variation improves the relation yield at the cost of more complex bookkeeping.

### Lenstra's Elliptic Curve Method (ECM)

#### The idea: Pollard's $p-1$, lifted onto a curve

Pollard's $p-1$ method factors $N$ by exploiting that if $p \mid N$ and $p - 1$ is $B$-smooth,
then for a base $a$, $a^k \equiv 1 \pmod{p}$ once $k = \mathrm{lcm}(1, \ldots, B)$ is a multiple
of the order of $a$ mod $p$. Then $\gcd(a^k - 1, N)$ reveals $p$.

The weakness: it only works when $p - 1$ happens to be smooth — a property of the single fixed
group $(\mathbb{Z}/p\mathbb{Z})^*$.

**Lenstra's insight (1987).** Replace the fixed group $(\mathbb{Z}/p\mathbb{Z})^*$ with the group
of points on a *random* elliptic curve $E$ over $\mathbb{Z}/N\mathbb{Z}$. The order $|E(\mathbb{F}_p)|$
varies with the curve choice and ranges over an interval around $p$ (Hasse's bound: $|p + 1 -
|E(\mathbb{F}_p)|| \leq 2\sqrt{p}$). If the chosen curve has $B$-smooth order mod some prime $p
\mid N$, then multiplying a starting point $P$ by $k = \prod q^e$ (prime powers $\leq B$) drives
$kP$ to the identity mod $p$ while it stays non-trivial mod $N$. The group law's modular inversion
then fails with a denominator sharing a factor with $N$, and $\gcd(\text{denominator}, N) = p$
falls out.

**The key advantage.** If one curve's order is not smooth, try another curve. ECM converts a
question of luck (is $p - 1$ smooth?) into a question one can re-roll (is *some* curve's order
smooth?).

#### The group-order smoothness argument

**Theorem (ECM expected complexity).** *The expected number of curves ECM must try before finding
a factor $p$ of $N$ is $O(\rho(u))^{-1}$, where $u = \log p / \log B$ and $\rho$ is the Dickman
function. The expected running time per curve is $O(B \log B \log N)$ (for stage 1). The total
expected running time is*

$$O\!\left(B \log B \log N \cdot \rho(u)^{-1}\right) = L_p\!\left[\frac{1}{2},\, \sqrt{2}\right]
\cdot \mathrm{poly}(\log N)$$

*when $B$ is chosen optimally as $B = L_p[1/2, 1/\sqrt{2}]$.*

*Proof sketch.* The probability that a random curve has $B$-smooth order mod $p$ is approximately
$\rho(u)$ (by the Canfield–Erdős–Pomerance theorem applied to the interval $[p + 1 - 2\sqrt{p},
p + 1 + 2\sqrt{p}]$). The optimal $B$ balances the per-curve cost $O(B \log B \log N)$ against the
number of curves $\rho(u)^{-1}$. Setting $B = L_p[1/2, c]$ and optimising over $c$ gives the
stated bound. See Lenstra [L87] and Crandall–Pomerance [CP05, §7.4] for the full argument. $\square$

**The subexponential complexity.** ECM runs in $L_p[1/2, \sqrt{2}]$ time in the size of the
*smallest prime factor* $p$, not in the size of $N$. This makes ECM the right tool for splitting
off small-to-medium prime factors: it is subexponential in $p$, but the cost grows with $p$, so it
is not competitive with NFS for large factors.

#### Montgomery-form curves and the $(X:Z)$ ladder

ECM uses *Montgomery-form curves* $Bv^2 = u^3 + Au^2 + u$ with the *Suyama parameterisation* to
generate a valid (curve, starting point) pair from a single integer $\sigma$. Montgomery form
admits projective $(X:Z)$ coordinates that omit the $Y$-coordinate entirely: scalar multiplication
needs only the $x$-coordinate, computed by the *Montgomery ladder* using *differential addition*
(given $P$, $Q$, and $P - Q$, compute $P + Q$) and doubling.

**Why Montgomery form?** The Montgomery ladder is *uniform* (no branch on the bit of the scalar)
and *efficient* (differential addition uses fewer field operations than the general addition
formula). The $Z = 0$ point represents the identity; a non-invertible denominator in the group law
signals that $\gcd(Z, N) > 1$, which is the factor.

**The Suyama parameterisation.** Given $\sigma$, compute the curve constant $A_{24} = (A+2)/4$ and
a starting point, returning early with a factor if any of the constructions hits a non-invertible
denominator mod $N$ (itself a lucky factor find). The parameterisation ensures that the starting
point has a specific structure that makes stage 1 more likely to succeed.

#### Stage 1 and stage 2

**Stage 1.** Compute $Q = kP$ with $k = \prod \{q^{\lfloor \log_q B_1 \rfloor} : q \text{ prime}
\leq B_1\}$ — every prime power up to $B_1$. After the ladder, test $\gcd(Z, N)$; a non-trivial
gcd is the factor. Stage 1 succeeds when $|E(\mathbb{F}_p)|$ is fully $B_1$-smooth.

**Stage 2.** The standard refinement for the common case where $|E(\mathbb{F}_p)|$ is $B_1$-smooth
*except for one* prime factor $q \in (B_1, B_2]$. Rather than raising $B_1$ (which re-does all
the work), stage 2 walks the multiples $qQ$ for primes $q \in (B_1, B_2]$ using differential
addition with a fixed step, accumulates the product of their $Z$-coordinates, and takes a *single*
gcd with $N$ at the end. This catches the one-large-prime case cheaply.

**The two-stage structure** is the ECM analogue of the large-prime variation in NFS sieving: both
extend the reach of the algorithm by handling the case where the "smooth" condition fails by one
prime factor.

### Tonelli–Shanks: square roots mod $p$

#### The problem

Given a prime $p$ and a quadratic residue $a$ (i.e. $\left(\frac{a}{p}\right) = 1$), find $r$ with
$r^2 \equiv a \pmod{p}$.

#### The 2-Sylow structure argument

The multiplicative group $(\mathbb{Z}/p\mathbb{Z})^*$ is cyclic of order $p - 1$. Write $p - 1 =
Q \cdot 2^S$ with $Q$ odd. The *2-Sylow subgroup* of $(\mathbb{Z}/p\mathbb{Z})^*$ is the unique
subgroup of order $2^S$.

**The easy case: $p \equiv 3 \pmod{4}$ (i.e. $S = 1$).** Here $r = a^{(p+1)/4}$ is a square root,
because $r^2 = a^{(p+1)/2} = a \cdot a^{(p-1)/2} = a \cdot 1 = a$ (using Euler's criterion for
a residue). One exponentiation, no search.

**The general case: $p \equiv 1 \pmod{4}$ (i.e. $S \geq 2$).** The Tonelli–Shanks algorithm
maintains a tuple $(M, c, t, r)$ where:
- $M$ is the current "error exponent" (starts at $S$),
- $c$ is a power of a quadratic non-residue $z$ (so $c^{2^{M-1}} = -1$),
- $t = a^Q$ (the "error term": $t^{2^{M-1}} = 1$ iff $r^2 = a$),
- $r = a^{(Q+1)/2}$ (the current candidate square root).

At each step, find the smallest $i$ such that $t^{2^i} = 1$. If $i = 0$, then $t = 1$ and $r^2 =
a$ — done. Otherwise, update:

$$c \leftarrow c^{2^{M-i-1}}, \quad t \leftarrow t \cdot c^2, \quad r \leftarrow r \cdot c,
\quad M \leftarrow i.$$

This reduces the error exponent $M$ at each step, so the algorithm terminates in at most $S - 1$
iterations.

**Finding the quadratic non-residue $z$.** The trial search tests $z = 2, 3, 5, \ldots$ until
$\left(\frac{z}{p}\right) = -1$. Non-residues have density $1/2$, so the expected number of trials
is 2. There is no known fast deterministic way to produce a non-residue (it is related to open
problems about the least quadratic non-residue under GRH), so the trial loop is the standard
practical choice.

**Theorem (Tonelli–Shanks correctness).** *The algorithm terminates in $O(S^2)$ multiplications
and returns $r$ with $r^2 \equiv a \pmod{p}$.*

*Proof sketch.* The invariant $r^2 \equiv a \cdot t \pmod{p}$ is maintained throughout. The error
exponent $M$ strictly decreases at each step (from $M$ to $i < M$), so the algorithm terminates.
At termination, $t = 1$ and $r^2 \equiv a$. $\square$

**Why this matters.** Tonelli–Shanks is used in this project for curve-point recovery (given an
$x$-coordinate, find the $y$-coordinate satisfying $y^2 = x^3 + ax + b$) and in the GNFS
square-root step (recovering a square root in the number field). The 2-Sylow structure argument
is the key insight: the algorithm works by iteratively reducing the "error" in the 2-Sylow
subgroup until the error vanishes.

### Cross-reference

For the phase-by-phase implementation of the α-substrate — the code, the KAT summary, the
downstream consumption table, and the scale-disconnect annotations — see
`shared/numth/docs/PEDAGOGY.md`. That chapter is the code-tour sibling to this one: it assumes
the reader knows the mathematics (this chapter) and focuses on the realisation in Rust.

---

## References

1. **Pollard, J. M. (1978).** "Monte Carlo methods for index computation (mod p)." *Mathematics of
   Computation*, 32(143), 918–924.

2. **Teske, E. (1998).** "Speeding up Pollard's rho method for computing discrete logarithms."
   *Algorithmic Number Theory Symposium (ANTS-III)*, LNCS 1423, 541–554.

3. **van Oorschot, P. C., and Wiener, M. J. (1999).** "Parallel collision search with
   cryptanalytic applications." *Journal of Cryptology*, 12(1), 1–28.

4. **Gallant, R. P., Lambert, R. J., and Vanstone, S. A. (2001).** "Faster point multiplication on
   elliptic curves with efficient endomorphisms." *Advances in Cryptology — CRYPTO 2001*, LNCS
   2139, 190–200.

5. **Lenstra, H. W. Jr. (1987).** "Factoring integers with elliptic curves." *Annals of
   Mathematics*, 126(3), 649–673.

6. **Montgomery, P. L. (1987).** "Speeding the Pollard and elliptic curve methods of
   factorization." *Mathematics of Computation*, 48(177), 243–264.

7. **Canfield, E. R., Erdős, P., and Pomerance, C. (1983).** "On a problem of Oppenheim concerning
   'Factorisatio Numerorum'." *Journal of Number Theory*, 17(1), 1–28. (The smooth-number density
   theorem.)

8. **Silverman, J. H. (2009).** *The Arithmetic of Elliptic Curves*. 2nd ed. Springer.

9. **Neukirch, J. (1999).** *Algebraic Number Theory*. Springer.

10. **Crandall, R., and Pomerance, C. (2005).** *Prime Numbers: A Computational Perspective*. 2nd
    ed. Springer.

11. **Davenport, H. (2000).** *Multiplicative Number Theory*. 3rd ed. Springer.

12. **Granville, A. (2008).** "Smooth numbers: computational number theory and beyond." In
    *Algorithmic Number Theory: Lattices, Number Fields, Curves and Cryptography*, MSRI
    Publications 44.

13. **Barbulescu, R., Gaudry, P., Joux, A., and Thomé, E. (2014).** "A heuristic quasi-polynomial
    algorithm for discrete logarithm in finite fields of small characteristic." *Advances in
    Cryptology — EUROCRYPT 2014*, LNCS 8441, 1–16.

14. **Shor, P. W. (1994).** "Algorithms for quantum computation: discrete logarithms and
    factoring." *Proceedings of the 35th Annual Symposium on Foundations of Computer Science*,
    124–134.

15. **Bos, J. W., Kleinjung, T., Niederhagen, R., and Schwabe, P. (2012).** "ECC2K-130 on Cell
    CPUs." *Progress in Cryptology — AFRICACRYPT 2010*, LNCS 6055, 225–242.

16. **Alford, W. R., Granville, A., and Pomerance, C. (1994).** "There are infinitely many
    Carmichael numbers." *Annals of Mathematics*, 139(3), 703–722.

17. **Sorenson, J., and Webster, J. (2017).** "Strong pseudoprimes to twelve prime bases."
    *Mathematics of Computation*, 86(304), 985–1003.
