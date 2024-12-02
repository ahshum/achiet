import { useCallback } from "react"
import { Path, useLocation } from "react-router-dom"

export default function useLocationPath(): [string, (path: string) => Path] {
  const { search, pathname, hash } = useLocation()

  const createLocation = useCallback((path: string): Path => {
    return { search, pathname: path, hash }
  }, [search, hash])

  return [
    pathname,
    createLocation,
  ]
}
