/**
 * Problem (inspired by
 * https://codegolf.stackexchange.com/questions/5529/is-string-x-a-subsequence-of-string-y):
 * given two lists `arr` and `sub`, find a list of indices where
 * `sub` occurs as a (not necessarily contiguous) subsequence of `arr`.
 */

/**
 * Return a strictly ascending list of indices `[i_0, i_1, ...]`
 * such that `[arr[i_0], arr[i_1], ...]` represents the same list as `sub`,
 * or `null` if there is no such list.
 *
 * If there are multiple such lists, return one of them;
 * which specific list to return is left unspecified.
 *
 * Equality of elements is decided using `===`.
 */
export default function subsequence_indices<T>(
    arr: T[],
    sub: T[],
): number[] | null {
    /*
        Example runthroughs:

        subsequence_indices([3, 1, 4, 1, 5, 9], [4, 1, 9])
        arr: [3, 1, 4, 1, 5, 9]    sub: [4, 1, 9]
              ^                          ^
        arr: [3, 1, 4, 1, 5, 9]    sub: [4, 1, 9]
                 ^                       ^
        arr: [3, 1, 4, 1, 5, 9]    sub: [4, 1, 9]
                    ^                    ^
        arr: [3, 1, 4, 1, 5, 9]    sub: [4, 1, 9]
                    @  ^                    ^
        arr: [3, 1, 4, 1, 5, 9]    sub: [4, 1, 9]
                    @  @  ^                    ^
        arr: [3, 1, 4, 1, 5, 9]    sub: [4, 1, 9]
                    @  @     ^                 ^
        arr: [3, 1, 4, 1, 5, 9]    sub: [4, 1, 9]
                    @  @     @  ^                 ^
        Final result: [2, 3, 5]

        subsequence_indices([2, 6, 5], [2, 4])
        arr: [2, 6, 5]    sub: [2, 4]
              ^                 ^
        arr: [2, 6, 5]    sub: [2, 4]
              @  ^                 ^
        arr: [2, 6, 5]    sub: [2, 4]
              @     ^              ^
        arr: [2, 6, 5]    sub: [2, 4]
              @        ^           ^
        Final result: null
    */
    const indices = [];
    let arrIndex = 0;
    let subIndex = 0;
    while (true) {
        if (reachedEndOfList(sub, subIndex)) {
            return indices;
        }
        if (reachedEndOfList(arr, arrIndex)) {
            return null;
        }
        if (arr[arrIndex] === sub[subIndex]) {
            indices.push(arrIndex);
            arrIndex++;
            subIndex++;
        } else {
            arrIndex++;
        }
    }
}

function reachedEndOfList<T>(arr: T[], index: number): boolean {
    return index === arr.length;
}
