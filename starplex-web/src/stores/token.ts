import {createSlice} from "@reduxjs/toolkit";

export const useTokenStore = createSlice(
    {
        name: "token",
        initialState: {
            token: localStorage.getItem('token') || undefined
        },
        reducers: {
            setToken: (state, action) => {
                state.token = action.payload
                localStorage.setItem('token', action.payload)
            },
            clearToken: (state) => {
                state.token = undefined
                localStorage.clear()
            },
        }
    }
)

export const {setToken, clearToken} = useTokenStore.actions