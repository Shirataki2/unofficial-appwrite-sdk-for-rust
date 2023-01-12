import React from 'react'
import { ComponentStoryObj, ComponentMeta } from '@storybook/react'

import { RoundedButton } from './RoundedButton'
import { Client, Account } from 'appwrite'

type Component = typeof RoundedButton
type Meta = ComponentMeta<Component>
type Story = ComponentStoryObj<Component>

export default {
  title: 'UI/RoundedButton',
  component: RoundedButton,
} as Meta

export const Sample: Story = {
  args: {
    label: 'Sample',
    textColor: 'white',
    backgroundColor: 'black',
  },
}
