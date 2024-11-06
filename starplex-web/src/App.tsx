import {NotFoundScreen} from "./screens/NotFound.tsx";
import {ErrorScreen} from "./screens/Error.tsx";
import HomeScreen from "./screens/Home.tsx";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import {useTokenStore} from "./stores/token.ts";
import {configureStore} from "@reduxjs/toolkit";
import LoginScreen from "./screens/Login.tsx";
import WelcomeScreen from "./screens/Welcome.tsx";
import {Provider} from "react-redux";

function App() {
    const store = configureStore({
        reducer: {
            token: useTokenStore.reducer
        },
    });

    const router = createBrowserRouter([
        {path: "/", element: <WelcomeScreen/>},
        {path: "/login", element: <LoginScreen/>},
        {path: "/home", element: <HomeScreen/>},
        {path: "/error", element: <ErrorScreen/>},
        {path: "*", element: <NotFoundScreen/>},
    ])

    return (
        <Provider store={store}>
            <RouterProvider router={router}/>
        </Provider>
    )
}

export default App
