/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./templates/*.{html,js}"
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
  variants: {
    extend: {
      display: ['responsive', 'group-hover', 'group-focus'],
    },
  },
  daisyui: {
    themes:[
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
      ],
  }
}

