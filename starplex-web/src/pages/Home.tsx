import {useSelector} from "react-redux";
import {AppStore} from "../stores";

export default function HomePage() {
    const token = useSelector((state: AppStore) => state.token)
    const account = useSelector((state: AppStore) => state.account)

    return (
        <div>
            {
                token.token && account.account && (
                    <div>
                        <h1>{token.token.token}</h1>
                        <h1>{account.account.avatar}</h1>
                    </div>
                )
            }
        </div>
    )
}