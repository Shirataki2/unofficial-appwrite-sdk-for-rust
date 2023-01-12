import React from 'react'
import { ComponentStoryObj, ComponentMeta } from '@storybook/react'

import { ChatBubbleList } from './ChatBubbleList'

type Component = typeof ChatBubbleList
type Meta = ComponentMeta<Component>
type Story = ComponentStoryObj<Component>

export default {
  title: 'UI/ChatBubbleList',
  component: ChatBubbleList,
} as Meta

export const SimpleConversation: Story = {
  args: {
    messages: [
      { message: 'Hello', isMine: true },
      { message: 'Hello', isMine: false },
    ],
  },
}

export const LongConversation: Story = {
  args: {
    messages: [
      { message: 'Hello', isMine: true },
      { message: 'Nice to meet you', isMine: false },
      { message: 'Nice to meet you too', isMine: true },
      { message: 'Today is a good day', isMine: true },
      { message: 'How are you?', isMine: true },
      { message: 'I am fine', isMine: false },
      { message: 'By the way, Storybook is awesome', isMine: false },
      { message: 'I agree', isMine: true },
      { message: 'I am glad to hear that', isMine: false },
      { message: 'Me too', isMine: true },
      { message: 'I will tell my friends about it', isMine: true },
    ],
  },
}

export const SimpleConversationWithName: Story = {
  args: {
    messages: [
      { message: 'Hello', isMine: true, username: 'Me' },
      { message: 'Hello', isMine: false, username: 'John' },
    ],
  },
}

export const LongSentence: Story = {
  args: {
    messages: [
      {
        message:
          'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec auctor, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eu nunc. Donec auctor, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eu nunc.',
        isMine: true,
      },
      {
        message:
          'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec auctor, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eu nunc. Donec auctor, nisl eget ultricies tincidunt, nunc nisl aliquam nisl, eget aliquam nunc nisl eu nunc.',
        isMine: false,
      },
    ],
  },
}
