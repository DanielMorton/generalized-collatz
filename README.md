# The Collatz Conjecture
The Collatz Conjecture is a famous, and deceptively simple, unsolved problem in mathematics. The algorithm it concerns is simplicity itself; if a number is odd, multiply by 3 and add 1, 
otherwise divide by 2. The conjecture states that for any starting value the algorithm will eventually reach 1. The problem has remained unsolved since 1937.

There is a heuristic argument that suggests why the conjecture should be true. The Collatz algorithm can be reformulated as follows. Start with an odd number. Multiply by 3 and add 1. Then divide by
two until the number is odd again.  Repeat. From this perspective it is obvious that it is sufficient to only consider
odd starting numbers since even starting values will always be divided until they are odd. After applying the 3n+1 step the output will always be even. Thus starting with a value n, half the time
the next odd number will be (3n + 1)/2, half of the remaining time (25%) it will be (3n + 1)/4, and half of the times left (12.5%) it will
be (3n + 1)/8. Taking the expected value we see that, on average, the next odd number should be 3/4 of it's predecessor, and
thus the sequence *should* at least converge to a cycle. In light of this, and the numerical evidence, we can expect
1 -> 4 -> 2 -> 1 (or 1 -> 1 excluding even numbers) to be the only cycle.

The conjecture has been confirmed up to 10<sup>22</sup>.

## Extending the  Collatz Conjecture to different divisors.

What if, instead of (3n + 1)/2, we had (4n + 1)/3 and (4n + 2)/3. Start with a number not divisible by 3, multiply by 4, and then add 1 or 2 to make the result divisible by 3. Then divide
by three until the result is not divisible by three. By an argument similar to the one above this should converge to a cycle. At least up to 2<sup>27</sup> it does, with two possible cycles.

| Cycle  | Frequency |
|:-------|----------:|
| 1 -> 2 |     3.3% |
| 7 -> 10 -> 14 -> 19 -> 26 -> 35 -> 47 |     96.7% |

If, instead, we replace 4 with 5 we also get two cycles. Noteably, 1 is not part of either of them.

| Cycle  | Frequency |
|:-------|----------:|
| 4 -> 7 |     61.5% |
| 8 -> 14 |     38.5% |

The conject is that for any prime p and positive number a where gcd(a,p) = 1 and a/p<sup>e + p/(p-1)</sup> < 1 then the following algorithm always terminates in a finite cycle
for any starting value n.

1. If n is not divisible by p, multiply by a and add the smallest number x so that a*n+x is divisible by p<sup>e</sup>. Else go to step 2.
2. Repeatedly divide by p until the result is not divisible by p.
3. Repeat, starting at step 1.

The rather complicated relationship between a and p in the inequality merely says that as a increases larger powers of p are necessary to ensure convergence.

This program evaluates the cycle for any values of a and p. The case where p = 2 is identical to the that covered by https://github.com/DanielMorton/extended-collatz

This program merely extends the same theory to a general denominator. The only difference in execution is that the value p must be specified.

| Command       | Definition                                                                           |
|:--------------|:-------------------------------------------------------------------------------------|
| -n            | Run algorithm for all starting values less than this number                          |
| -p            | The divisor for the algorithm.
| -s --start    | Lowest value of $a$.                                        |
| -e --end      | Highest value of $a$.                                                                |
| --write-cycle | Outputs a csv of cycles for each value of $a$.                                       |
| --write-table | Outputs a csv of minimum cycle values for each starting value and each value of $a$. | 
