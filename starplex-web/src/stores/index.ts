import {Account, Token} from "../types.ts";

export type AppStore = {
    token: {
        token: undefined | Token
    },
    account: {
        account: undefined | Account
    }
}