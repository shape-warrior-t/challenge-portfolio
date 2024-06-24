/**
 * Tests for Extract Strings:
 * - Testing `extractStrings` with input-output pairs
 * formed through the inverse process of incorporating a list of strings into text.
 * - Unit tests to ensure that `extractStrings` throws on invalid input.
 */

import { expect, test } from '@jest/globals';
import extractStrings from './extract-strings';
import fc from 'fast-check';
import { roundrobin } from 'itertools';

/**
 * A non-string portion of a text input to `extractStrings`.
 *
 * Cannot contain single quotes, since those delimit strings.
 */
const nonStringPortion = fc.stringOf(fc.char().filter((c) => c !== "'"));

/**
 * A string to be extracted from text input to `extractStrings`.
 *
 * Single quotes are more likely to appear than other characters
 * in order to better test the escaping-by-doubling aspect of the problem.
 */
const extractedString = fc.stringOf(fc.oneof(fc.constant("'"), fc.char()));

/**
 * Text together with the strings to be extracted from the text.
 *
 * Formed by generating the extracted strings first, then incorporating them into text.
 */
const textAndExtractedStrings: fc.Arbitrary<[string, string[]]> = fc
    .array(extractedString)
    .chain((extractedStrings) => {
        const n = extractedStrings.length;
        return (
            fc
                .array(nonStringPortion, {
                    minLength: n + 1,
                    maxLength: n + 1,
                })
                /*
                    The text input `'a''b'` contains one string, `a'b`,
                    not two strings, `a` and `b`.
                    Thus, strings in the text need to be separated by at least one character,
                    lest they be identified as a single string.
                    This means the non-string portions of the text must be nonempty,
                    with the exception of the first and last portions.
                */
                .filter((nonStringPortions) =>
                    nonStringPortions.every(
                        (s, i) => i === 0 || i === n || s.length > 0,
                    ),
                )
                .map((nonStringPortions) => {
                    const interleavedTextPortions = roundrobin(
                        nonStringPortions,
                        extractedStrings.map((s) => repr(s)),
                    );
                    const text = [...interleavedTextPortions].join('');
                    return [text, extractedStrings];
                })
        );
    });

/** Return the given string as it would be represented in text input to `extractStrings`. */
function repr(s: string): string {
    const withEscapedQuotes = [...s]
        .map((char) => (char === "'" ? "''" : char))
        .join('');
    return `'${withEscapedQuotes}'`;
}

test('generated input-output pairs', () => {
    fc.assert(
        fc.property(textAndExtractedStrings, ([text, expected]) => {
            expect(extractStrings(text)).toStrictEqual(expected);
        }),
    );
});

test.each([["'"], ["'abc"], ["abc'"], ["a'b'c'd"], ["'''abc'''def'''"]])(
    'throws for input with an unmatched single quote: %p',
    (text) => {
        expect(() => extractStrings(text)).toThrow();
    },
);
