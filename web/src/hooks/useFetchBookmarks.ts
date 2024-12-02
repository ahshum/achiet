import { useQuery, UseQueryResult } from "@tanstack/react-query"
import useHttpClient from "./useHttpClient"

export default function useFetchBookmarks(): UseQueryResult<BookmarkModel[]> {
  const httpClient = useHttpClient()
  const query = useQuery<BookmarkModel[]>({
    queryKey: ["bookmark"],
    queryFn: async () => {
      return await httpClient("/api/bookmark")
    },
  })
  return query
}
