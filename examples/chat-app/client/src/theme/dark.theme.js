import { createTheme } from '@mui/material'
import { cyan, amber, blueGrey } from '@mui/material/colors'

export const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: amber['A400'],
    },
    secondary: {
      main: cyan['A400'],
    },
    background: {
      default: blueGrey['800'],
      paper: blueGrey['900'],
    },
  },
})
