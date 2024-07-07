/**
 * Property-based test for Split on New Chars,
 * ensuring that the output meets the specifications outlined in the challenge description.
 */

import { expect, test } from '@jest/globals';
import fc from 'fast-check';
import splitOnNewChars from './split-on-new-chars';
import { uniqueEverseen } from 'itertools';

/**
 * Arbitrary for inputs to `splitOnNewChars`.
 *
 * Characters are taken from a set of size 5
 * so that generated strings are more likely to repeat characters.
 */
const testString = fc.stringOf(fc.constantFrom('a', 'b', 'c', 'd', 'e'));

test('satisfies required properties', () => {
    fc.assert(
        fc.property(testString, (str: string) => {
            const arr = splitOnNewChars(str);
            expect(arr.join('')).toStrictEqual(str);
            expect(indicesOfSplit(arr)).toStrictEqual(
                charFirstOccurrences(str),
            );
        }),
    );
});

/**
 * Given a split of a string,
 * return the index at which each substring begins in the original string.
 *
 * Example: `['aa', 'baba', 'ccabac'] => [0, 2, 6]` \
 * In `'aababaccabac'` (`['aa', 'baba', 'ccabac'].join('')`), \
 * `'aa'` begins at index 0, `'baba'` begins at index 2, and `'ccabac'` begins at index 6.
 */
function indicesOfSplit(arr: string[]): number[] {
    const indices = [];
    let currIndex = 0;
    for (const substring of arr) {
        indices.push(currIndex);
        currIndex += substring.length;
    }
    return indices;
}

/**
 * Return the index of the first occurrence of each character in the given string.
 *
 * Example: `'aababaccabac' => [0, 2, 6]` \
 * `'a'` first occurs at index 0, `'b'` first occurs at index 2, and `'c'` first occurs at index 6.
 */
function charFirstOccurrences(str: string): number[] {
    return [...uniqueEverseen(str)].map((char) => str.indexOf(char));
}
