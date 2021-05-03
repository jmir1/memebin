/*
//  Copyright 2021 Catgirl.Moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

module.exports = {
  purge: [
    "./templates/**/*.html"
  ],
  theme: {
    extend: {
      spacing: {
        "100": "25rem",
        "320": "80rem",
        "23.5": "5.875rem",
        "42": "11.5rem",
        "274.5": "68.625rem",
        "90": "22.5rem",
        "50.5": "12.625rem"
      },
      maxWidth: {
        "100": "25rem",
        "320": "80rem",
        "23.5": "5.875rem",
        "42": "11.5rem",
        "274.5": "68.625rem"
      },
      minWidth: {
        "100": "25rem",
        "320": "80rem",
        "23.5": "5.875rem",
        "42": "11.5rem",
        "274.5": "68.625rem"
      },
      maxHeight: {
        "100": "25rem",
        "320": "80rem",
        "23.5": "5.875rem",
        "42": "11.5rem",
        "274.5": "68.625rem",
        "77vh": "77vh"
      },
      minHeight: {
        "100": "25rem",
        "320": "80rem",
        "23.5": "5.875rem",
        "42": "11.5rem",
        "274.5": "68.625rem"
      },
      colors: {
        'dark': {
          '50': '#616161',
          '100': '#5C5C5C',
          '200': '#525252',
          '300': '#474747',
          '400': '#3D3D3D',
          '500': '#333333',
          '600': '#292929',
          '700': '#1F1F1F',
          '800': '#141414',
          '900': '#0A0A0A'
        },
        'light': {
          DEFAULT: '#FAFAFA',
          '50': '#FAFAFA',
          '100': '#F5F5F5',
          '200': '#EBEBEB',
          '300': '#E0E0E0',
          '400': '#D6D6D6',
          '500': '#CCCCCC',
          '600': '#C2C2C2',
          '700': '#B8B8B8',
          '800': '#ADADAD',
          '900': '#A3A3A3'
        },
        'accent': {
          DEFAULT: '#E91E63',
          '50': '#F1729D',
          '100': '#F06897',
          '200': '#EE568A',
          '300': '#ED437D',
          '400': '#EB3170',
          '500': '#E91E63',
          '600': '#DD1659',
          '700': '#CA1452',
          '800': '#B8124A',
          '900': '#A51043'
        },
      },
      boxShadow: {
        navbar: "0 1px 2px rgba(0,0,0,0.9),0 0px 2px rgba(0,0,0,0.9)"
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}