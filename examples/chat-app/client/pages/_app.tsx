import '../styles/globals.css'
import type { AppProps } from 'next/app'
import { CacheProvider, EmotionCache, ThemeProvider } from '@emotion/react'
import createEmotionCache from '../src/createEmotionCache'
import { CssBaseline } from '@mui/material'
import { lightTheme } from '../src/theme/light.theme'
import { darkTheme } from '../src/theme/dark.theme'
import React from 'react'
import { AuthProvider } from '../src/context/auth'

interface MyAppProps extends AppProps {
  emotionCache?: EmotionCache
}

const clientEmotionCache = createEmotionCache()

export default function App({ Component, emotionCache = clientEmotionCache, pageProps }: MyAppProps) {
  const [activeTheme, setActiveTheme] = React.useState(lightTheme)
  const [selectedTheme, setSelectedTheme] = React.useState<'light' | 'dark'>('light')
  const getActiveTheme = (themeMode: 'light' | 'dark') => {
    if (themeMode === 'light') {
      return lightTheme
    }
    return darkTheme
  }
  const toggleTheme: React.MouseEventHandler<HTMLButtonElement> = () => {
    const nextTheme = selectedTheme === 'light' ? 'dark' : 'light'
    setSelectedTheme(nextTheme)
    setActiveTheme(getActiveTheme(nextTheme))
  }
  React.useEffect(() => {
    setActiveTheme(getActiveTheme(selectedTheme))
  }, [selectedTheme])

  return (
    <CacheProvider value={emotionCache}>
      <ThemeProvider theme={activeTheme}>
        <AuthProvider>
          <CssBaseline />
          <Component {...pageProps} toggleTheme={toggleTheme} />
        </AuthProvider>
      </ThemeProvider>
    </CacheProvider>
  )
}
