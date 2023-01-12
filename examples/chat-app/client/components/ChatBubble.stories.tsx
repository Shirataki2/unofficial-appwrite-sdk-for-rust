import React from 'react'
import { ComponentStoryObj, ComponentMeta } from '@storybook/react'

import { ChatBubble } from './ChatBubble'

type Component = typeof ChatBubble
type Meta = ComponentMeta<Component>
type Story = ComponentStoryObj<Component>

export default {
  title: 'UI/ChatBubble',
  component: ChatBubble,
} as Meta

export const Mine: Story = {
  args: {
    message: 'Hello',
    isMine: true,
  },
}

export const Him: Story = {
  args: {
    message: 'Hello',
    isMine: false,
  },
}

export const HimWithName: Story = {
  args: {
    message: 'Hello',
    isMine: false,
    username: 'John',
  },
}
