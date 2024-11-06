import Footer from "../components/Footer.tsx";

export function ErrorScreen() {
    return (
        <div className="bg-[#f9f0b2] h-screen w-screen">
            <div className="flex flex-col justify-center items-center h-full gap-4">
                <div className="text-2xl font-bold">😣 Oops! Something went wrong.</div>
                <p>Please try again later.</p>
            </div>
            <Footer/>
        </div>
    )
}