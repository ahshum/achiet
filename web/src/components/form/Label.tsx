import { LabelHTMLAttributes } from "react"

export type LabelProps = LabelHTMLAttributes<HTMLLabelElement>

export default function Label(props?: LabelProps) {
  return (
    <label
      {...props}
    />
  )
}
