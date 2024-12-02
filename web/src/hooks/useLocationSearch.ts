import { useCallback, useMemo } from "react"
import { Path, useLocation } from "react-router-dom"

export default function useLocationSearch(): [URLSearchParams, (search: URLSearchParams) => Path] {
  const { search, pathname, hash } = useLocation()

  const searchParams = useMemo((): URLSearchParams => {
    return new URLSearchParams(search)
  }, [search])

  const createLocation = useCallback((search: URLSearchParams): Path => {
    return { search: search.toString(), pathname, hash }
  }, [pathname, hash])

  return [
    searchParams,
    createLocation,
  ]
}
