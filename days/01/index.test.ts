import { expect, test, describe } from 'bun:test';
import * as dayModule from './index';


const input = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`;

describe('day 1', () => {
  test('part 1', () => {
    expect(dayModule.part1(input)).toBe(24000);
  });

  test('part 2', () => {
    expect(dayModule.part2(input)).toBe(45000);
  });
});
