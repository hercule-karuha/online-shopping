/* eslint-env node */
module.exports = {
    root: true,
    'extends': [
        'plugin:vue/vue3-recommended',
        'eslint:recommended',
    ],
    parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module'
    },
    plugins: [
        'vue'
    ],
    rules: {
        indent: ['error', 4]
    },
    env: {
        browser: true,
        es2021: true,
        node: true
    }
}
