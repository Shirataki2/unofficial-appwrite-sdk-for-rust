import React from 'react'
import { Box, ButtonBase } from '@mui/material'
import { styled } from '@mui/system'

export interface GitHubLoginButtonProps {
  label: string
  textColor: string
  backgroundColor: string
  href?: string
  onClick?: () => void
}

const RoundedButtonStyle = styled(ButtonBase)<GitHubLoginButtonProps>(({ textColor, backgroundColor }) => ({
  justifyContent: 'center',
  backgroundColor: backgroundColor,
  color: textColor,
  padding: '8px 6px',
  borderRadius: '10px',
  display: 'inline-flex',
  fontSize: '14px',
  fontWeight: 600,
  margin: '0 4px',
  height: '40px',
}))

export const RoundedButton = (props: GitHubLoginButtonProps) => {
  return (
    // @ts-ignore
    <RoundedButtonStyle {...props}>
      <Box mx={2}>{props.label}</Box>
    </RoundedButtonStyle>
  )
}
