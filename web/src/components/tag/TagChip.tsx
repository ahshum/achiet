import useLocationTag from "@/hooks/useLocationTag"
import clsx from "clsx"
import { useMemo } from "react"

export type TagChipProps = {
  tag: TagModel | Pick<TagModel, "path">,
  hideForCurrent?: boolean,
  hideValue?: boolean,
  className?: string,
}

export default function TagChip(props: TagChipProps) {
  const { 
    tag: { path: pathStr },
    hideForCurrent = false,
    hideValue = false,
    className,
  } = props
  const [currentPath, , isRoot] = useLocationTag()

  const [path, value] = useMemo((): [string, string?] => {
    const [path, value] = pathStr.split(":")
    return [path, value]
  }, [pathStr])

  const subPaths = useMemo((): string[] => path.split("/"), [path])
  const prefix = useMemo((): string => subPaths.slice(0, subPaths.length - 1).join("/"), [subPaths])
  const name = useMemo((): string => subPaths[subPaths.length - 1], [subPaths])

  if (hideForCurrent && !isRoot && currentPath === path) {
    return null
  }
  return (
    <div className={clsx("flex-none rounded-full border border-white px-2 flex flex-row text-sm", className)}>
      <span className="opacity-60">{prefix}/</span>
      <span>{name}</span>
      {!hideValue && value && (<span>:{value}</span>)}
    </div>
  )
}
