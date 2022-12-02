import { expect, test, describe } from 'bun:test';
import * as dayModule from './index';

const input = `A Y
B X
C Z
`;

describe('day 2', () => {
  test('part 1', () => {
    expect(dayModule.part1(input)).toBe(15);
  });

  test('part 2', () => {
    expect(dayModule.part2(input)).toBe(12);
  })
});
