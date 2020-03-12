import leven from './leven';

// Arbitrary threshold all suggestions must hit
// A lower number means higher quality suggestions but
// will be less likley to find a suggestion
const threshold = 2;

export function suggestClosest(
  text: string,
  suggestions: Array<string>,
): string | undefined {
  let current;
  let currentScore = threshold;

  for (const suggestion of suggestions) {
    const score = leven(text, suggestion);

    if (score <= currentScore) {
      currentScore = currentScore;
      current = suggestion;
    }
  }

  return current;
}
