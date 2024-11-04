import Footer from "../components/Footer.tsx";

export function NotFoundScreen() {
    return (
        <div className="bg-[#fff5e1] h-screen w-screen">
            <div className="fixed bottom-0 right-0 mb-12 mr-12 hidden md:block">
                <img src="mascot.png" alt="Description" className="w-64 h-auto"/>
            </div>
            <div className="flex flex-col justify-center items-center h-full gap-4">
                <div className="text-2xl font-bold">Page Not Found</div>
                <p>The page you are looking for does not exist.</p>
            </div>
            <Footer/>
        </div>
    )
}