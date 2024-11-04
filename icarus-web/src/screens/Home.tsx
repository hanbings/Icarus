import {AppStore} from "../stores";
import {useSelector} from "react-redux";
import {useNavigate} from "react-router-dom";
import {useEffect} from "react";

export function HomeScreen() {
    const path = window.location.pathname.split("/")
    if (!path || path.length < 1) path.push("/activity")

    const token = useSelector((state: AppStore) => state.token)
    const navigate = useNavigate();

    useEffect(() => { if (!token.token) navigate("/login") }, [navigate, token.token]);

    return (
        <>
            <p>Home</p>
        </>
    )
}