import { useMemo } from "react"
import { Path } from "react-router-dom"
import useLocationSearchParam from "./useLocationSearchParam"

export default function useLocationTag(): [string, (path?: string) => Path, boolean] {
  const [value, createLoc] = useLocationSearchParam("path")

  const tagPath = useMemo(() => value || "/", [value])
  const isRoot = useMemo((): boolean => tagPath === "/", [tagPath])

  return [ tagPath, createLoc, isRoot ]
}
