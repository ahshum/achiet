import { accessTokenAtom } from "@/shared/state"
import { useQuery } from "@tanstack/react-query"
import { useAtom } from "jotai"
import { Link, Outlet } from "react-router-dom"
import useHttpClient from "@/hooks/useHttpClient"

export default function Landing() {
  const [, setAccessToken] = useAtom(accessTokenAtom)
  const httpClient = useHttpClient()

  const tagsQuery = useQuery<TagModel[]>({
    queryKey: ["tag"], 
    queryFn: async () => {
      return await httpClient("/api/tag")
    },
  })

  const bookmarksQuery = useQuery<BookmarkModel[]>({
    queryKey: ["bookmark"], 
    queryFn: async () => {
      return await httpClient("/api/bookmark")
    },
  })

  return (
    <>
      <div className="">
        <div className="w-[240px] bg-[#2f2f2f] min-h-screen h-full">
          <div>
            <div onClick={() => setAccessToken(null)}>
              Account
            </div>
          </div>
          <div>Tags</div>
          {tagsQuery.isSuccess && tagsQuery.data.map(tag => (
            <div key={tag.id}>
              {tag.path}
            </div>
          ))}
        </div>
      </div>
      <div className="flex-1">
        <div className="">
          top
        </div>
        <div className="flex">
          <div className="w-[300px] grid gap-4">
            {bookmarksQuery.isSuccess && bookmarksQuery.data.map(bm => (
              <div key={bm.id} className="flex gap-2 px-4 relative">
                <div className="flex-none border border-gray-400 border-solid rounded size-12"></div>
                <div className="flex flex-col gap-1">
                  <div className="min-w-0">
                    {bm.url ? (
                      <a href={bm.url} target="_blank">{bm.title}</a>
                    ) : (
                      <span>{bm.title}</span>
                    )}
                  </div>
                  <div className="flex flex-row gap-1 text-sm">
                    {bm.tags.map(tag => (
                      <span key={tag} className="rounded-lg border border-gray-500 bg-gray-700 px-2">
                        {tag.split(":")[0]}
                      </span>
                    ))}
                  </div>
                </div>
                <Link to={`/bookmark/${bm.id}`} className="absolute inset-0" />
              </div>
            ))}
          </div>
          <div className="">
            <Outlet />
          </div>
        </div>
      </div>
    </>
  )
}
