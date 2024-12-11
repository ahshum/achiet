import useFetchTags from "@/hooks/useFetchTags"
import useLocationTag from "@/hooks/useLocationTag"
import useTagInput from "@/hooks/useTagInput"
import { TagIcon, XCircleIcon } from "@heroicons/react/16/solid"
import clsx from "clsx"
import { useCombobox } from "downshift"
import { useEffect, useMemo, useRef, useState } from "react"
import { useHotkeys, useHotkeysContext } from "react-hotkeys-hook"
import { Link, useNavigate } from "react-router-dom"

const HOTKEY_SCOPE_1 = "currentPath_combo1"
const HOTKEY_SCOPE_2 = "currentPath_combo2"

export default function CurrentPath() {
  const inputRef = useRef<HTMLInputElement>(null)
  const [currentPath, createLocWithTag,] = useLocationTag()
  const [isInput, setIsInput] = useState<boolean>(false)
  const {
    inputValue,
    setInputValue,
    normalizedInput,
    isRoot,
    filterTags,
  } = useTagInput()
  const navigate = useNavigate()
  const { disableScope, enableScope } = useHotkeysContext()
  const { data: tags } = useFetchTags()

  const hasExact = useMemo((): boolean => {
    if (!tags) {
      return false
    }
    return isRoot || tags.some(tag => tag.path === normalizedInput)
  }, [tags, isRoot, normalizedInput])

  const items = useMemo((): TagModel[] => {
    if (!tags) {
      return []
    }
    return filterTags(tags)
  }, [tags, filterTags])

  const {
    isOpen,
    getMenuProps,
    getItemProps,
    highlightedIndex,
    setHighlightedIndex,
    openMenu,
    closeMenu,
    getInputProps,
  } = useCombobox({
    items,
    inputValue,
    selectedItem: null,
    onStateChange: ({ type, selectedItem: newSelectedItem, inputValue: newInputValue }) => {
      switch (type) {
        case useCombobox.stateChangeTypes.ItemClick:
          if (newSelectedItem) {
            setInputValue(newSelectedItem.path)
          }
          break
        case useCombobox.stateChangeTypes.InputKeyDownEnter:
          if (newSelectedItem) {
            setInputValue(newSelectedItem.path)
          }
          break
        case useCombobox.stateChangeTypes.InputChange:
          setInputValue(newInputValue || "")
          break
        default:
          break
      }
    },
  })

  const dsInputProps = getInputProps({}, { suppressRefError: true })

  useHotkeys("c", () => {
    enableScope(HOTKEY_SCOPE_2)
  }, {
    scopes: HOTKEY_SCOPE_1,
    enabled: !isInput,
    preventDefault: true,
  })

  useHotkeys("d", () => {
    disableScope(HOTKEY_SCOPE_2)
    setIsInput(true)
    setInputValue(currentPath)
    inputRef.current?.focus()
    openMenu()
  }, {
    scopes: HOTKEY_SCOPE_2,
    enabled: !isInput,
    preventDefault: true,
  })

  useHotkeys("*", () => {
    disableScope(HOTKEY_SCOPE_2)
  }, {
    scopes: HOTKEY_SCOPE_2,
    enabled: !isInput,
    preventDefault: true,
  })

  useEffect(() => {
    const handleDocBlur = () => {
      inputRef.current?.blur()
    }

    enableScope(HOTKEY_SCOPE_1)
    document.addEventListener("blur", handleDocBlur)
    return () => {
      disableScope(HOTKEY_SCOPE_1)
      document.removeEventListener("blur", handleDocBlur)
    }
  }, [enableScope, disableScope])

  return (
    <div className="flex items-center space-x-1">
      <TagIcon className="size-5" />
      <div className={clsx("flex", isInput && "hidden")}>
        {currentPath.split("/").map((subPath, idx, subPaths) => (
          <div key={`${currentPath}-${idx}`} className="flex group">
            <div className="px-1">/</div>
            <Link to={createLocWithTag(subPaths.slice(0, idx).join("/"))} className="group-hover:outline group-hover:outline-gray-600 rounded relative items-center">
              <span>
                {subPath}
              </span>
              <XCircleIcon className="size-5 hidden group-hover:block absolute inset-y-0 left-full" />
            </Link>
          </div>
        )).slice(1)}
      </div>
      <div className="flex flex-none relative">
        <input
          tabIndex={-1}
          className={clsx(
            "w-0 focus:w-auto rounded px-0 focus:px-1 focus:min-w-[200px]",
          )}
          ref={inputRef}
          id={dsInputProps.id}
          value={inputValue}
          onFocus={() => {
            openMenu()
          }}
          onChange={(e) => {
            setInputValue(e.currentTarget.value)
            openMenu()
          }}
          onBlur={() => {
            setIsInput(false)
            closeMenu()
          }}
          onKeyDown={(e) => {
            const selectBestPath = (): string => {
              if (highlightedIndex >= 0) {
                return items[highlightedIndex].path
              } else if (inputValue.replace(/[/\s]+/, "") === "") {
                return "/"
              } else if (items.length > 0) {
                return items[0].path
              }
              return "/"
            }

            if (e.key === "Escape") {
              e.preventDefault()
              e.currentTarget.blur()
            } else if (e.key === "Tab") {
              e.preventDefault()
              setInputValue(selectBestPath())
              setHighlightedIndex(-1)
            } else if (e.key === "Enter") {
              e.preventDefault()
              if (highlightedIndex >= 0) {
                setInputValue(selectBestPath())
                setHighlightedIndex(-1)
              } else if (hasExact) {
                e.currentTarget.blur()
                navigate(createLocWithTag(inputValue))
                closeMenu()
              }
            } else {
              dsInputProps.onKeyDown?.(e)
            }
          }}
        />
        <div
          className={clsx(
            "absolute top-full max-h-[120px] overflow-auto",
            "inset-x-0 z-[100] text-sm",
            "w-full flex flex-col",
            "shadow shadow-black text-sm",
            "bg-[var(--color-tag-select-bg)]",
            !isOpen && "hidden",
          )}
          {...getMenuProps()}
        >
          {items.map((item, index) => (
            <div
              key={item.path}
              className={clsx(
                "px-4 py-0.5",
                highlightedIndex === index && "bg-[var(--color-tag-highlight-bg)]",
              )}
              {...getItemProps({ item, index })}
            >
              {item.path}
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
