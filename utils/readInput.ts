import { readFileSync } from 'fs';
import { resolve } from 'path';

export const readInput = (baseDir: string) => {
  return readFileSync(resolve(baseDir, 'input.txt'), {
    encoding: 'utf-8',
  }).trimEnd();
};
