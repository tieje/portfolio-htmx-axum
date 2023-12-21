/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      fontFamily: {
        'mono': ['Menlo'],
      }
    },
  },
  plugins: ["prettier-plugin-tailwindcss"],
};
