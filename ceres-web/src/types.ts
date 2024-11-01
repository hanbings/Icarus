export interface Service {
    code: number,
    time: number,
    status: 'success' | 'failure' | 'timeout',
}