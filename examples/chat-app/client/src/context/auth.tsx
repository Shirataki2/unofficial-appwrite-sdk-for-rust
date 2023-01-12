import { Account, Models, RealtimeResponseEvent } from 'appwrite'
import { useRouter } from 'next/router'
import React from 'react'
import { useAppwrite } from '../../hooks/appwrite'

export type CurrentUser = Models.Account<Models.Preferences>

export interface AuthContextProps {
  user: CurrentUser | null
}

export interface AuthProps {
  children: React.ReactNode
}

const AuthContext = React.createContext<Partial<AuthContextProps>>({})

export const useAuthContext = () => React.useContext(AuthContext)

export const AuthProvider = ({ children }: AuthProps) => {
  const router = useRouter()
  const [currentUser, setCurrentUser] = React.useState<CurrentUser | null>(null)
  const { client } = useAppwrite()
  const account = new Account(client)
  const props = {
    user: currentUser,
  }

  React.useEffect(() => {
    let user: CurrentUser | null = null
    account
      .get()
      .then((response) => {
        user = response
        setCurrentUser(user)
      })
      .catch(() => {
        user = null
        setCurrentUser(user)
      })
    const authListener = client.subscribe('account', async (resp: RealtimeResponseEvent<CurrentUser>) => {
      setCurrentUser(resp.payload)
    })
    return () => {
      authListener()
    }
  }, [])

  return <AuthContext.Provider value={props}>{children}</AuthContext.Provider>
}
