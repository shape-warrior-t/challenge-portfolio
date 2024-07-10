/**
 * Problem (taken from
 * https://codegolf.stackexchange.com/questions/156170/split-string-on-first-occurrence-of-each-character):
 * transform a string `str` into a list of strings `arr` such that:
 * - `arr.join('') === str`
 * - A new substring begins precisely when
 * a previously unencountered character is encountered in the string.
 *
 * For example, `'aababaccabac'` should be split into `['aa', 'baba', 'ccabac']`.
 */

/** Split the given string at newly encountered characters, as per the challenge description. */
export default function splitOnNewChars(str: string): string[] {
    /*
        Note: V8 apparently optimizes repeated string concatenation to not be quadratic time,
        so the elements of `arr` can be strings rather than arrays of characters.
    */
    const arr: string[] = [];
    const encounteredChars = new Set();
    for (const char of str) {
        if (!encounteredChars.has(char)) {
            encounteredChars.add(char);
            arr.push('');
        }
        arr[arr.length - 1]! += char;
    }
    return arr;
}
