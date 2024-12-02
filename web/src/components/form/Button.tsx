import { ForwardedRef, forwardRef, ButtonHTMLAttributes } from "react"

export type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> & {
  component?: React.Component,
}

export default forwardRef(function Input(props: ButtonProps, ref: ForwardedRef<HTMLButtonElement>) {
  return (
    <button
      {...props}
      ref={ref}
      className="outline-none border p-1 rounded border-[var(--color-input-border)] focus:border-[var(--color-input-border-focus)]"
    />
  )
})
