export interface ConfigCenterEntry {
    key: string,
    value: string,
}

export interface ExploreServiceEntry {
    endpoint: string,
    created: number,
    last_updated: number,
    service_name: string,
    instance_name: string,
}

export interface MessageQueue {
    channel: string,
    entries: string[],
}