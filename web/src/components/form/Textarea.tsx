import { TextareaHTMLAttributes } from "react"
import { useController, UseControllerProps } from "react-hook-form"

export type TextareaProps = TextareaHTMLAttributes<HTMLTextAreaElement> & UseControllerProps

export default function Textarea(props: TextareaProps) {
  const {
    name,
    rules,
    shouldUnregister,
    defaultValue,
    disabled,
    control,
    ...restProps
  } = props
  const {
    field: { value, onChange, onBlur }
  } = useController({ name, rules, shouldUnregister, defaultValue, disabled, control })

  return (
    <textarea
      {...restProps}
      className="outline-none rounded border px-2 py-1 bg-[var(--color-input-bg)] border-[var(--color-input-border)] focus:border-[var(--color-input-border-focus)]"
      value={value || ""}
      onChange={onChange}
      onBlur={onBlur}
    />
  )
}
