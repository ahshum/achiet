import { TextareaHTMLAttributes } from "react"
import { useController, UseControllerProps } from "react-hook-form"

export type TextareaProps = TextareaHTMLAttributes<HTMLTextAreaElement> & UseControllerProps

function splitProps(props: TextareaProps): [UseControllerProps, TextareaHTMLAttributes<HTMLTextAreaElement>] {
  const {
    name, rules, shouldUnregister, defaultValue, control, disabled, ...rest
  } = props
  return [
    { name, rules, shouldUnregister, defaultValue, control, disabled },
    rest,
  ]
}

export default function Textarea(props: TextareaProps) {
  const [fieldProps, textareaProps] = splitProps(props)
  const {
    field: { value, onChange, onBlur }
  } = useController(fieldProps)

  return (
    <textarea
      {...textareaProps}
      className="outline-none rounded border px-2 py-1 bg-[var(--color-input-bg)] border-[var(--color-input-border)] focus:border-[var(--color-input-border-focus)]"
      value={value || ""}
      onChange={onChange}
      onBlur={onBlur}
    />
  )
}
