import { useCallback, useMemo } from "react"
import { useController, UseControllerProps } from "react-hook-form"
import { useCombobox, useMultipleSelection } from "downshift"
import clsx from "clsx"
import useFetchTags from "@/hooks/useFetchTags"
import useTagInput from "@/hooks/useTagInput"
import TagChip from "./TagChip"

export type TagSelectProps = UseControllerProps & {
  tabIndex?: number,
}

type Tagging = {
  path: string,
  value?: string,
  raw: string,
}

function parseTagging(str: string): Tagging {
  const [path, value] = str.split(":")
  return { path, value, raw: str }
}

function formatTagging(tg: Tagging): string {
  return tg.value
    ? `${tg.path}:${tg.value}`
    : tg.path
}

export default function TagSelect(props: TagSelectProps) {
  const {
    field: { value, onChange, ref, name }
  } = useController(props)
  const { data: tags, isSuccess } = useFetchTags()
  const {
    inputValue,
    setInputValue,
    isRoot,
    filterTags,
  } = useTagInput()

  const selectedItems = useMemo((): Tagging[] => {
    if (!value) {
      return []
    }
    return value.map(parseTagging)
  }, [value])

  const setSelectedItems = useCallback((newValue?: Tagging[]) => {
    onChange((newValue || []).map(formatTagging))
  }, [onChange])

  const items = useMemo((): Tagging[] => {
    if (!isSuccess) {
      return []
    }

    const items = tags
      .map(t => ({ path: t.path, raw: t.path }))

    if (isRoot) {
      return items
    }
    return filterTags(items)
  }, [tags, isSuccess, filterTags, isRoot])

  const {
    activeIndex,
    getDropdownProps,
    setActiveIndex,
  } = useMultipleSelection({
    selectedItems,
    onStateChange: ({ type }) => {
      switch (type) {
        case useMultipleSelection.stateChangeTypes.SelectedItemKeyDownBackspace:
        case useMultipleSelection.stateChangeTypes.SelectedItemKeyDownDelete:
        case useMultipleSelection.stateChangeTypes.DropdownKeyDownBackspace:
        case useMultipleSelection.stateChangeTypes.FunctionRemoveSelectedItem: {
          let idx = activeIndex
          if (idx < 0 && selectedItems.length > 0) {
            idx = selectedItems.length - 1
          }

          if (idx > -1) {
            const item = selectedItems[idx]
            setActiveIndex(-1)
            setSelectedItems([ ...selectedItems.slice(0, idx), ...selectedItems.slice(idx + 1) ])
            setInputValue(formatTagging(item))
          }
          break
        }
        default:
          break
      }
    },
  })

  const {
    isOpen,
    openMenu,
    closeMenu,
    highlightedIndex,
    setHighlightedIndex,
    getInputProps,
    getMenuProps,
    getItemProps,
  } = useCombobox({
    items,
    inputValue,
    selectedItem: null,
    stateReducer: (_state, { type, changes }) => {
      switch (type) {
        case useCombobox.stateChangeTypes.InputKeyDownEnter:
        case useCombobox.stateChangeTypes.ItemClick:
          return { ...changes, isOpen: true }
        default:
          return changes
      }
    },
  })

  const dsInputProps = getInputProps(getDropdownProps({}, { suppressRefError: true }), { suppressRefError: true })

  return (
    <div className="flex relative">
      <div
        className={clsx(
          "w-full border",
          "rounded px-2 py-1 flex flex-row flex-wrap content-start items-center gap-1",
          "bg-[var(--color-input-bg)]",
          "border-[var(--color-input-border)]",
          "focus-within:border-[var(--color-input-border-focus)]",
        )}
      >
        {selectedItems.map((item, index) => (
          <TagChip
            key={item.path}
            tag={{ path: item.raw }}
            className={clsx(
              activeIndex === index && "outline",
            )}
          />
        ))}
        <input
          className="outline-none bg-transparent flex-1 min-w-0"
          value={inputValue}
          onChange={(e) => {
            setActiveIndex(-1)
            setInputValue(e.currentTarget.value)
          }}
          onBlur={() => closeMenu()}
          onFocus={() => openMenu()}
          onClick={dsInputProps.onClick}
          ref={ref}
          name={name}
          tabIndex={props.tabIndex}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              e.stopPropagation()
              if (highlightedIndex >= 0) {
                setHighlightedIndex(-1)
                setInputValue(items[highlightedIndex].path)
              } else if (inputValue) {
                const [path, value] = inputValue.split(":")
                if (selectedItems.findIndex(item => item.path === path) < 0) {
                  setSelectedItems([ ...selectedItems, { path, value, raw: inputValue } ])
                  setInputValue("")
                }
              }
            } else if (e.key === "ArrowLeft") {
              if (!inputValue) {
                setActiveIndex((activeIndex + selectedItems.length + 1) % (selectedItems.length + 1) - 1)
              }
            } else if (e.key === "ArrowRight") {
              if (!inputValue) {
                setActiveIndex((activeIndex + 2) % (selectedItems.length + 1) - 1)
              }
            } else {
              dsInputProps.onKeyDown?.(e)
            }
          }}
        />
      </div>
      <div
        className={clsx(
          "absolute top-full max-h-[120px] overflow-auto",
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
  )
}
