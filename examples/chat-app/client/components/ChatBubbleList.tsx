import { styled } from '@mui/system'
import { ChatBubble, ChatBubbleProps } from './ChatBubble'

interface ChatBubbleListProps {
  messages: ChatBubbleProps[]
  maxWidth?: string
}

const StyledChatBubbleList = styled('ul')({
  listStyle: 'none',
  padding: 0,
  margin: 0,
})

export const ChatBubbleList = ({ messages, maxWidth }: ChatBubbleListProps) => {
  return (
    <StyledChatBubbleList>
      {messages.map((message, index) => (
        <ChatBubble {...message} key={index} maxWidth={maxWidth} />
      ))}
    </StyledChatBubbleList>
  )
}
