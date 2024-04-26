/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,rs}", // Пути к Rust шаблонам, если они генерируют HTML
    "./public/**/*.html"
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

