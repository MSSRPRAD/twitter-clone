// @refresh reload
import { Suspense } from "solid-js";
import {
  useLocation,
  A,
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Meta,
  Routes,
  Scripts,
  Title,
  Route,
} from "solid-start";
import "./root.css";
import Register from "./routes/register";
import Login from "./routes/login";
import User from "./routes/user";
import Timeline from "./routes/timeline.tsx";
import ViewTweet from "./routes/tweet.tsx";
export default function Root() {
  const location = useLocation();
  const active = (path: string) =>
    path == location.pathname
      ? "border-sky-600"
      : "border-transparent hover:border-sky-600";
  return (
    <Html lang="en">
      <Head>
        <Title>Twitter</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary>
            <nav class="bg-sky-800">
              <ul class="container flex items-center p-3 text-gray-200">
                <li class={`border-b-2 ${active("/")} mx-1.5 sm:mx-6`}>
                  <A href="/register">Register</A>
                </li>
                <li class={`border-b-2 ${active("/")} mx-1.5 sm:mx-6`}>
                  <A href="/login">Login</A>
                </li>
                <li class={`border-b-2 ${active("/about")} mx-1.5 sm:mx-6`}>
                  <A href="/about">About</A>
                </li>
              </ul>
            </nav>
            <Routes>
              <FileRoutes />
              <Route path = "/register" component={Register}/>
              <Route path = "/login" component={Login}/>
              <Route path="/users/:username" component={User} />
              <Route path="/timeline/me" component={Timeline} />
              <Route path="/tweet/:tweet_id" component={ViewTweet} />
            </Routes>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
