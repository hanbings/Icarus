import Footer from "../components/Footer.tsx";
import {Button, Card, CardBody, Input} from "@nextui-org/react";
import {useDispatch, useSelector} from "react-redux";
import {AppStore} from "../stores";
import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import {setToken} from "../stores/token.ts";

export function LoginScreen() {
    const token = useSelector((state: AppStore) => state.token)
    const dispatch = useDispatch()

    const navigate = useNavigate()
    const [inputValue, setInputValue] = useState("");

    useEffect(() => {
        if (token.token) {
            navigate("/home")
        }
    })

    const onLogin = () => {
        dispatch(setToken(inputValue))
        window.location.href = "/home"
    }

    return (
        <div className="bg-[#fff5e1] h-screen w-screen">
            <div className="fixed bottom-0 right-0 mb-12 mr-12 hidden md:block">
                <img src="mascot.png" alt="Description" className="w-64 h-auto"/>
            </div>
            <div className="flex justify-center items-center h-full">
                <Card>
                    <CardBody className="p-4 flex flex-col gap-4 w-[340px]">
                        <div>
                            <h1 className="text-2xl">Login</h1>
                            <p className="text-gray-500">Ask the administrator to get a token.</p>
                        </div>
                        <Input type="password" label="Token" value={inputValue} onValueChange={setInputValue}/>
                        <Button color="primary" onClick={onLogin}>
                            Login
                        </Button>
                    </CardBody>
                </Card>
            </div>
            <Footer/>
        </div>
    )
}