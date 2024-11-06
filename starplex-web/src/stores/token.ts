import {createSlice} from "@reduxjs/toolkit";

export const useTokenStore = createSlice(
    {
        name: "token",
        initialState: {
            token: undefined
        },
        reducers: {
            setToken: (state, action) => {
                state.token = action.payload
            },
            clearToken: (state) => {
                state.token = undefined
            },
        }
    }
)

export const {setToken, clearToken} = useTokenStore.actions