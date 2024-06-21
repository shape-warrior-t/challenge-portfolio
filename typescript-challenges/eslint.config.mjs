// @ts-check

import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

export default tseslint.config(
    eslint.configs.recommended,
    ...tseslint.configs.strictTypeChecked,
    ...tseslint.configs.stylisticTypeChecked,
    {
        languageOptions: {
            parserOptions: {
                project: true,
                tsconfigRootDir: import.meta.dirname,
            },
        },
        rules: {
            eqeqeq: 'error',
            'no-constant-condition': 'off',
            'sort-imports': 'error',
            '@typescript-eslint/no-unnecessary-condition': [
                'error',
                { allowConstantLoopConditions: true },
            ],
        },
    },
);
