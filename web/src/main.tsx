import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import "./index.scss"
import { createBrowserRouter, RouterProvider } from "react-router-dom"
import Root from "./routes/Root.tsx"
import Login from "./routes/Login.tsx"
import BookmarkEdit from "./routes/BookmarkEdit.tsx"
import { QueryClient, QueryClientProvider } from "@tanstack/react-query"
import { HotkeysProvider } from "react-hotkeys-hook"

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    children: [
      {
        path: "/bookmark/new",
        element: <BookmarkEdit isNew />,
      },
      {
        path: "/bookmark/:bookmarkId",
        element: <BookmarkEdit />,
      },
    ],
  },
  {
    path: "/login",
    element: <Login />,
  },
  {
    path: "/register",
    element: <Login isRegister />,
  },
])

const queryClient = new QueryClient()

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <HotkeysProvider>
        <RouterProvider router={router} />
      </HotkeysProvider>
    </QueryClientProvider>
  </StrictMode>,
)
