import { ForwardedRef, forwardRef, LabelHTMLAttributes } from "react"

export type LabelProps = LabelHTMLAttributes<HTMLLabelElement>

export default forwardRef(function Input(props: LabelProps, ref: ForwardedRef<HTMLLabelElement>) {
  return (
    <label
      {...props}
      ref={ref}
    />
  )
})
