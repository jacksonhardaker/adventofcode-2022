import { readInput } from 'utils/readInput';

const input = readInput(import.meta.dir);

export const part1 = (input: string) => {
  const elves = input
    .split(/\n\n/)
    .map((elf: string) => elf.split(/\n/).map(Number));

  return Math.max(...elves.map((elf) => elf.reduce((sum, num) => sum + num)));
};

export const part2 = (input: string) => {
  const elves = input
    .split(/\n\n/)
    .map((elf: string) => elf.split(/\n/).map(Number));

  return elves
    .map((elf) => elf.reduce((sum, num) => sum + num))
    .sort((a, b) => a > b ? -1 : 1)
    .slice(0, 3)
    .reduce((sum, num) => sum + num);
};

console.log(`Part 1: ${part1(input)}`);
console.log(`Part 2: ${part2(input)}`);
