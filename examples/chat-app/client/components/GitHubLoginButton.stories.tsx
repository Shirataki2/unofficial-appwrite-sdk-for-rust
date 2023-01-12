import React from 'react'
import { ComponentStoryObj, ComponentMeta } from '@storybook/react'

import { GitHubLoginButton } from './GitHubLoginButton'
import { Client, Account } from 'appwrite'

type Component = typeof GitHubLoginButton
type Meta = ComponentMeta<Component>
type Story = ComponentStoryObj<Component>

export default {
  title: 'UI/GitHubLoginButton',
  component: GitHubLoginButton,
} as Meta

export const Dummy: Story = {
  args: {
    authUrl: '#',
  },
}
