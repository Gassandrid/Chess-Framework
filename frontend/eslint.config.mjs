import { dirname } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const compat = new FlatCompat({
  baseDirectory: __dirname,
});

const eslintConfig = [
  ...compat.extends("next/core-web-vitals", "next/typescript"),
  {
    rules: {
      // Disable warnings for unused variables and unused code
      "@typescript-eslint/no-unused-vars": "off",
      "no-unused-vars": "off",
      // You can add more rules to disable if needed
    },
  },
];

export default eslintConfig;
