import {randomBentoBox} from "./utils/bentobox.ts";

function App() {
    const items = randomBentoBox();

    return (
        <>
            <div className="bg-[#f9f0b2] h-screen">
                <div className="flex justify-center items-center flex-col h-full">
                    <div className="flex flex-row justify-center items-center">
                        <p className="text-4xl font-bold">🤓</p>
                        <p className="text-4xl font-bold bg-gradient-to-r from-blue-500 to-green-500 bg-clip-text text-transparent">
                            Github Rank</p>
                    </div>
                    <p className="text-2xl font-bold">快速计算 Github 排名</p>
                </div>
            </div>
            <div className="bg-[#f9f0b2] h-screen">
                <div className="flex justify-center items-center flex-col h-full">
                    <div className="grid grid-rows-4 grid-cols-4 gap-2">
                        {items.map((item, index) => (
                            <div
                                key={index}
                                className={`flex items-center justify-center bg-gray-200 border border-gray-300 rounded-lg shadow ${item.span}`}
                            >
                                {item.index}
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </>
    )
}

export default App
