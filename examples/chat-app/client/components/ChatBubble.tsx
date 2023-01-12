import { Typography } from '@mui/material'
import { styled } from '@mui/system'
export interface ChatBubbleProps {
  message: string
  isMine: boolean
  maxWidth?: string
  username?: string
}

const StyledChatBubble = styled('li')<ChatBubbleProps>(({ theme, username, isMine, maxWidth }) => {
  const isDark = theme.palette.mode === 'dark'
  const him = {
    backgroundColor: '#ddd',
    color: isDark ? '#000' : null,
    float: 'left',
  }
  const mine = {
    backgroundColor: isDark ? '#3f5ddf' : '#105fff',
    color: '#fff',
    float: 'right',
  }
  const himSiblingMine = {
    borderBottomRightRadius: '5px',
  }
  const mineSiblingMine = {
    borderTopRightRadius: '5px',
    borderBottomRightRadius: '5px',
  }
  const mineLastOfType = {
    borderBottomRightRadius: '20px',
  }
  const usernameStyle = username
    ? {
        '&::before': {
          content: `"${username}"`,
          color: isMine ? '#aaa' : '#666',
          display: 'block',
          fontSize: '0.8em',
          marginBottom: '-2px',
          float: isMine ? 'right' : 'left',
        },
      }
    : {}
  return {
    display: 'inline-block',
    clear: 'both',
    padding: '6px 10px',
    borderRadius: '10px',
    marginBottom: '2px',
    maxWidth: maxWidth || '80%',
    ...usernameStyle,
    '&.him': him,
    '&.mine': mine,
    '&.him + .mine': himSiblingMine,
    '&.mine + .mine': mineSiblingMine,
    '&.mine:last-of-type': mineLastOfType,
  }
})

export const ChatBubble = (props: ChatBubbleProps) => {
  const { message, isMine } = props
  return (
    <StyledChatBubble {...props} className={`chat-bubble ${isMine ? 'mine' : 'him'}`}>
      <Typography variant='body1'>{message}</Typography>
    </StyledChatBubble>
  )
}
