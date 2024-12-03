import { FieldErrors, FieldName, FieldValues } from "react-hook-form"

export type ErrorMessageProps = {
  name: FieldName<FieldValues>,
  errors: FieldErrors,
}

export default function ErrorMessage({ errors, name }: ErrorMessageProps) {
  return errors[name] && (
    <div className="text-red-300">
      {errors[name].message?.toString()}
    </div>
  )
}
