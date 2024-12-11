import { accessTokenAtom } from "@/shared/state"
import { useAtom } from "jotai"
import { useCallback, useEffect } from "react"
import { Link, useNavigate } from "react-router-dom"
import { FormProvider, useForm } from "react-hook-form"
import Input from "@/components/form/Input.jsx"
import Label from "@/components/form/Label.jsx"
import Button from "@/components/form/Button.jsx"
import ErrorMessage from "@/components/form/ErrorMessage.jsx"
import useAuthRequest from "@/hooks/useAuthRequest"

export type LoginFormData = {
  username: string,
  password: string,
}

export type LoginProps = {
  isRegister?: boolean,
}

export default function Login(props: LoginProps) {
  const isRegister = props.isRegister || false
  const [accessToken] = useAtom(accessTokenAtom)
  const navigate = useNavigate()
  const mutation = useAuthRequest({ isRegister })
  const methods = useForm<LoginFormData>({
    mode: "onSubmit"
  })

  useEffect(() => {
    if (accessToken) {
      navigate("/")
    }
  }, [accessToken, navigate])

  const onSubmit = useCallback(async (data: LoginFormData) => {
    await mutation.mutateAsync(data)
  }, [mutation])

  return (
    <div className="flex min-h-screen">
      <div className="flex flex-col min-w-[320px] p-2 m-auto">
        <FormProvider {...methods}>
          <div className="flex flex-col gap-4">
            <h2 className="text-xl">
              <span className="font-bold">
                Achiet
              </span>
              <span className="border-r border-solid mx-3" />
              <span>
                {isRegister ? "Register" : "Login"}
              </span>
            </h2>

            <div className="grid">
              <Label>Username</Label>
              <Input
                name="username"
                rules={{
                  required: "Username is required",
                }}
              />
              <ErrorMessage errors={methods.formState.errors} name="username" />
            </div>

            <div className="grid">
              <Label>Password</Label>
              <Input
                type="password"
                name="password"
                rules={{
                  required: "Password is required",
                }}
              />
              <ErrorMessage errors={methods.formState.errors} name="password" />
            </div>

            <Button onClick={methods.handleSubmit(onSubmit)}>
              {isRegister ? "Register" : "Login"}
            </Button>

            {isRegister ? (
              <Link className="text-center" to="/login">Switch to Login</Link>
            ) : (
              <Link className="text-center" to="/register">Switch to Register</Link>
            )}
          </div>
        </FormProvider>
      </div>
    </div>
  )
}
