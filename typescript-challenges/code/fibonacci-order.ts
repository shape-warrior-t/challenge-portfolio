/**
 * Problem (modified from
 * https://codegolf.stackexchange.com/questions/243477/how-many-bonacci-like-is-this-sequence):
 * For a natural number `n`, a list of numbers is `n`-bonacci-like
 * if each element is the sum of the previous `n` elements (other than the first `n` elements).
 * Given a list of positive integers, find the smallest `n` for which the list is `n`-bonacci-like.
 */

import assert from 'node:assert/strict';

/**
 * Return the smallest `n` for which `arr` is `n`-bonacci-like,
 * as defined in the problem description.
 *
 * Note that every list of length `n` is, at minimum, vacuously `n`-bonacci-like.
 *
 * Every element of `arr` must be strictly positive.
 */
export default function fibonacciOrder(arr: bigint[]): number {
    /*
        Example runthrough: fibonacciOrder([1n, 1n, 2n, 3n, 6n, 13n, 25n, 49n])
        (BigInts presented as regular numbers to avoid writing the `n` suffix all the time.)

        arr: [1, 1, 2, 3, 6, 13, 25, 49]    possible outputs: [0, 1, 2, 3, 4, 5, 6, 7, 8]
              ^                                                x  -----------------------
         sum [] != 1                                           x

        arr: [1, 1, 2, 3, 6, 13, 25, 49]    possible outputs: [   1, 2, 3, 4, 5, 6, 7, 8]
                 ^                                                ?  --------------------
         sum [1] = 1                                              ?

        arr: [1, 1, 2, 3, 6, 13, 25, 49]    possible outputs: [   1, 2, 3, 4, 5, 6, 7, 8]
                    ^                                             x  ?  -----------------
         sum    [1] != 2                                          x
         sum [1, 1]  = 2                                             ?

        arr: [1, 1, 2, 3, 6, 13, 25, 49]    possible outputs: [      2, 3, 4, 5, 6, 7, 8]
                       ^                                             ?  x  --------------
         sum    [1, 2]  = 3                                          ?
         sum [1, 1, 2] != 3                                             x

        arr: [1, 1, 2, 3, 6, 13, 25, 49]    possible outputs: [      2,    4, 5, 6, 7, 8]
                          ^                                          x     x  -----------
         sum       [2, 3] != 6                                       x
         sum [1, 1, 2, 3] != 6                                             x

        arr: [1, 1, 2, 3, 6, 13, 25, 49]    possible outputs: [               5, 6, 7, 8]
                             ^                                                ?  --------
         sum [1, 1, 2, 3, 6] = 13                                             ?

        arr: [1, 1, 2, 3, 6, 13, 25, 49]    possible outputs: [               5, 6, 7, 8]
                                 ^                                            ?  x  -----
         sum    [1, 2, 3, 6, 13]  = 25                                        ?
         sum [1, 1, 2, 3, 6, 13] != 25                                           x

        arr: [1, 1, 2, 3, 6, 13, 25, 49]    possible outputs: [               5,    7, 8]
                                     ^                                        ?     x  --
         sum       [2, 3, 6, 13, 25]  = 49                                    ?
         sum [1, 1, 2, 3, 6, 13, 25] != 49                                          x

        arr: [1, 1, 2, 3, 6, 13, 25, 49]    possible outputs: [               5,       8]
                                         ^                                    ^        ^
        Final result: 5

        (Reminder: every list of length `n` is `n`-bonacci-like,
        so the length of the list is always a possible output.)
    */
    if (!arr.every((x) => x > 0n)) {
        throw new Error(
            '`fibonacciOrder` only accepts lists of positive integers',
        );
    }
    /*
        For efficiency, instead of tracking possible outputs,
        we track array slices and their sums.
        The length of each slice is the same as the corresponding possible output,
        and we filter out and add in slices instead of filtering and adding in
        their corresponding possible outputs.
        The left side of the example runthrough shows how `slices` changes on each iteration:
        slices that are not filtered out move to the right by one index,
        and a new, long slice is added to the end of `slices`.
    */
    let slices = [];
    // For efficiency when adding in the new, long slice on every iteration.
    let expandingSlice = SumSlice.empty(arr);
    slices.push(expandingSlice);
    for (const x of arr) {
        assert(slices.length <= 2);
        slices = slices
            /*
                Since elements are required to be positive, there is at most one `n`
                for which the current element is the sum of the previous `n` elements --
                and therefore, at most one slice that will survive the filtering process.
            */
            .filter((slice) => slice.sum === x)
            .map((slice) => slice.moveRight());
        expandingSlice = expandingSlice.expandRight();
        slices.push(expandingSlice);
    }
    return slices[0]!.length;
    /*
        This function completes in `O(n)` time in the worst case:
        - Note that all operations of the `SumSlice` class complete in `O(1)`.
        - The input validation completes in `O(n)`.
        - Everything outside the for loop completes in `O(1)`.
        - The for loop completes in `O(n)`,
          as it takes `n` iterations to complete, and each iteration is `O(1)`:
            - Since `slices` has at most two elements,
              mapping and filtering over it takes `O(1)` time.
            - Everything else completes in `O(1)`.
    */
}

/** A contiguous slice of an array that keeps track of its own sum. */
class SumSlice {
    /**
     * Create a new slice.
     *
     * Preconditions:
     * - The slice is fully in bounds.
     * - The provided sum is accurate.
     */
    private constructor(
        /** The array that the slice is taken from. */
        readonly arr: bigint[],
        /** The (inclusive) start index of the slice. */
        readonly start: number,
        /** The (exclusive) end index of the slice. */
        readonly end: number,
        /** The sum of the elements in the slice. */
        readonly sum: bigint,
    ) {}

    /** Return a new, empty slice at the start of the given array. */
    static empty(arr: bigint[]): SumSlice {
        return new SumSlice(arr, 0, 0, 0n);
    }

    /** The length of the slice. */
    get length(): number {
        return this.end - this.start;
    }

    /**
     * Return a new slice whose edges are one index to the right compared to this slice.
     *
     * <A B C D E F> --> <A B C D E F>
     *   [^    ]^            [^    ]^
     *
     * Precondition: the new slice is fully in bounds.
     */
    moveRight(): SumSlice {
        const { arr, start, end, sum } = this;
        return new SumSlice(
            arr,
            start + 1,
            end + 1,
            sum - arr[start]! + arr[end]!,
        );
    }

    /**
     * Return a new slice whose right edge is one index to the right compared to this slice.
     *
     * <A B C D E F> --> <A B C D E F>
     *   [^    ]^          [^      ]^
     *
     * Precondition: the new slice is fully in bounds.
     */
    expandRight(): SumSlice {
        const { arr, start, end, sum } = this;
        return new SumSlice(arr, start, end + 1, sum + arr[end]!);
    }
}
