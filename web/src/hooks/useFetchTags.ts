import { useQuery, UseQueryResult } from "@tanstack/react-query"
import useHttpClient from "./useHttpClient"

export default function useFetchTags(): UseQueryResult<TagModel[]> {
  const httpClient = useHttpClient()
  const query = useQuery<TagModel[]>({
    queryKey: ["tag"],
    queryFn: async () => {
      return await httpClient("/api/tag")
    },
  })
  return query
}
