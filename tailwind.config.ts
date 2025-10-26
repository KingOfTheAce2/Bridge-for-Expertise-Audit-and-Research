export default {
  darkMode: 'class',
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: {
          light: '#3b82f6',
          dark: '#60a5fa',
        },
        background: {
          light: '#ffffff',
          dark: '#111827',
        },
        surface: {
          light: '#f9fafb',
          dark: '#1f2937',
        },
      },
    },
  },
  plugins: [],
};
