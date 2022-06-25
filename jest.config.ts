import type { Config } from '@jest/types';

const config: Config.InitialOptions = {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  globals: {
    'ts-jest': {
      tsconfig: './tsconfig.json',
    },
  },
  moduleNameMapper: {
    '^.+\\.s?css$': '<rootDir>/src/__mocks__/styleMock.ts',
  },
};

export default config;
