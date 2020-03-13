#include <vector>
class MCBM {
private:
	std::vector<int> match, vis;
	std::vector<int> g[200100];
	int Aug(int L) {
		if (vis[L]++) return 0;
		for (auto &R : g[L]) {
			if (match[R] == -1 || Aug(match[R])) {
				match[R] = L;return 1;
			}
		}
		return 0;
	}
public:
	void add_edge(int u, int v) {
		g[u].push_back(v);
		g[v].push_back(u);
	}
	int solve() {
		int MCBM = 0;
		match.assign(V, -1);
		forn(L, 0, n - 1) {
			vis.assign(V, 0);
			MCBM += Aug(L);
		}
		return MCBM
	}
};


/**
Hall's marriage theorem

1. (Combinatorial Hall’s) Let S1, S2, . . . , Sm be sets. A transversal is an ordered
set (s1, s2, . . . , sm) such that each of the si
’s are different and si ∈ Si
. Prove
that there exists a transversal if and only if the union of any k sets has at least
k elements.
2. (K¨onig’s Marriage Theorem) Prove that if all the vertices of a bipartite graph
have the same degree, then it has a perfect matching.
3. (Bollob´as [2]) A bipartite graph G has vertex sets X and Y . Let A be the set of
vertices of G with the largest degree. Prove there exists a matching that covers
A ∩ X. Why does this imply the above problem?
4. We have a regular deck of 52 playing cards, with exactly 4 cards of each of the
13 ranks. The cards have been randomly dealt into 13 piles, each with 4 cards in
it. Prove that there is a way to take a card from each pile so that after we take
a card from every pile, we have exactly a card of every rank.
Prove that, in fact, we can go further: after taking a card of every rank, there
are 3 cards left in each pile. We can then take a card of every rank once more,
leaving 2 cards in each pile. Finally, we do it once more, and the remaining card
in each pile must be of every rank.
5. (Putnam 2012/B3 [3]) Suppose 2m teams play in a round-robin tournament.
Over a period of 2m − 1 days, every team plays every other team exactly once.
There are no ties. Show that for each day we can select a winning team, without
selecting the same team twice.
6. A class of 100 is participating in an oral exam. The committee consists of 25
members. Each student is interviewed by one member of the committee. It is
known that each student likes at least 10 committee members. Prove that we
can arrange the exam schedule such that each student is interviewed by one of
the committee members that he likes, and each committee member interviews at
most 10 students.
5.2 Involved
1. Let G be a connected bipartite graph with bipartite sets X and Y . Suppose that
for any two adjacent vertices x ∈ X and y ∈ Y , we have deg(x) ≥ deg(y). Prove
that G has a matching that covers X.
2. (Making Latin squares) We are given an n × k board with k < n such that in
every square there is a number from 1 to n. We know that in every column and
12
Hall’s marriage theorem Carl Joshua Quines
every row no number repeats itself. Show that we can extend this to an n × n
board with a number from 1 to n in each square, such that in every row and
every column no number repeats itself.
3. (Baltic Way 2013/6 [4]) Santa Claus has at least n gifts for n children. For
i ∈ {1, 2, ..., n}, the ith child considers xi > 0 of these items to be desirable.
Assume that
1
x1
+ · · · +
1
xn
≤ 1.
Prove that Santa Claus can give each child a gift that this child likes.
4. (Bollob´as [2]) All the vertices of a bipartite graph have the same degree d. What
is the maximum number of edges we can remove to ensure the resulting graph
has a perfect matching?
5. (IMC 2011/2/2b [5]) An alien race has three genders: male, female and emale.
A married triple consists of three persons, one from each gender, who all like
each other. Any person is allowed to belong to at most one married triple. A
special feature of this race is that feelings are always mutual – if x likes y, then
y likes x.
The race is sending an expedition to colonize a planet. The expedition has n
males, n females, and n emales. It is known that every expedition member likes
at least k persons of each of the two other genders. The problem is to create
as many married triples as possible to produce healthy offspring so the colony
could grow and prosper.
Show that if k ≥ 3n/4, then it is always possible to create n disjoint married
triples, thus marrying all of the expedition members.
6. (K¨onig’s Theorem) An m × n table is filled with 0s and 1s. Call a column or a
row a line. Prove that the minimal number of lines containing all the 1s of A is
precisely the maximal number of 1s with no two in the same line.
5.3 Challenging
1. (AMSP C3 2014 [6]) An n×n table is filled with 0 and 1 so that if we choose any
n cells, no two of them on the same row or column, then at least one contains
1. Prove that we can find i rows and j columns so that i + j ≥ n + 1 and their
intersection contains only 1s.
2. (Vietnam TST 2001/3 [7]) A club has 42 members. Suppose that for any 31
members in this club, there exists a boy and a girl among these 31 members who
know each other. Show that we can form 12 disjoint pairs of people, each pair
having one boy and one girl, such that the people in each pair know each other.
3. (ISL 2010/C2 [8]) On some planet, there are 2N countries, where N ≥ 4. Each
country has a flag N units wide and one unit high composed of N fields of size
1 × 1, each field being either yellow or blue. No two countries have the same flag.
We say that a set of N flags is diverse if these flags can be arranged into an
N × N square so that all N fields on its main diagonal will have the same color.
Determine the smallest positive integer M such that among any M distinct flags,
there exist N flags forming a d
Hall’s marriage theorem Carl Joshua Quines
4. (WOOT 2011) A table has m rows and n columns with m, n > 1. The following
permutations of its mn elements are permitted: any permutation leaving each
element in the same row (a “horizontal move”), and any permutation leaving
each element in the same column (a “vertical move”). Find the smallest integer
k such that any permutation of the mn elements can be realized by at most k
permitted moves.
5. (ISL 2012/C5 [9]) The columns and the rows of a 3n × 3n square board are
numbered 1, 2, ..., 3n. Every square (x, y) with 1 ≤ x, y ≤ 3n is colored asparagus,
byzantium or citrine according as the modulo 3 remainder of x + y is 0, 1 or 2
respectively. One token colored asparagus, byzantium or citrine is placed on
each square, so that there are 3n
2
tokens of each color.
Suppose that one can permute the tokens so that each token is moved to a
distance of at most d from its original position, each asparagus token replaces
a byzantium token, each byzantium token replaces a citrine token, and each
citrine token replaces an asparagus token. Prove that it is possible to permute
the tokens so that each token is moved to a distance of at most d + 2 from its
original position, and each square contains a token with the same color as the
square.
6. (ISL 2006/C6 [10]) A holey triangle is an upward equilateral triangle of side
length n with n upward unit triangular holes cut out. A diamond is a 60◦ − 120◦
unit rhombus. Prove that a holey triangle T can be tiled with diamonds if and
only if the following condition holds: Every upward equilateral triangle of side
length k in T contains at most k holes, for 1 ≤ k ≤ n.
14
Hall’s marriage theorem Carl Joshua Quines
6 Hints
6.1 Straightforward
1. Let the elements of X be the sets and the elements of Y be the elements of the
union of all the sets.
2. First show X and Y have the same number of vertices.
3. Try contradiction. Count the degrees of the vertices.
4. The sets are the thirteen ranks and the thirteen piles.
5. The sets are the 2m teams and the 2m − 1 days. Try contradiction.
6. Instead of 25 commitee members, consider each member having 10 “time slots”,
to account for the fact each member interviews at most 10 students.
6.2 Involved
1. How can we use deg(x) ≥ deg(y) to prove the matching condition?
2. Use induction: create n × (k + 1) from n × k. Try k = n − 1 case and generalize.
3. The obvious choice of sets works; prove the matching condition by contradiction.
4. The answer is d − 1. Why won’t deleting edges break the matching condition?
5. First marry males to females, then marry the male-female pairs to emales.
6. The sets are rows and columns. Choose vertices adjacent to all edges, show by
Hall’s some matching covers these vertices.
6.3 Challenging
1. The sets are rows and columns. Prove the contrapositive.
2. Try contradiction. Use the condition to force adjacent vertices.
3. The answer is M = 2N−2 + 1. To prove the upper bound, construct two graphs
with the rows and columns as bipartite sets: one graph has an edge if the square
is yellow, the other has an edge if the square is blue.
4. We only need three moves. Get all the pieces in their proper rows in two moves.
5. Without loss of generality, we only need to prove that the A-tokens can be moved
to distinct A-squares that have distance at most d + 2 from their original places.
Match tokens to squares.
6. We’re matching downward unit triangles to adjacent upward unit triangles. For
necessity, consider largest set of downward triangles that break condition. These
triangles can’t be disjoint, use this to construct contradiction
*/