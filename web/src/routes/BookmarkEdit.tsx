import Button from "@/components/form/Button"
import Input from "@/components/form/Input"
import Label from "@/components/form/Label"
import Textarea from "@/components/form/Textarea"
import TagSelect from "@/components/tag/TagSelect"
import useDeleteBookmark from "@/hooks/useDeleteBookmark"
import useFetchBookmarkById from "@/hooks/useFetchBookmarkById"
import useLocationMode, { Mode } from "@/hooks/useLocationMode"
import useLocationPath from "@/hooks/useLocationPath"
import useSaveBookmark from "@/hooks/useSaveBookmark"
import { LinkIcon, XMarkIcon } from "@heroicons/react/16/solid"
import { useCallback, useEffect, useMemo } from "react"
import { FormProvider, useForm } from "react-hook-form"
import { Link, useNavigate, useParams } from "react-router-dom"

export type BookmarkEditProps = {
  isNew?: boolean,
}

export default function BookmarkEdit(props: BookmarkEditProps) {
  const { isNew = false } = props
  const { bookmarkId } = useParams()
  const methods = useForm<BookmarkModel>()
  const { data, isSuccess } = useFetchBookmarkById(bookmarkId!, {
    enabled: !isNew
  })
  const { mutateAsync } = useSaveBookmark()
  const [, createLocWithPath] = useLocationPath()
  const [, createLocWithMode] = useLocationMode()
  const [mode] = useLocationMode()
  const { reset, formState } = methods
  const navigate = useNavigate()
  const { mutateAsync: deleteAsync } = useDeleteBookmark()

  const isReady = useMemo((): boolean => {
    return isSuccess || isNew
  }, [isSuccess, isNew])

  const currentMode = useMemo((): Mode => {
    if (isNew) {
      return Mode.Edit
    }
    return mode
  }, [mode, isNew])

  const handleSubmit = methods.handleSubmit(async (data) => {
    const bm = await mutateAsync(data)
    if (isNew) {
      navigate(createLocWithPath(`/bookmark/${bm.id}`))
    }
    navigate(createLocWithMode(Mode.View))
  })

  const handleDelete = useCallback(async () => {
    if (!isSuccess) {
      return
    }
    await deleteAsync(data)
    navigate(createLocWithPath("/"))
  }, [data, isSuccess, deleteAsync, createLocWithPath, navigate])

  useEffect(() => {
    if (isNew) {
      reset({
        id: null,
        title: "",
        url: "",
        description: "",
        tags: [],
      })
      return
    } else if (isSuccess) {
      reset(data, {
        keepValues: formState.defaultValues?.id === data.id,
      })
    }
  }, [isSuccess, data, isNew])

  return isReady && (
    <div className="flex flex-col py-8 relative">
      {currentMode === Mode.View && (
        <div className="flex flex-col px-8">
          <h2 className="text-xl">
            {data?.title}
          </h2>
          {data?.url && (
            <a className="flex flex-row flex-wrap items-center hover:underline" href={data?.url} target="_blank">
              <LinkIcon className="size-5 mr-1" />
              <span>
                {data?.url}
              </span>
            </a>
          )}
          {data?.tags && data.tags.length > 0 && (
            <div className="flex flex-row flex-wrap pt-4">
              {data?.tags.map(tag => (
                <div key={tag} className="border border-white rounded-full px-2 text-sm">
                  {tag}
                </div>
              ))}
            </div>
          )}
          <div className="pt-4">
            {data?.description}
          </div>
        </div>
      )}

      {currentMode === Mode.Edit && (
        <FormProvider {...methods}>
          <form className="flex flex-col px-8 gap-4" onSubmit={handleSubmit}>

            <div className="grid">
              <Label>Title</Label>
              <Input name="title" />
            </div>

            <div className="grid">
              <Label>URL</Label>
              <Input name="url" />
            </div>

            <div className="grid">
              <Label>Tags</Label>
              <TagSelect
                name="tags"
              />
            </div>

            <div className="grid">
              <Label>Description</Label>
              <Textarea name="description" />
            </div>

            <Button>
              Save
            </Button>

            {!isNew && (
              <Button type="button" className="text-red-300" onClick={handleDelete}>
                Delete
              </Button>
            )}

          </form>
        </FormProvider>
      )}

      <Link to={createLocWithPath("/")}  className="absolute right-0 top-0 p-2">
        <XMarkIcon className="size-6" />
      </Link>
    </div>
  )
}