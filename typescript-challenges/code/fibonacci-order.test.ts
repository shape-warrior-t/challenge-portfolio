/**
 * Tests for Fibonacci Order:
 * - Comparing the output of `fibonacciOrder` against a slower, simpler oracle implementation.
 * - Unit tests for more special cases not likely to be covered by the property-based test.
 * - Unit tests to ensure that `fibonacciOrder` throws on invalid input.
 *
 * NOTE: generating an `n`-bonacci-like list `arr` and testing that `fibonacciOrder(arr) === n`
 * only works if we can guarantee that `arr` is not `m`-bonacci-like for `m < n`.
 * The oracle approach seems simpler to me than fulfilling the guarantee,
 * so that's what we're going with.
 */

import { count, find, range } from 'itertools';
import { expect, test } from '@jest/globals';
import assert from 'node:assert/strict';
import fc from 'fast-check';
import fibonacciOrder from './fibonacci-order';

/**
 * Return the sum of the elements in `arr` at the `n` indices before `index`.
 *
 * The `n` indices before `index` must all be in bounds.
 */
function sumOfPreviousNElements(
    arr: bigint[],
    index: number,
    n: number,
): bigint {
    const previousNElements = arr.slice(index - n, index);
    assert.equal(previousNElements.length, n);
    return previousNElements.reduce((a, b) => a + b, 0n);
}

/** Arbitrary for `n`-bonacci-like lists for randomly chosen `n`. */
const nBonacciLikeList = fc
    .tuple(fc.array(fc.bigInt(1n, 100n), { minLength: 1 }), fc.nat(10))
    .map(([arr, numAdditionalElements]) => {
        const n = arr.length;
        arr = [...arr];
        for (const _ of range(numAdditionalElements)) {
            const indexOfNewElement = arr.length;
            arr.push(sumOfPreviousNElements(arr, indexOfNewElement, n));
        }
        return arr;
    });

test('comparison against oracle', () => {
    fc.assert(
        fc.property(nBonacciLikeList, (arr) => {
            const actual = fibonacciOrder(arr);
            const expected = fibonacciOrderOracle(arr);
            expect(actual).toStrictEqual(expected);
        }),
    );
});

function fibonacciOrderOracle(arr: bigint[]): number {
    return find(count(), (n) => isNBonacciLikeList(arr, n))!;
}

function isNBonacciLikeList(arr: bigint[], n: number): boolean {
    return arr.every(
        (x, i) => i < n || sumOfPreviousNElements(arr, i, n) === x,
    );
}

test.each([
    // The empty list is the only 0-bonacci-like input.
    [[], 0],
    // From the example runthrough
    [[1n, 1n, 2n, 3n, 6n, 13n, 25n, 49n], 5],
    /*
        An earlier implementation failed this test case, returning 3 based on the [10, 18, 33] slice
        instead of realizing that the [3, 5] slice prevents the array from being 3-bonacci-like.
    */
    [[1n, 1n, 2n, 3n, 5n, 10n, 18n, 33n], 8],
])('special case: %p', (arr, expected) => {
    expect(fibonacciOrder(arr)).toStrictEqual(expected);
});

test.each([
    [[0n, 1n, 1n, 2n, 3n]],
    [[1n, -2n, 1n]],
    [[5n, 4n, 3n, 2n, 1n, 0n]],
])('throws upon encountering non-positive elements: %p', (arr) => {
    expect(() => fibonacciOrder(arr)).toThrow();
});
