import {Button, Tab, Tabs} from "@nextui-org/react";
import RankPage from "../pages/Rank.tsx";
import HomePage from "../pages/Home.tsx";
import ProfilePage from "../pages/Profile.tsx";
import {Token} from "../types.ts";
import {useSelector} from "react-redux";
import {AppStore} from "../stores";
import axios from "axios";
import {StarplexConfig} from "../config.ts";

export default function RankScreen() {
    const token = useSelector((state: AppStore) => state.token)
    const account = useSelector((state: AppStore) => state.account)

    const tabs = [
        {id: "1", label: "Rank", content: getRankPage()},
        {id: "2", label: "Home", content: getHomePage(token.token)},
        {id: "3", label: "Profile", content: getProfilePage(account.account?.username)},
    ];

    return (
        <div className="bg-[#f9f0b2] h-screen w-screen flex justify-center items-center">
            <div className="bg-[#f9f0b2] flex h-screen w-full flex-col justify-center items-center">
                <Tabs aria-label="Dynamic tabs" items={tabs}
                      className="py-4 top-0 left-0 right-0 z-50 items-center justify-center">
                    {(item) => (
                        <Tab key={item.id} title={item.label} className="h-full w-full px-4">
                            {item.content}
                        </Tab>
                    )}
                </Tabs>
            </div>
        </div>
    )
}

function getRankPage() {
    return (
        <div className="bg-[#f9f0b2] h-full w-full flex justify-center">
            <RankPage/>
        </div>
    )
}

function getHomePage(token: Token | undefined) {
    return (
        <div className="bg-[#f9f0b2] h-full w-full flex justify-center">
            {
                token ?
                    <HomePage/> :
                    (
                        <Button className="bg-black text-white"
                                onClick={
                                    () => axios
                                        .get(`${StarplexConfig.api}/oauth/github`)
                                        .then(data => window.location.href = data.data)}>
                            <img src={"github-mark.svg"} className="scale-50" alt="Login with Github"/>
                            使用 Github 登录
                        </Button>
                    )
            }
        </div>
    )
}

function getProfilePage(username: string | undefined) {
    return (
        <div className="bg-[#f9f0b2] h-full w-full flex justify-center items-center">
            {
                username ?
                    <ProfilePage username={username} isProfilePage={false}/> :
                    (
                        <Button className="bg-black text-white"
                                onClick={
                                    () => axios
                                        .get(`${StarplexConfig.api}/oauth/github`)
                                        .then(data => window.location.href = data.data)}>
                            <img src={"github-mark.svg"} className="scale-50" alt="Login with Github"/>
                            使用 Github 登录
                        </Button>
                    )
            }
        </div>
    )
}