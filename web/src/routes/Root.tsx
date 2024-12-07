import { accessTokenAtom } from "@/shared/state"
import { useAtom } from "jotai"
import { useEffect, useMemo } from "react"
import { Link, Outlet, useMatch, useNavigate } from "react-router-dom"
import {
  BookmarkIcon,
  ChevronRightIcon,
  DocumentIcon,
  PencilSquareIcon,
  UserCircleIcon,
} from "@heroicons/react/16/solid"
import useFetchTags from "@/hooks/useFetchTags"
import useFetchBookmarks from "@/hooks/useFetchBookmarks"
import useLocationTag from "@/hooks/useLocationTag"
import useLocationPath from "@/hooks/useLocationPath"
import useLocationMode, { Mode } from "@/hooks/useLocationMode"
import CurrentPath from "@/components/nav/CurrentPath"
import { useHotkeys, useHotkeysContext } from "react-hotkeys-hook"

const HOTKEY_SCOPE_ROOT = "root"

export default function Root() {
  const [accessToken, setAccessToken] = useAtom(accessTokenAtom)
  const navigate = useNavigate()
  const [currentPath, createLocWithTag, isRoot] = useLocationTag()
  const { data: tags, isSuccess: tagsIsSuccess } = useFetchTags()
  const { data: bookmarks, isSuccess: bookmarksIsSuccess} = useFetchBookmarks()
  const [, createLocWithPath] = useLocationPath()
  const [, createLocWithMode] = useLocationMode()
  const bmMatch = useMatch("/bookmark/:bookmarkId")
  const { enableScope, disableScope } = useHotkeysContext()

  const currentTags = useMemo((): TagModel[] => {
    return tagsIsSuccess
      ? tags
        .filter(tag => tag.prefix === currentPath)
        .toSorted((a, b) => a.name.localeCompare(b.name))
      : []
  }, [tagsIsSuccess, tags, currentPath])

  const currentBookmarks = useMemo((): BookmarkModel[] => {
    if (!bookmarksIsSuccess) {
      return []
    }
    return isRoot
      ? bookmarks
      : bookmarks.filter(bm => bm.tags.some(tag => tag.split(":")[0] === currentPath))
  }, [bookmarksIsSuccess, bookmarks, isRoot, currentPath])

  const bmIndex = useMemo((): number => {
    return currentBookmarks.findIndex(bm => bm.id === bmMatch?.params.bookmarkId)
  }, [currentBookmarks, bmMatch])

  useHotkeys("j", () => {
    const nextIndex = (bmIndex + 1) % currentBookmarks.length
    const nextBm = currentBookmarks[nextIndex]
    navigate({
      ...createLocWithPath(`/bookmark/${nextBm.id}`),
      search: createLocWithMode(Mode.View).search,
    })
  }, {
    scopes: HOTKEY_SCOPE_ROOT,
    preventDefault: true,
  })

  useHotkeys("k", () => {
    const prevIndex = (bmIndex - 1 + currentBookmarks.length) % currentBookmarks.length
    const prevBm = currentBookmarks[prevIndex]
    navigate({
      ...createLocWithPath(`/bookmark/${prevBm.id}`),
      search: createLocWithMode(Mode.View).search,
    })
  }, {
    scopes: HOTKEY_SCOPE_ROOT,
    preventDefault: true,
  })

  useHotkeys("o", () => {
    if (bmIndex < 0 || !currentBookmarks[bmIndex].url) {
      return
    }
    Object.assign(document.createElement('a'), {
      target: '_blank',
      rel: 'noopener noreferrer',
      href: currentBookmarks[bmIndex].url,
    }).click()
  }, {
    scopes: HOTKEY_SCOPE_ROOT,
    preventDefault: true,
  })

  useHotkeys("n", () => {
    navigate(createLocWithPath("/bookmark/new"))
  }, {
    scopes: HOTKEY_SCOPE_ROOT,
    preventDefault: true,
  })

  useHotkeys("e", () => {
    navigate(createLocWithMode(Mode.Edit))
  }, {
    scopes: HOTKEY_SCOPE_ROOT,
    preventDefault: true,
  })

  useEffect(() => {
    if (!accessToken) {
      navigate("/login")
    }

    enableScope(HOTKEY_SCOPE_ROOT)
    return () => {
      disableScope(HOTKEY_SCOPE_ROOT)
    }
  }, [accessToken])

  return (
    <div className="flex flex-row items-stretch h-screen relative max-w-[var(--container-w)] mx-auto">
      <div className="fixed left-0 inset-x pl-[calc(50%-var(--container-w)/2)] bg-[#2f2f2f]">
        <div className="px-[var(--sidebar-p)] w-[calc(var(--sidebar-w)+var(--sidebar-p)*2)] min-h-screen h-full">
          <div className="py-3 flex items-center cursor-pointer" onClick={() => setAccessToken(null)}>
            <div className="grow">
              Account
            </div>
            <UserCircleIcon className="size-5" />
          </div>

          <div className="py-2 flex items-center">
            <div className="grow">
              Tags
            </div>
          </div>
          {currentTags
            .map(tag => (
              <Link
                key={tag.id}
                className="block"
                to={createLocWithTag(tag.path)}
              >
                {tag.name}
              </Link>
            ))
          }
        </div>
      </div>

      <div className="grow flex flex-col pl-[calc(var(--sidebar-w)+var(--sidebar-p)*2)]">
        <div className="shadow shadow-black">
          <div className="px-4 py-2 flex items-center">
            <CurrentPath />

            <div className="grow"></div>

            <div className="px-2 py-1 border-[#3f3f3f] border flex divide-x divide-[#383838] rounded-full mx-2">
              <Link className="px-2" to={createLocWithMode(Mode.View)}>
                <DocumentIcon className="size-5" />
              </Link>
              <Link className="px-2" to={createLocWithMode(Mode.Edit)}>
                <PencilSquareIcon className="size-5" />
              </Link>
            </div>

            <div className="px-3 py-1 border-[#3f3f3f] border-l">
              <Link className="" to={createLocWithPath("/bookmark/new")}>
                <BookmarkIcon className="size-5" />
              </Link>
            </div>

          </div>
        </div>

        <div className="min-h-0 grow flex flex-row">
          <div className="overflow-auto shrink-0">
            <div className="flex-1">
              <div className="flex relative">
                <div className="w-[400px] flex flex-col divide-y divide-[#383838]">
                  {currentBookmarks.map(bm => (
                    <div key={bm.id} className="flex gap-2 px-4 py-3 relative">
                      <div className="flex flex-col gap-1">
                        <div className="flex min-w-0">
                          {bm.url ? (
                            <a
                              href={bm.url}
                              target="_blank"
                              className="z-10 hover:underline flex items-center"
                            >
                              {bm.title || bm.url}
                            </a>
                          ) : (
                            <span>
                              {bm.title}
                            </span>
                          )}
                        </div>
                        {bm.tags.length > 0 && (
                          <div className="flex flex-row gap-1 text-sm overflow-x-scroll z-10 scrollbar-hidden">
                            {bm.tags.map(tag => {
                              const path = tag.split(":")[0]
                              if (!isRoot && path === currentPath) {
                                return null
                              }
                              return (
                                <div key={tag} className="rounded-full border border-white px-2 z-10">
                                  {path}
                                </div>
                              )
                            })}
                          </div>
                        )}
                      </div>
                      {bmMatch?.params.bookmarkId === bm.id && (
                        <div className="absolute right-0 inset-y-0 flex flex-row items-center">
                          <div className="pr-2">
                            <ChevronRightIcon className="size-5" />
                          </div>
                        </div>
                      )}
                      <Link to={createLocWithPath(`/bookmark/${bm.id}`)} className="absolute inset-0 z-0" />
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>

          <div className="grow flex flex-col shadow shadow-black overflow-auto">
            <Outlet />
          </div>
        </div>
      </div>
    </div>
  )
}
