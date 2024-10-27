import {motion} from "framer-motion";

interface ButtonProps {
    text: string,
    onClick: () => void,
    height?: string,
    width?: string,
}

export default function Button(props: ButtonProps) {
    return (
        <motion.button
            className="bg-blue-400 hover:bg-blue-600 text-white py-2 px-4 rounded-xl"
            style={{height: props.height, width: props.width}}
            onClick={props.onClick}
            whileHover={{scale: 1.1}}
            whileTap={{scale: 0.9}}
        >
            {props.text}
        </motion.button>
    )
}