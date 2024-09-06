import { join } from 'path';
import type { Config } from 'tailwindcss';
import { skeleton } from '@skeletonlabs/tw-plugin';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import {stargazerTheme} from './stargazer-theme';

const config: Config = {
  darkMode: 'class',
  content: [
    './src/**/*.{html,js,svelte,ts}',
    join(require.resolve('@skeletonlabs/skeleton'), '../**/*.{html,js,svelte,ts}')
  ],
  theme: {
    extend: {
      fontFamily: {
        aleo: ['Aleo', 'sans-serif'],
      },
      maxWidth: {
				'prose': '120ch',
				'prose-sm': '112ch'
			},
    },
  },
  plugins: [
    skeleton({
    themes: {
    custom: [
      stargazerTheme,
    ]},
  }),
    typography,
    forms,
  ],
};

export default config;
