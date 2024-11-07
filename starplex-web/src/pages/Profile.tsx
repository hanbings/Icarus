import {useSelector} from "react-redux";
import {AppStore} from "../stores";
import axios from "axios";
import {StarplexConfig} from "../config.ts";
import {Link, Spinner} from "@nextui-org/react";
import {useQuery} from "@tanstack/react-query";
import {SimpleRating} from "../types.ts";

export interface ProfilePageProps {
    username: string,
    isProfilePage: boolean
}

interface SimpleRatingMessage {
    rating: SimpleRating
}

export default function ProfilePage(props: ProfilePageProps) {
    const token = useSelector((state: AppStore) => state.token)
    const account = useSelector((state: AppStore) => state.account)

    const {data, isLoading} = useQuery({
        queryKey: ["profile", props.username, token.token, account.account],
        queryFn: async (): Promise<SimpleRatingMessage> => {
            console.log(token.token?.token)
            if (account.account) {
                const data = await axios.get(`${StarplexConfig.api}/rating`, {
                    headers: {
                        'Authorization': `Bearer ${token.token?.token}`
                    }
                });
                return data.data;
            } else {
                const profile = await axios.get(`${StarplexConfig.api}/github/${props.username}`);
                return profile.data;
            }
        }
    })

    const getRank = (rating: SimpleRating | undefined) => {
        if (!rating) {
            return 0
        }

        let rank = 0
        rank = rank + rating.rating.backlinks_rating
        rank = rank + rating.rating.repositories_description_rating
        rank = rank + rating.rating.webpages_rating
        rank = rank + rating.rating.user_popularity
        rank = rank + rating.rating.repositories_popularity

        return rank / 5
    }

    const profileBentoBox = [
        {
            index: "0",
            span: "col-span-2 row-span-1",
            height: 1,
            width: 2,
            heightGap: 0,
            widthGap: 0.5,
            content: (
                <div className="w-full h-full flex flex-row p-2">
                    <div className="w-1/2 h-full rounded-2xl shadow-xl border-2 border-black"
                         style={{
                             backgroundImage: `url(${account.account?.avatar})`,
                             backgroundSize: "cover",
                             backgroundPosition: "center"
                         }}></div>
                    <div className="m-2 flex flex-col gap-1 justify-center items-center">
                        <div className="p-2">
                            <Link isBlock showAnchorIcon href={`https://github.com/${account.account?.username}`}
                                  className="text-green-400">
                                {`@${account.account?.username}`}
                            </Link>
                            <Link isBlock showAnchorIcon href={`/profile/${account.account?.username}`}
                                  className="text-green-400">
                                {`Profile @${account.account?.username}`}
                            </Link>
                        </div>
                    </div>
                </div>
            )
        },
        {
            index: "1",
            span: "col-span-1 row-span-1",
            height: 1,
            width: 1,
            heightGap: 0,
            widthGap: 0,
            content: (
                <div className="w-full h-full flex flex-row p-2">
                    <div
                        className="w-full h-full rounded-2xl bg-gradient-to-t from-green-300 to-green-50 flex justify-center items-center text-center">
                        <p className="text-6xl text-white">{getRank(data?.rating)}</p>
                    </div>
                </div>
            )
        },
        {
            index: "2",
            span: "col-span-1 row-span-2",
            height: 2,
            width: 1,
            heightGap: 0.5,
            widthGap: 0,
            content: (
                <div className="flex justify-center items-center h-full w-full">
                    <Link isBlock showAnchorIcon href={`${data?.rating.blog}`}
                          className="text-green-400">
                        {`@${data?.rating.blog}'s blog`}
                    </Link>
                </div>
            )
        },
        {
            index: "3",
            span: "col-span-1 row-span-1",
            height: 1,
            width: 1,
            heightGap: 0,
            widthGap: 0,
            content: <div className="flex justify-center items-center h-full w-full rounded-2xl"
                          style={{
                              backgroundImage: `url(https://cdn.jsdelivr.net/npm/programming-languages-logos/src/javascript/javascript.png)`,
                              backgroundSize: "cover",
                              backgroundPosition: "center"
                          }}></div>
        },
        {
            index: "4",
            span: "col-span-1 row-span-1",
            height: 1,
            width: 1,
            heightGap: 0,
            widthGap: 0,
            content: <div className="flex justify-center items-center h-full w-full">LANG#1 / 热门项目</div>
        },
        {
            index: "5",
            span: "col-span-1 row-span-2",
            height: 2,
            width: 1,
            heightGap: 0.5,
            widthGap: 0,
            content: <div className="flex justify-center items-center h-full w-full">Github Hot 热力图</div>
        },
        {
            index: "6",
            span: "col-span-2 row-span-1",
            height: 1,
            width: 2,
            heightGap: 0,
            widthGap: 0.5,
            content: <div className="flex justify-center items-center h-full w-full">LANG#2</div>
        },
        {
            index: "7",
            span: "col-span-1 row-span-1",
            height: 1,
            width: 1,
            heightGap: 0,
            widthGap: 0,
            content: <div className="flex justify-center items-center h-full w-full rounded-2xl"
                          style={{
                              backgroundImage: `url(https://cdn.jsdelivr.net/npm/programming-languages-logos/src/typescript/typescript.png)`,
                              backgroundSize: "cover",
                              backgroundPosition: "center"
                          }}></div>
        },
    ]

    return (
        <div>
            {
                isLoading && <div className="flex flex-col justify-center items-center h-full w-full">
                    <Spinner/>
                </div>
            }
            {
                !isLoading && <div className="grid grid-cols-4 gap-2">
                    {profileBentoBox.map((item, index) => (
                        <div
                            key={index}
                            className={`bg-white rounded-3xl shadow ${item.span}`}
                            style={{
                                height: `${item.height * 24 + item.heightGap}vh`,
                                width: `${item.width * 12 + item.widthGap}vw`
                            }}
                        >
                            {item.content}
                        </div>
                    ))}
                </div>
            }
        </div>
    )
}