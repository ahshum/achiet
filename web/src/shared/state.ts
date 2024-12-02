import { atom } from "jotai"
import { atomWithStorage } from "jotai/utils"

export const appConfigAtom = atom<AppConfig>(() => {
  return {
    host: location.origin,
  }
})

export const accessTokenAtom = atomWithStorage<Nullable<string>>("accessToken", null, undefined, { getOnInit: true })
