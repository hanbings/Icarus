import Footer from "../components/Footer.tsx";

export function NotFoundScreen() {
    return (
        <div className="bg-[#f9f0b2] h-screen w-screen">
            <div className="flex flex-col justify-center items-center h-full gap-4">
                <div className="text-2xl font-bold">😶‍🌫️ Page Not Found</div>
                <p>The page you are looking for does not exist.</p>
            </div>
            <Footer/>
        </div>
    )
}