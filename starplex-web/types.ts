export interface Token {
    token: string,
    belong: string,
    permissions: string[],
    expire: number,
    created: number
}