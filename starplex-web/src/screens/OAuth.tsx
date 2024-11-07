import {useNavigate} from "react-router-dom";
import {useDispatch, useSelector} from "react-redux";
import {Card, Spinner} from "@nextui-org/react";
import {useQuery} from "@tanstack/react-query";
import {StarplexConfig} from "../config.ts";
import {useEffect} from "react";
import {setToken} from "../stores/token.ts";
import axios from "axios";
import {setAccount} from "../stores/account.ts";
import {AppStore} from "../stores";

export default function OAuthScreen() {
    const queryParams = new URLSearchParams(location.search);
    const code = queryParams.get('code');
    const state = queryParams.get('state');

    const token = useSelector((state: AppStore) => state.token)
    const account = useSelector((state: AppStore) => state.account)

    const dispatch = useDispatch()
    const navigate = useNavigate()
    const {data} = useQuery({
        queryKey: ["oauth_token", state, code],
        queryFn: () => axios.get(`${StarplexConfig.api}/oauth/github/callback?state=${state}&code=${code}`).then(data => data.data),
    })

    useEffect(() => {
        if (data && data.token && data.account) {
            dispatch(setToken(data.token))
            dispatch(setAccount(data.account))
            navigate("/rank")
        }

        if (token.token && account.account) {
            navigate("/rank")
        }
    })

    return (
        <div className="bg-[#f9f0b2] h-screen w-screen flex justify-center items-center">
            <Card className="flex flex-col gap-4 w-[420px] p-8">
                <div className="flex flex-col gap-2">
                    <h1 className="text-xl font-bold">Github OAuth</h1>
                    <p className="text-gray-500">请稍等</p>
                    <div className="h-[64px] flex flex-col justify-center items-center">
                        <Spinner/>
                    </div>
                </div>
            </Card>
        </div>
    )
}
