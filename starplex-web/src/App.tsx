import {NotFoundScreen} from "./screens/NotFound.tsx";
import {ErrorScreen} from "./screens/Error.tsx";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import {useTokenStore} from "./stores/token.ts";
import {configureStore} from "@reduxjs/toolkit";
import LoginScreen from "./screens/Login.tsx";
import WelcomeScreen from "./screens/Welcome.tsx";
import {Provider} from "react-redux";
import RankScreen from "./screens/Rank.tsx";
import ProfileScreen from "./screens/Profile.tsx";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import OAuthScreen from "./screens/OAuth.tsx";
import {useAccountStore} from "./stores/account.ts";

function App() {
    const store = configureStore({
        reducer: {
            token: useTokenStore.reducer,
            account: useAccountStore.reducer
        },
    });

    const router = createBrowserRouter([
        {path: "/", element: <WelcomeScreen/>},
        {path: "/login", element: <LoginScreen/>},
        {path: "/rank", element: <RankScreen/>},
        {path: "/profile/:username", element: <ProfileScreen/>},
        {path: "/oauth/github/callback", element: <OAuthScreen/>},
        {path: "/error", element: <ErrorScreen/>},
        {path: "*", element: <NotFoundScreen/>},
    ])

    const query = new QueryClient()

    return (
        <QueryClientProvider client={query}>
            <Provider store={store}>
                <RouterProvider router={router}/>
            </Provider>
        </QueryClientProvider>
    )
}

export default App
