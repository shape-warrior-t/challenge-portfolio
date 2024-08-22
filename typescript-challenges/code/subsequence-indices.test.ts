/**
 * Tests for Subsequence Indices, ensuring that the output of
 * `subsequenceIndices(arr, sub)` is always correct.
 *
 * If `subsequenceIndices(arr, sub)` returns a list of indices `[i_0, i_1, ...]`,
 * we can always verify correctness by checking that the list is strictly ascending
 * and that `[arr[i_0], arr[i_1], ...]` represents the same list as `sub`.
 * Based on that, we have the following tests:
 * - When `sub` is guaranteed to be a subsequence of `arr`,
 *   `subsequenceIndices(arr, sub)` returns the correct list of indices and never returns null.
 * - When `arr` and `sub` are arbitrary,
 *   `subsequenceIndices(arr, sub)` either returns the correct list of indices or returns null.
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

function expectIndicesAreCorrect<T>(
    indices: number[],
    arr: T[],
    sub: T[],
): void {
    expectStrictlyAscending(indices);
    expect(indices.map((i) => arr[i])).toStrictEqual(sub);
}

function expectStrictlyAscending(arr: number[]): void {
    for (const [a, b] of pairwise(arr)) {
        expect(a).toBeLessThan(b);
    }
}

/** Arbitrary for a list together with a random subsequence of that list. */
const listAndSubsequence: fc.Arbitrary<[number[], number[]]> = fc
    .array(fc.integer({ min: 0, max: 9 }))
    .chain((arr) => fc.subarray(arr).map((sub) => [arr, sub]));

test('`sub` subsequence of `arr`', () => {
    fc.assert(
        fc.property(listAndSubsequence, ([arr, sub]) => {
            const indices = subsequenceIndices(arr, sub);
            expect(indices).not.toBeNull();
            expectIndicesAreCorrect(indices!, arr, sub);
        }),
    );
});

/**
 * Arbitrary for list inputs to `subsequenceIndices`.
 *
 * Elements are taken from a set of size 3 in order to raise the chances of
 * `sub` being a subsequence or near-subsequence of `arr`.
 */
function list(maxLength: number): fc.Arbitrary<string[]> {
    return fc.array(fc.constantFrom('a', 'b', 'c'), { maxLength });
}

test('`arr` and `sub` arbitrary', () => {
    fc.assert(
        fc.property(list(10), list(5), (arr, sub) => {
            const indices = subsequenceIndices(arr, sub);
            if (indices !== null) {
                expectIndicesAreCorrect(indices, arr, sub);
            }
        }),
    );
});
