import React from 'react'
import { ComponentStoryObj, ComponentMeta } from '@storybook/react'

import { ChatTextField } from './ChatTextField'
import { Box } from '@mui/system'
import { Typography } from '@mui/material'

type Component = typeof ChatTextField
type Meta = ComponentMeta<Component>
type Story = ComponentStoryObj<Component>

export default {
  title: 'UI/ChatTextField',
  component: ChatTextField,
} as Meta

export const Simple: Story = {
  args: {
    label: 'Write a message',
  },
}

export const Flat: Story = {
  args: {
    label: 'Write a message',
    cardType: 'square',
    elevation: 0,
  },
}

export const HandleInput: Story = {
  args: {
    label: 'Write a message',
    cardType: 'square',
    elevation: 0,
  },
  render: (args) => {
    const [value, setValue] = React.useState('')
    const [send, setSend] = React.useState(false)
    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => setValue(event.target.value)
    const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms))
    const handleSubmit = async () => {
      await wait(1000)
      setSend(true)
      setValue('')
    }
    return (
      <>
        <Box mb={2}>
          <Typography variant='body1'>Value: {value}</Typography>
          <Typography variant='body1'>Is Sended: {send ? 'True' : 'False'}</Typography>
        </Box>
        <ChatTextField {...args} value={value} handleChange={handleChange} handleSubmit={handleSubmit} />
      </>
    )
  },
}
