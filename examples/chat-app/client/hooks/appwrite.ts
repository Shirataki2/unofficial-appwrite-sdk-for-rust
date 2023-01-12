import * as React from 'react'
import { Account, Client } from 'appwrite'

interface AppwriteHooks {
  client: Client
  account: Account
  web_url: string
}

export const useAppwrite: () => AppwriteHooks = () => {
  const project_id = process.env.NEXT_PUBLIC_APPWRITE_PROJECT_ID
  const endpoint = process.env.NEXT_PUBLIC_APPWRITE_ENDPOINT
  const web_url = process.env.NEXT_PUBLIC_WEB_URL
  if (!project_id || !endpoint || !web_url) {
    throw new Error('Missing environment variables')
  }
  const client = new Client().setEndpoint(endpoint).setProject(project_id)
  const account = new Account(client)
  return {
    client,
    account,
    web_url,
  }
}
