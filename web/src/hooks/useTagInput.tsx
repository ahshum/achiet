import { useCallback, useMemo, useState } from "react"

function escapeChar(char: string): string {
  if ("()[]./\\+=".includes(char)) {
    return `\\${char}`
  }
  return char
}

export default function useTagInput() {
  const [inputValue, setInputValue] = useState<string>("")

  const normalizedInput = useMemo((): string => {
    return inputValue
      .replace(/\/+$/, "")
      .replace(/^\/*/, "/")
  }, [inputValue])

  const isRoot = useMemo((): boolean => {
    return normalizedInput === "/"
  }, [normalizedInput])

  const filterTags = useCallback(<T extends TagModel | Pick<TagModel, "path">>(tags: T[]): T[] => {
    const re = new RegExp(["", ...inputValue.split("").map(escapeChar), ""].join(".*"))
    const matches = tags.filter(t => re.test(t.path))
    return matches
  }, [inputValue])

  return {
    inputValue,
    setInputValue,
    normalizedInput,
    isRoot,
    filterTags,
  }
}
