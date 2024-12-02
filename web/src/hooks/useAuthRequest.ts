import { useMutation } from "@tanstack/react-query";
import useHttpClient from "./useHttpClient";
import { useAtom } from "jotai";
import { accessTokenAtom } from "@/shared/state";

export type UseAuthRequestOptions = {
  isRegister: boolean,
}

export type AuthRequest = {
  username: string,
  password: string,
}

export default function useAuthRequest(options?: UseAuthRequestOptions) {
  const isRegister = options?.isRegister || false
  const [, setAccessToken] = useAtom(accessTokenAtom)
  const httpClient = useHttpClient()
  const mutation = useMutation({
    mutationFn: async (data: AuthRequest) => {
      if (isRegister) {
        await httpClient("/api/register", {
          method: "POST",
          body: data,
        })
      }
      return await httpClient("/api/auth", {
        method: "POST",
        body: data,
      })
    },
    onSuccess: (data) => {
      setAccessToken(data.accessToken)
    },
  })
  return mutation
}
