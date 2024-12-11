import { useMutation, useQueryClient } from "@tanstack/react-query"
import useHttpClient from "./useHttpClient"

export default function useDeleteBookmark() {
  const httpClient = useHttpClient()
  const queryClient = useQueryClient()
  const mutation = useMutation({
    mutationFn: async (data: BookmarkModel) => {
      return await httpClient(`/api/bookmark/${data.id}`, {
        method: "DELETE",
      })
    },
    onSuccess: (newItem) => {
      queryClient.setQueryData(
        ["bookmark"],
        (oldItems: BookmarkModel[]) => {
          const idx = oldItems.findIndex(item => item.id === newItem.id)
          return idx > -1
            ? [ ...oldItems.slice(0, idx), ...oldItems.slice(idx + 1) ]
            : oldItems
        },
      )
      queryClient.invalidateQueries({ queryKey: ["tag"] })
    },
  })
  return mutation
}
