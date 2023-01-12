import { Button, Container } from '@mui/material'
import { useRouter } from 'next/router'
import { GitHubLoginButton } from '../components/GitHubLoginButton'
import { useAppwrite } from '../hooks/appwrite'
import { useAuthContext } from '../src/context/auth'

export default function Home() {
  const { account, web_url } = useAppwrite()
  const { user } = useAuthContext()
  const router = useRouter()
  const isLoggedIn = user !== null
  const auth = () => {
    account.createOAuth2Session('github', web_url)
  }
  const logout = () => {
    account.deleteSession('current')
    router.reload()
  }
  return (
    <main>
      <Container maxWidth='md'>
        <p>Hello World!</p>
        {isLoggedIn && <p>You are logged in as {user?.name}</p>}
        <GitHubLoginButton authCallback={auth} />
        <Button variant='contained' onClick={logout}>
          Logout
        </Button>
      </Container>
    </main>
  )
}
