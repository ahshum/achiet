import { accessTokenAtom, appConfigAtom } from "@/shared/state"
import { useAtom } from "jotai"

export type UseHttpClientOptions = {
  withJson?: boolean,
}

export type HttpClientRequestInit = Omit<RequestInit, "headers" | "body"> & {
  headers?: Record<string, string>,
  body?: { [key: string]: any },
}

export default function useHttpClient(options: UseHttpClientOptions = {}) {
  const { withJson = true } = options
  const [appConfig] = useAtom(appConfigAtom)
  const [accessToken] = useAtom(accessTokenAtom)

  return async (url: string, initOptions?: HttpClientRequestInit) => {
    const { body, headers = {}, ...rest } = initOptions || {}
    const reqInit: RequestInit = { ...rest }

    if (body && withJson) {
      reqInit.body = JSON.stringify(body)
      headers["content-type"] = "application/json"
    }
    if (accessToken) {
      headers["authorization"] = "Bearer " + accessToken
    }
    if (withJson) {
      headers["accept"] = "application/json"
    }
    reqInit.headers = headers

    const res = await fetch(appConfig.host + url, reqInit)
    if (!res.ok) {
      const msg = await res.json()
      throw new Error(msg.error)
    }
    if (withJson) {
      return await res.json()
    }
    return res
  }
}
