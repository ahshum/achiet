import clsx from "clsx"
import { ButtonHTMLAttributes } from "react"

export type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement>

export default function Button(props?: ButtonProps) {
  return (
    <button
      {...props}
      className={clsx(
        "outline-none border p-1 rounded border-[var(--color-input-border)] focus:border-[var(--color-input-border-focus)]",
        props?.className,
      )}
    />
  )
}
