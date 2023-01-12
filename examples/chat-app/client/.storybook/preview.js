import { useMemo } from 'react'

import '@fontsource/roboto/300.css'
import '@fontsource/roboto/400.css'
import '@fontsource/roboto/500.css'
import '@fontsource/roboto/700.css'
import '@fontsource/material-icons'

import { CssBaseline, ThemeProvider } from '@mui/material'
import { darkTheme } from '../src/theme/dark.theme'
import { lightTheme } from '../src/theme/light.theme'

const themes = {
  light: lightTheme,
  dark: darkTheme,
}

export const globalTypes = {
  theme: {
    name: 'Theme',
    title: 'Theme',
    description: 'Theme for your components',
    defaultValue: 'light',
    toolbar: {
      icon: 'paintbrush',
      dynamicTitle: true,
      items: [
        { value: 'light', left: '☀️', title: 'Light mode' },
        { value: 'dark', left: '🌙', title: 'Dark mode' },
      ],
    },
  },
}

export const withMuiTheme = (Story, context) => {
  const { theme: key } = context.globals

  const theme = useMemo(() => themes[key] || themes.light, [key])

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Story />
    </ThemeProvider>
  )
}

export const decorators = [withMuiTheme]

export const parameters = {
  actions: { argTypesRegex: '^on[A-Z].*' },
  controls: {
    expanded: true,
    matchers: {
      color: /(background|color)$/i,
      date: /Date$/,
    },
  },
}
