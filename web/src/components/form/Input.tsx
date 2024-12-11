import { InputHTMLAttributes } from "react"
import { useController, UseControllerProps } from "react-hook-form"

export type InputProps = InputHTMLAttributes<HTMLInputElement> & UseControllerProps

function splitProps(props: InputProps): [UseControllerProps, InputHTMLAttributes<HTMLInputElement>] {
  const {
    name, rules, shouldUnregister, defaultValue, control, disabled, ...rest
  } = props
  return [
    { name, rules, shouldUnregister, defaultValue, control, disabled },
    rest,
  ]
}

export default function Input(props: InputProps) {
  const [fieldProps, inputProps] = splitProps(props)
  const {
    field: { value, onChange, onBlur, ref, name }
  } = useController(fieldProps)

  return (
    <input
      {...inputProps}
      className="outline-none rounded border px-2 py-1 bg-[var(--color-input-bg)] border-[var(--color-input-border)] focus:border-[var(--color-input-border-focus)]"
      value={value || ""}
      onChange={onChange}
      onBlur={onBlur}
      ref={ref}
      name={name}
    />
  )
}
