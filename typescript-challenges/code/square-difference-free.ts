/**
 * Problem (taken from
 * https://codegolf.stackexchange.com/questions/220159/first-sequence-with-no-square-differences):
 * Generate the lexicographically earliest square-difference-free infinite sequence
 * (OEIS A030193) -- in other words, the sequence where
 * each successive element is the smallest natural number
 * that is not a perfect square's distance away from any of the previous elements.
 * The first 10 elements are `[0, 2, 5, 7, 10, 12, 15, 17, 20, 22]`.
 */

import assert from 'node:assert/strict';

/**
 * Data type modelling a perfect square distance between two numbers.
 *
 * Consists of an origin integer (`origin`, available as `this.origin`),
 * an integer square root of a distance (`sqrtDist`, available as `this.sqrtDist`),
 * and a destination integer (`dest`, not directly available on the object)
 * such that `origin + sqrtDist^2 = dest`.
 */
interface SquareDistance {
    origin: bigint;
    sqrtDist: bigint;
}

/** Yield the elements of OEIS sequence A030193. */
export default function* squareDifferenceFree(): Generator<bigint, never> {
    /*
        Runthrough:
        start     {}
        include 0 {0 + 1^2 =  1}
        exclude 1 {0 + 2^2 =  4}
        include 2 {0 + 2^2 =  4, 2 + 1^2 =  3}
        exclude 3 {0 + 2^2 =  4, 2 + 2^2 =  6}
        exclude 4 {0 + 3^2 =  9, 2 + 2^2 =  6}
        include 5 {0 + 3^2 =  9, 2 + 2^2 =  6, 5 + 1^2 =  6}
        exclude 6 {0 + 3^2 =  9, 2 + 3^2 = 11, 5 + 2^2 =  9}
        include 7 {0 + 3^2 =  9, 2 + 3^2 = 11, 5 + 2^2 =  9, 7 + 1^2 =  8}
        exclude 8 {0 + 3^2 =  9, 2 + 3^2 = 11, 5 + 2^2 =  9, 7 + 2^2 = 11}
        exclude 9 {0 + 4^2 = 16, 2 + 3^2 = 11, 5 + 3^2 = 14, 7 + 2^2 = 11}
        ...
        Included: 0, 2, 5, 7, ...

        NOTE: the data in the above is organized in a way that
        prioritizes understandability over precise accuracy to the actual implementation.
        For efficiency, `{0 + 4^2 = 16, 2 + 3^2 = 11, 5 + 3^2 = 14, 7 + 2^2 = 11}`
        is actually organized as something closer to
        `{11 = 2 + 3^2 = 7 + 2^2, 14 = 5 + 3^2, 16 = 0 + 4^2}`.
    */
    const destToDistance = new Map<bigint, SquareDistance[]>();
    for (let dest = 0n; ; dest++) {
        if (!destToDistance.has(dest)) {
            yield dest;
            destToDistance.set(dest, [{ origin: dest, sqrtDist: 0n }]);
        }
        for (const { origin, sqrtDist } of destToDistance.get(dest)!) {
            assert(origin + sqrtDist ** 2n === dest);
            const nextSqrtDist = sqrtDist + 1n;
            const nextDest = origin + nextSqrtDist ** 2n;
            if (!destToDistance.has(nextDest)) {
                destToDistance.set(nextDest, []);
            }
            destToDistance
                .get(nextDest)!
                .push({ origin, sqrtDist: nextSqrtDist });
        }
        /*
            Not necessary for correctness,
            but it's more memory-efficient to delete entries that will no longer be accessed.
        */
        assert(destToDistance.delete(dest));
    }
}
