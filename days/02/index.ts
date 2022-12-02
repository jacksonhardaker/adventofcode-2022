import { readInput } from 'utils/readInput';

const rawInput = readInput(import.meta.dir);

enum Scores {
  rock = 1,
  paper = 2,
  scissors = 3,
  win = 6,
  draw = 3,
  lose = 0,
}

enum ElfMove {
  A = 'rock',
  B = 'paper',
  C = 'scissors',
}
enum YourMove {
  X = 'rock',
  Y = 'paper',
  Z = 'scissors',
}

const result = (a: ElfMove, b: YourMove): 'win' | 'lose' | 'draw' => {
  if (ElfMove[a] === YourMove[b]) return 'draw';

  if (
    (ElfMove[a] === 'rock' && YourMove[b] === 'paper') ||
    (ElfMove[a] === 'paper' && YourMove[b] === 'scissors') ||
    (ElfMove[a] === 'scissors' && YourMove[b] === 'rock')
  )
    return 'win';

  return 'lose';
};

export const part1 = (input: string) => {
  return input
    .trimEnd()
    .split('\n')
    .map((line) => {
      const [a, b] = line.split(' ') as [ElfMove, YourMove];
      const outcome = result(a, b);
      return Scores[outcome] + Scores[YourMove[b]];
    })
    .reduce((sum, round) => sum + round);
};

enum Tactic {
  X = 'lose',
  Y = 'draw',
  Z = 'win',
}

enum WinningMove {
  paper = 'scissors',
  scissors = 'rock',
  rock = 'paper',
}

enum LosingMove {
  paper = 'rock',
  scissors = 'paper',
  rock = 'scissors',
}

export const part2 = (input: string) => {
  return input
    .trimEnd()
    .split('\n')
    .map((line) => {
      const [a, outcome] = line.split(' ') as [ElfMove, Tactic];
      const yourMove =
        Tactic[outcome] === 'win'
          ? WinningMove[ElfMove[a]]
          : Tactic[outcome] === 'lose'
          ? LosingMove[ElfMove[a]]
          : ElfMove[a];
      return Scores[Tactic[outcome]] + Scores[yourMove];
    })
    .reduce((sum, round) => sum + round);
};

// console.log(`Part 1: ${part1(rawInput)}`);
// console.log(`Part 2: ${part2(rawInput)}`);
