import { useMutation, useQueryClient } from "@tanstack/react-query"
import useHttpClient from "./useHttpClient"

export default function useSaveBookmark() {
  const httpClient = useHttpClient()
  const queryClient = useQueryClient()
  const mutation = useMutation({
    mutationFn: async (data: BookmarkModel) => {
      const isNew = !data.id
      const baseUrl = "/api/bookmark"
      if (isNew) {
        return await httpClient(baseUrl, {
          method: "POST",
          body: data,
        })
      } else {
        return await httpClient(baseUrl + `/${data.id}`, {
          method: "PUT",
          body: data,
        })
      }
    },
    onSuccess: (newItem) => {
      queryClient.setQueryData(
        ["bookmark", { id: newItem.id }],
        newItem,
      )
      queryClient.setQueryData(
        ["bookmark"],
        (oldItems: BookmarkModel[]) => {
          const idx = oldItems.findIndex(item => item.id === newItem.id)
          return idx > -1
            ? [ ...oldItems.slice(0, idx), newItem, ...oldItems.slice(idx + 1) ]
            : [ ...oldItems, newItem ]
        },
      )
      queryClient.invalidateQueries({ queryKey: ["tag"] })
    },
  })
  return mutation
}
