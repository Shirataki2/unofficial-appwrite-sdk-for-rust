import React from 'react'
import { Box, ButtonBase } from '@mui/material'
import GitHubIcon from '@mui/icons-material/GitHub'
import { styled } from '@mui/system'

export interface GitHubLoginButtonProps {
  authUrl?: string
  authCallback?: () => void
}

const GitHubButton = styled(ButtonBase)({
  justifyContent: 'center',
  backgroundColor: '#24292e',
  color: '#fff',
  padding: '8px 16px',
  borderRadius: '10px',
  display: 'inline-flex',
  fontSize: '14px',
  fontWeight: 600,
  margin: '0 8px',
})

export const GitHubLoginButton = ({ authUrl, authCallback }: GitHubLoginButtonProps) => {
  return (
    // @ts-ignore
    <GitHubButton href={authUrl} onClick={authCallback}>
      <GitHubIcon />
      <Box mx={2}>Login with GitHub</Box>
    </GitHubButton>
  )
}
