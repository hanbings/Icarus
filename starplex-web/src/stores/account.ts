import {createSlice} from "@reduxjs/toolkit";

export const useAccountStore = createSlice(
    {
        name: "account",
        initialState: {
            account: localStorage.getItem('account') || undefined
        },
        reducers: {
            setAccount: (state, action) => {
                state.account = action.payload
                localStorage.setItem('account', action.payload)
            },
            clearAccount: (state) => {
                state.account = undefined
                localStorage.clear()
            },
        }
    }
)

export const {setAccount, clearAccount} = useAccountStore.actions