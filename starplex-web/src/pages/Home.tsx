import {useDispatch, useSelector} from "react-redux";
import {AppStore} from "../stores";
import {Avatar, Button, Card, Chip, Link} from "@nextui-org/react";
import {useNavigate} from "react-router-dom";
import {clearToken} from "../stores/token.ts";
import {clearAccount} from "../stores/account.ts";

export default function HomePage() {
    const navigate = useNavigate()
    const token = useSelector((state: AppStore) => state.token)
    const account = useSelector((state: AppStore) => state.account)

    const dispatch = useDispatch()

    function clear() {
        dispatch(clearToken())
        dispatch(clearAccount())

        navigate("/rank")
    }

    return (
        <div className="flex flex-col gap-4 w-full md:w-[420px]">
            {
                token.token && account.account && (
                    <Card className="flex flex-col gap-2 p-8">
                        <div className="flex flex-col gap-1">
                            <Avatar size="lg" src={account.account.avatar}/>
                            <div>
                                <Link isBlock showAnchorIcon href={`https://github.com/${account.account.username}`}
                                      className="text-green-400">
                                    {`@${account.account.username}`}
                                </Link>
                            </div>
                        </div>
                        <div className="flex flex-col gap-1">
                            {
                                account.account.email && account.account.email.map(e => {
                                    return (
                                        <div key={e}>
                                            <Chip className="bg-green-200">
                                                {e}
                                            </Chip>
                                        </div>
                                    )
                                })
                            }
                        </div>
                        <Button className="bg-red-600 text-white" onClick={clear}>
                            退出登录
                        </Button>
                    </Card>
                )
            }
        </div>
    )
}