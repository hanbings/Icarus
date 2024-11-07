import {useParams} from "react-router-dom";
import ProfilePage from "../pages/Profile.tsx";

export default function ProfileScreen() {
    const {username} = useParams();

    return (
        <div className="bg-[#f9f0b2] h-screen w-screen flex justify-center items-center">
            {
                username ?
                    <ProfilePage username={username} isProfilePage={true}/> :
                    (
                        <div className="flex flex-col justify-center items-center h-full gap-4">
                            <div className="text-2xl font-bold">😶‍🌫️ Page Not Found</div>
                            <p>The page you are looking for does not exist.</p>
                        </div>
                    )
            }
        </div>
    )
}