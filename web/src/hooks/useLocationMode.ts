import { useCallback, useMemo } from "react"
import { Path } from "react-router-dom"
import useLocationSearchParam from "./useLocationSearchParam"

export enum Mode {
  View,
  Edit,
}

export type ModeTypeStr = keyof typeof Mode

export default function useLocationMode(): [Mode, (mode?: Mode) => Path] {
  const [value, createLoc] = useLocationSearchParam("mode")

  const mode = useMemo((): Mode => Mode[value as ModeTypeStr] || Mode.View, [value])
  const createLocWithMode = useCallback((mode?: Mode): Path => {
    return mode !== undefined
      ? createLoc(Mode[mode])
      : createLoc()
  }, [createLoc])

  return [ mode, createLocWithMode ]
}
