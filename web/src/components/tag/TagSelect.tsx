import { useCallback, useMemo, useState } from "react"
import { useController, UseControllerProps } from "react-hook-form"
import { useCombobox, useMultipleSelection } from "downshift"
import clsx from "clsx"
import useFetchTags from "@/hooks/useFetchTags"

export type TagSelectProps = UseControllerProps

type Tagging = {
  path: string,
  value?: string,
}

export default function TagSelect(props: TagSelectProps) {
  const {
    field: { value, onChange, onBlur }
  } = useController(props)
  const { data: tags, isSuccess } = useFetchTags()
  const [inputValue, setInputValue] = useState<string>("")

  const selectedItems = useMemo((): Tagging[] => {
    if (!value) {
      return []
    }
    return value.map((str: string) => {
      const [path, value] = str.split(":")
      return { path, value }
    })
  }, [value])

  const setSelectedItems = useCallback((newValue?: Tagging[]) => {
    onChange((newValue || []).map(t => {
      return t.value
        ? `${t.path}:${t.value}`
        : t.path
    }))
  }, [onChange])

  const normalizedInput = useMemo((): string => {
    return (inputValue || "")
      .replace(/^\/*/, "/")
      .replace(/\/+$/, "")
  }, [inputValue])

  const hasExact = useMemo((): boolean => {
    return !!tags?.some(t => t.path === normalizedInput)
  }, [tags, normalizedInput])

  const items = useMemo((): Tagging[] => {
    if (!isSuccess) {
      return []
    }

    const items = tags
      .map(t => ({ path: t.path }))
      .filter(t => !selectedItems.some(s => s.path == t.path))

    if (!inputValue) {
      return items
    }

    const re = new RegExp(".*" + inputValue.split("").join(".*") + ".*")
    const matches = items.filter(t => re.test(t.path))
    return hasExact
      ? matches.filter(t => t.path !== normalizedInput)
      : matches
  }, [tags, isSuccess, inputValue, selectedItems, hasExact, normalizedInput])

  const multiSelectionState = useMultipleSelection({
    selectedItems,
    onStateChange: ({ selectedItems: newSelectedItems, type }) => {
      switch (type) {
        case useMultipleSelection.stateChangeTypes.SelectedItemKeyDownBackspace:
        case useMultipleSelection.stateChangeTypes.SelectedItemKeyDownDelete:
        case useMultipleSelection.stateChangeTypes.DropdownKeyDownBackspace:
        case useMultipleSelection.stateChangeTypes.FunctionRemoveSelectedItem:
          setSelectedItems(newSelectedItems)
          break
        default:
          break
      }
    },
  })

  const comboboxState = useCombobox({
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
            {...multiSelectionState.getSelectedItemProps({ selectedItem: item, index })}
          >
            {item.path}
            {item.value && `:${item.value}`}
          </div>
        ))}
        <input
          className="outline-none bg-transparent flex-1 min-w-0"
          {...comboboxState.getInputProps(multiSelectionState.getDropdownProps({
            onKeyDown: (e) => {
              if (e.key === "Enter") {
                if (comboboxState.highlightedIndex < 0 && inputValue) {
                  const [path, value] = inputValue.split(":")
                  setSelectedItems([ ...selectedItems, { path, value } ])
                  setInputValue("")
                }
                e.stopPropagation()
              }
            },
          }))}
          onBlur={onBlur}
        />
      </div>
      <div
        className={clsx(
          "absolute top-full max-h-[120px] overflow-auto",
          "w-full flex flex-col",
          "shadow shadow-black",
          "bg-[#202020]",
          !comboboxState.isOpen && "hidden",
        )}
        {...comboboxState.getMenuProps()}
      >
        {items.map((item, index) => (
          <div
            key={item.path}
            className={clsx(
              "px-4 py-0.5 text-sm",
              comboboxState.highlightedIndex === index && "bg-[#3f3f3f]",
            )}
            {...comboboxState.getItemProps({ item, index })}
          >
            {item.path}
          </div>
        ))}
      </div>
    </div>
  )
}
