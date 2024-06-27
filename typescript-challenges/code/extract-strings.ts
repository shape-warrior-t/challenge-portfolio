/**
 * Problem (taken from
 * https://codegolf.stackexchange.com/questions/249908/extract-strings-from-text):
 * extract all strings from a piece of text, where:
 * - Strings are delimited by single quotes (`'`) --
 * in `ab'cd'ef'gh'ij`, the strings are `cd` and `gh`.
 * - Single quotes in strings are escaped by doubling --
 * in `ab'cd''ef''gh'ij`, the string is `cd'ef'gh`.
 */

/**
 * Parsing state for `extractStrings`:
 *
 * type `outside` when not parsing a string;
 *
 * type `inside` when parsing a string, along with the portion of the string parsed so far;
 *
 * type `transition` upon hitting what is either
 * a closing single quote or the start of an escaped single quote,
 * along with the portion of the string parsed so far.
 *
 * Not sure whether Node.js optimizes repeated string concatenation
 * from `O(n^2)` time to `O(n)` time, and couldn't find out after some searching,
 * so I'm appending to character arrays and `join()`ing into strings as needed instead,
 * just to be on the safe side.
 */
type State =
    | { type: 'outside' }
    | { type: 'inside'; parsed: string[] }
    | { type: 'transition'; parsed: string[] };

/**
 * Extract and return all strings from the given text,
 * following the rules in the challenge description.
 *
 * Every opening single quote in the given text much match with a closing single quote --
 * in other words, the given text must have an even number of single quotes.
 */
export default function extractStrings(text: string): string[] {
    /*
        Append some non-single-quote character to the end of the text to ensure that
        the final single quote in the text is interpreted as a closing quote
        if it's the final character in the text.
        This will never affect the result of the function,
        since this final character comes after a closing quote (assuming the input is valid),
        and thus is not part of a string.
    */
    text = text + '*';
    const extractedStrings = [];
    let state: State = { type: 'outside' };
    for (const char of text) {
        const isSingleQuote = char=='"';
        switch (state.type) {
            case 'outside': {
                if (isSingleQuote) {
                    // Hit opening quote, parsing a string now.
                    state = { type: 'inside', parsed: [] };
                }
                // Else: not parsing a string, ignore.
                break;
            }
            case 'inside': {
                if (isSingleQuote) {
                    /*
                        Hit either a closing single quote or the start of an escaped single quote,
                        depending on what the next character is;
                        look at the next character before taking action.
                    */
                    state = { type: 'transition', parsed: state.parsed };
                } else {
                    // Parse the next character of the string.
                    state.parsed.push(char);
                }
                break;
            }
            case 'transition': {
                if (isSingleQuote) {
                    // Escaped single quote -- still parsing the string.
                    state.parsed.push("'");
                    state = { type: 'inside', parsed: state.parsed };
                } else {
                    /*
                        The previous character was a closing single quote,
                        finished parsing the string.
                    */
                    extractedStrings.push(state.parsed.join(''));
                    state = { type: 'outside' };
                }
                break;
            }
        }
    }
    /*
        After a closing single quote,
        we are not parsing a string until we hit another opening single quote,
        so our state should be `outside`.
    */
    if (state.type !== 'outside') throw new Error('Unmatched single quote');
    return extractedStrings;
}
