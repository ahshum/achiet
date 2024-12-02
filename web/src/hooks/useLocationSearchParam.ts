import { useCallback, useMemo } from "react"
import { Path } from "react-router-dom"
import useLocationSearch from "./useLocationSearch"

export default function useLocationSearchParam(key: string): [Nullable<string>, (value?: string) => Path] {
  const [search, createLoc] = useLocationSearch()

  const value = useMemo(() => search.get(key), [search])

  const createLocWithValue = useCallback((value?: string): Path => {
    const newSearch = new URLSearchParams(search)
    if (value) {
      newSearch.set(key, value)
    } else {
      newSearch.delete(key)
    }
    return createLoc(newSearch)
  }, [search, createLoc])

  return [
    value,
    createLocWithValue,
  ]
}
