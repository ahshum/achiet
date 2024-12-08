import { useCallback, useMemo } from "react"
import { useController, UseControllerProps } from "react-hook-form"
import { useCombobox, useMultipleSelection } from "downshift"
import clsx from "clsx"
import useFetchTags from "@/hooks/useFetchTags"
import useTagInput from "@/hooks/useTagInput"

export type TagSelectProps = UseControllerProps & {
  tabIndex?: number,
}

type Tagging = {
  path: string,
  value?: string,
}

function parseTagging(str: string): Tagging {
  const [path, value] = str.split(":")
  return { path, value }
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
    normalizedInput,
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

  const hasExact = useMemo((): boolean => {
    return !!tags?.some(t => t.path === normalizedInput)
  }, [tags, normalizedInput])

  const items = useMemo((): Tagging[] => {
    if (!isSuccess) {
      return []
    }

    const items = tags
      .map(t => ({ path: t.path }))

    if (isRoot) {
      return items
    }
    return filterTags(items)
  }, [tags, isSuccess, filterTags, isRoot])

  const {
    activeIndex,
    getDropdownProps,
    getSelectedItemProps,
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
          <div
            key={item.path}
            className="border border-white rounded-full px-2 text-sm"
            {...getSelectedItemProps({ selectedItem: item, index })}
          >
            {item.path}
            {item.value && `:${item.value}`}
          </div>
        ))}
        <input
          className="outline-none bg-transparent flex-1 min-w-0"
          value={inputValue}
          onChange={(e) => setInputValue(e.currentTarget.value)}
          onBlur={() => closeMenu()}
          onFocus={() => openMenu()}
          onClick={dsInputProps.onClick}
          ref={ref}
          name={name}
          tabIndex={props.tabIndex}
          onKeyDown={(e) => {
            if (e.key === "Enter" && highlightedIndex < 0) {
              if (inputValue) {
                const [path, value] = inputValue.split(":")
                setSelectedItems([ ...selectedItems, { path, value } ])
                setInputValue("")
              }
              e.stopPropagation()
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
