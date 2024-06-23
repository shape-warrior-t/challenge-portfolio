/**
 * Tests for Subsequence Indices, ensuring that the output of
 * `subsequenceIndices(arr, sub)` is always correct.
 *
 * Properties tested:
 * a. If `subsequenceIndices(arr, sub)` is a list of indices `[i_0, i_1, ...]`,
 * then the list is strictly ascending
 * and `[arr[i_0], arr[i_1], ...]` represents the same list as `sub`.
 * b. If `subsequenceIndices(arr, sub)` is null, then `sub` is not a subsequence of `arr` --
 * equivalently, if `sub` is a subsequence of `arr`,
 * then `subsequenceIndices(arr, sub)` is not null.
 *
 * NOTE: generating indices `[i_0, i_1, ...]` and testing that
 * `subsequenceIndices(arr, [arr[i_0], arr[i_1], ...])` is the same as `[i_0, i_1, ...]`
 * does not work, since there may be other lists of indices that produce the same subsequence.
 * For example, ['a', 'b'] may be produced from ['a', 'b', 'b', 'a']
 * by taking indices [0, 1] or by taking indices [0, 2],
 * so `subsequenceIndices(['a', 'b', 'b', 'a'], ['a', 'b'])` is allowed to return either result.
 */

import { expect, test } from '@jest/globals';
import fc from 'fast-check';
import { pairwise } from 'itertools';
import subsequenceIndices from './subsequence-indices';

/**
 * Arbitrary for list inputs to `subsequenceIndices`.
 *
 * Elements are taken from a set of size 2 in order to maximize the chances that,
 * given randomly generated `sub` and `arr`,
 * `sub` is a subsequence of `arr` and also has a reasonably high length.
 */
function list(maxLength: number): fc.Arbitrary<string[]> {
    return fc.array(fc.constantFrom('a', 'b'), { maxLength });
}

test('a. list returns are valid', () => {
    fc.assert(
        fc.property(list(10), list(5), (arr, sub) => {
            const indices = subsequenceIndices(arr, sub);
            if (indices === null) return;
            expectStrictlyAscending(indices);
            const subFromIndices = indices.map((i) => arr[i]);
            expect(subFromIndices).toStrictEqual(sub);
        }),
    );
});

function expectStrictlyAscending(arr: number[]): void {
    for (const [a, b] of pairwise(arr)) {
        expect(a).toBeLessThan(b);
    }
}

/** Arbitrary for a list together with a random subsequence of that list. */
const listAndSubsequence: fc.Arbitrary<[string[], string[]]> = list(10).chain(
    (arr) => {
        const inclusions = fc.array(fc.boolean(), {
            minLength: arr.length,
            maxLength: arr.length,
        });
        return inclusions.map((ithElementIncluded) => {
            const sub = arr.filter((_, i) => ithElementIncluded[i]);
            return [arr, sub];
        });
    },
);

test('b. returns non-null for subsequences', () => {
    fc.assert(
        fc.property(listAndSubsequence, ([arr, sub]) => {
            expect(subsequenceIndices(arr, sub)).not.toBeNull();
        }),
    );
});
