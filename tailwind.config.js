/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {},
        colors: {
            'coffee-black': "#3D3828",
            'coffee-darkest': "#706233",
            'coffee-dark': "#B0926A",
            'coffee-light': "#E1C78F",
            'coffee-lightest': "#FAE7C9",
            'coffee-white': "#FAF7F2",
        },
        plugins: [],
    }
}