import React from 'react'
import { Input, Paper, Box, FormControl, InputLabel, InputAdornment, IconButton } from '@mui/material'
import SendIcon from '@mui/icons-material/Send'

export interface TextFieldProps {
  label: string
  value: string
  cardType?: 'rounded' | 'square'
  elevation?: number
  handleChange?: (event: React.ChangeEvent<HTMLInputElement>) => void
  handleSubmit?: () => void | Promise<void>
}

export const ChatTextField = ({
  label,
  value = '',
  cardType = 'rounded',
  elevation = 1,
  handleChange = () => {},
  handleSubmit = () => {},
}: TextFieldProps) => {
  const isSquare = cardType === 'square'
  const [loading, setLoading] = React.useState(false)
  const isSendable = value.length > 0 && !loading
  const wrappedHandleSubmit = async () => {
    setLoading(true)
    await handleSubmit()
    setLoading(false)
  }
  return (
    <Paper elevation={elevation} square={isSquare}>
      <Box pb={2} px={2} component='form'>
        <FormControl variant='standard' fullWidth>
          <InputLabel htmlFor='standard-adornment-amount'>{label}</InputLabel>
          <Input
            id='standard-adornment-amount'
            disabled={loading}
            value={value}
            onChange={handleChange}
            endAdornment={
              <InputAdornment position='end' disablePointerEvents={!isSendable}>
                <IconButton disabled={!isSendable} aria-label='send message' edge='end' onClick={wrappedHandleSubmit}>
                  <SendIcon />
                </IconButton>
              </InputAdornment>
            }
          />
        </FormControl>
      </Box>
    </Paper>
  )
}
