import {useEffect, useMemo, useState} from "react";
import {randomBentoBox, sloganBentoBoxTemplate} from "../utils/bentobox.ts";
import {motion} from "framer-motion";
import {Button} from "@nextui-org/react";
import {useNavigate} from "react-router-dom";

export default function WelcomeScreen() {
    const navigate = useNavigate()
    const texts = useMemo(() => ["✨", "🚀", "😶‍🌫️", "🎉", "🎁"], [])
    const [currentText, setCurrentText] = useState(texts[0])
    const [index, setIndex] = useState(0)
    const [bento, setBento] = useState(randomBentoBox(4, 4, 24, 12, 0.5))

    useEffect(() => setCurrentText(texts[index]), [index, texts])
    useEffect(() => {
        const titleEmoji = setInterval(() => {
            setIndex((prevIndex) => (prevIndex + 1) % texts.length)
        }, 2000)

        return () => clearInterval(titleEmoji)
    }, [texts.length])
    useEffect(() => {
        const bentoBox = setInterval(() => setBento(randomBentoBox(4, 4, 24, 12, 0.5)), 10000)

        return () => clearInterval(bentoBox)
    }, [])

    return (
        <div className="bg-[#f9f0b2] flex h-screen relative overflow-hidden">
            <div className="flex flex-col justify-center items-center h-full w-full md:w-1/2 gap-3">
                <div className="flex flex-col gap-3 justify-center items-center h-full">
                    <div className="flex flex-row justify-center items-center gap-3">
                        <motion.p
                            key={currentText}
                            initial={{opacity: 0, scale: 0.5}}
                            animate={{opacity: 1, scale: 1}}
                            transition={{duration: 0.5}}
                            className="text-4xl font-bold">
                            {currentText}
                        </motion.p>
                        <motion.p className="
                            text-4xl font-bold
                            bg-gradient-to-r from-blue-500 to-green-500 bg-clip-text text-transparent"
                        >
                            Github Rank
                        </motion.p>
                    </div>
                    <p className="text-2xl font-bold">快速计算 Github 排名</p>
                    <Button color="secondary" onClick={() => navigate("/login")}>
                        开始探索
                    </Button>
                </div>
                <footer className="text-center py-4 bottom-0 left-0 right-0 z-50">
                    <a className="text-gray-500" href="https://github.com/hanbings/icarus">❤ Created by Icarus
                        Project</a>
                </footer>
            </div>
            <div className=" hidden md:flex flex-col justify-center items-center h-full gap-3">
                <div className="absolute right-0 grid grid-rows-4 grid-cols-4 gap-[0.5rem]">
                    {bento.map((item) => (
                        <div
                            key={item.index}
                            className={` 
                                    flex flex-col items-center justify-center gap-2
                                    shadow hover:shadow-2xl
                                    rounded-2xl
                                    ${item.span}
                                    ${
                                sloganBentoBoxTemplate[item.index]?.background ?
                                    `bg-${sloganBentoBoxTemplate[item.index].background}` :
                                    "bg-white"
                            }
                                `}
                            style={{
                                height: `${item.height}vh`,
                                width: `${item.width}vw`,
                            }}
                        >
                            {
                                sloganBentoBoxTemplate[item.index] && (
                                    <>
                                        {
                                            sloganBentoBoxTemplate[item.index].emoji && (
                                                <motion.p
                                                    key={item.index}
                                                    initial={{opacity: 0, scale: 0.5}}
                                                    animate={{opacity: 1, scale: 1}}
                                                    transition={{duration: 0.5}}
                                                    className="text-4xl font-bold"
                                                >
                                                    {sloganBentoBoxTemplate[item.index].emoji}
                                                </motion.p>
                                            )
                                        }
                                        {
                                            sloganBentoBoxTemplate[item.index].emoji && (
                                                <p className="text-4xl font-bold"
                                                   style={{
                                                       background:
                                                           `
                                                               linear-gradient(
                                                                   to right, 
                                                                   ${sloganBentoBoxTemplate[item.index].colorTo}, 
                                                                   ${sloganBentoBoxTemplate[item.index].colorFrom}
                                                               )
                                                           `,
                                                       WebkitBackgroundClip: 'text',
                                                       backgroundClip: 'text',
                                                       color: 'transparent',
                                                   }}
                                                >
                                                    {sloganBentoBoxTemplate[item.index].title}
                                                </p>
                                            )
                                        }
                                        {
                                            sloganBentoBoxTemplate[item.index].text && (
                                                <p>{sloganBentoBoxTemplate[item.index].text}</p>
                                            )
                                        }
                                    </>
                                )
                            }
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}