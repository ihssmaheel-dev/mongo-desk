/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,svelte,ts,js}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        brand: {
          forest: '#023430',
          evergreen: '#00684A',
          spring: '#00ED64',
          'spring-dim': '#00C75A',
          mist: '#E3FCF7',
        },
        slate: {
          50: '#F7FAFA',
          100: '#EEF3F4',
          200: '#C3D4DE',
          300: '#DCE6EA',
          400: '#7E97A7',
          600: '#465A6B',
          700: '#2D3A45',
          800: '#1F2933',
          900: '#161D24',
          950: '#0E1318',
        },
        bson: {
          string: '#00ED64',
          number: '#5DD0FF',
          objectid: '#FFC010',
          date: '#B79CFF',
          boolean: '#FF8966',
          null: '#7E97A7',
        },
        semantic: {
          success: '#00ED64',
          warning: '#FFC010',
          danger: '#FF5C5C',
          info: '#5DD0FF',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      fontSize: {
        xs: ['11px', '16px'],
        sm: ['12px', '18px'],
        base: ['13px', '20px'],
        md: ['14px', '22px'],
        lg: ['16px', '24px'],
        xl: ['20px', '28px'],
      },
      borderRadius: {
        sm: '4px',
        md: '6px',
        lg: '10px',
      },
      spacing: {
        1: '4px',
        2: '8px',
        3: '12px',
        4: '16px',
        6: '24px',
        8: '32px',
      },
    },
  },
};
