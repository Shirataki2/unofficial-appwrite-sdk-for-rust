import { createTheme } from '@mui/material'
import { pink, lightBlue, grey } from '@mui/material/colors'

export const lightTheme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: lightBlue['A400'],
    },
    secondary: {
      main: pink['400'],
    },
    background: {
      default: grey['100'],
      paper: grey['50'],
    },
  },
})
