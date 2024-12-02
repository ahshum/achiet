import { useQuery, UseQueryOptions, UseQueryResult } from "@tanstack/react-query"
import useHttpClient from "./useHttpClient"

export type UseFetchBookmarkByIdOptions = Partial<UseQueryOptions<BookmarkModel>>

export default function useFetchBookmarkById(
  id: string, 
  options?: UseFetchBookmarkByIdOptions,
): UseQueryResult<BookmarkModel> {
  const httpClient = useHttpClient()
  const query = useQuery<BookmarkModel>({
    ...options,
    queryKey: ["bookmark", { id }],
    queryFn: async () => {
      return await httpClient(`/api/bookmark/${id}`)
    },
  })
  return query
}
