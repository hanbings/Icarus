export interface Token {
    token: string,
    belong: string,
    permissions: string[],
    expire: number,
    created: number
}

export interface Account {
    openid: string,
    username: string,
    avatar: string,
    email?: string[],
    nickname: string,
}