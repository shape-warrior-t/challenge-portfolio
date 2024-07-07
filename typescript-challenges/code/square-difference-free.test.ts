/** Output check for Square-difference-free. */

import { expect, test } from '@jest/globals';
import squareDifferenceFree from './square-difference-free';
import { take } from 'itertools';

test('sequence head matches', () => {
    // Taken from https://oeis.org/A030193
    const expected = [
        0, 2, 5, 7, 10, 12, 15, 17, 20, 22, 34, 39, 44, 52, 57, 62, 65, 67, 72,
        85, 95, 109, 119, 124, 127, 130, 132, 137, 142, 147, 150, 170, 177, 180,
        182, 187, 192, 197, 204, 210, 215, 238, 243, 249, 255, 257, 260, 262,
    ].map((n) => BigInt(n));
    const actual = take(expected.length, squareDifferenceFree());
    expect(actual).toStrictEqual(expected);
});
