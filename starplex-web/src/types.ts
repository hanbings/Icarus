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

export interface SimpleRating {
    openid: string,
    created: number,
    rank: number,
    username: string,
    nickname: string,
    avatar: string,
    company: string,
    location: string,
    country: string,
    twitter: string,

    star: number,
    followers: number,
    rating: Rating,

    ai_rating: number,
    languages: string[],
    repositories: number,

    summarize: string,
    blog: string
}

export interface Rating {
    is_bio_exist: boolean,
    is_company_exist: boolean,
    is_location_exist: boolean,
    is_blog_exist: boolean,
    bio_rating: number,
    backlinks_rating: number,
    repositories_description_rating: number,
    webpages_rating: number,
    user_popularity: number,
    repositories_popularity: number,
    forks_count: number,
    stars_count: number,
    repositories_count: number,
    followers_count: number,
}