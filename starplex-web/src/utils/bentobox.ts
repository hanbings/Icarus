export function randomBentoBox(row: number, col: number, height: number, width: number, gap: number) {
    enum BoxType {
        base = "base",
        row = "row",
        col = "col",
    }

    const BoxSpan = [
        {
            key: BoxType.base,
            value: `col-span-1 row-span-1`,
            height: height,
            width: width
        },
        {
            key: BoxType.row,
            value: `col-span-2 row-span-1`,
            height: height,
            width: width * 2 + gap
        },
        {
            key: BoxType.col,
            value: `col-span-1 row-span-2`,
            height: height * 2 + gap,
            width: width
        },
    ]

    const Box = [
        {type: BoxType.base, properties: {height: 1, width: 1}},
        {type: BoxType.row, properties: {height: 2, width: 1}},
        {type: BoxType.col, properties: {height: 1, width: 2}},
    ]

    const boxPosition: { box: BoxType, index: number }[][] = [[]]
    let sequence = 0;
    for (let i = 0; i < col; i++) {
        for (let j = 0; j < row; j++) {
            if (boxPosition[i] && boxPosition[i][j]) continue

            let box = Box[Math.floor(Math.random() * Box.length)]
            let width = box.properties.width
            let height = box.properties.height

            if (i + width > col || j + height > row) {
                box = Box[0]
                width = 1
                height = 1
            }

            if (
                (width > 1 || height > 1) &&
                (
                    (boxPosition[i] && boxPosition[i][j + 1]) ||
                    (boxPosition[i + 1] && boxPosition[i + 1][j])
                )
            ) {
                box = Box[0]
                width = 1
                height = 1
            }

            for (let k = 0; k < width; k++) {
                for (let l = 0; l < height; l++) {
                    if (!boxPosition[i + k]) boxPosition[i + k] = []

                    if (!boxPosition[i + k][j + l]) {
                        boxPosition[i + k][j + l] = {box: box.type, index: sequence}
                    }
                }
            }

            sequence = sequence + 1;
        }
    }

    const bento: {
        index: number,
        span: string | undefined,
        height: number | undefined,
        width: number | undefined
    }[] = []
    for (let index = 0; index < sequence; index++) {
        for (let i = 0; i < row; i++) {
            for (let j = 0; j < col; j++) {
                const box = boxPosition[i][j];
                if (box.index === index) {
                    const x = BoxSpan.find(x => x.key === box.box)

                    bento[index] = {
                        index: index,
                        span: x?.value,
                        height: x?.height,
                        width: x?.width
                    }
                }
            }
        }
    }

    return bento
}

export const sloganBentoBoxTemplate = [
    {
        colorFrom: "#2fccba",
        colorTo: "#1abc9c",
        emoji: "❤️",
        title: "感谢有你",
        text: "你是我们的动力源泉！",
        background: undefined
    },
    {
        colorFrom: "#3498db",
        colorTo: "#2980b9",
        emoji: "✨",
        title: "开源无限",
        text: "源源不断的惊喜等你来探索！",
        background: undefined
    },
    {
        colorFrom: "#9b59b6",
        colorTo: "#8e44ad",
        emoji: "💜",
        title: "有趣到爆",
        text: "趣味无限，停不下来！",
        background: undefined
    },
    {
        colorFrom: "#e74c3c",
        colorTo: "#c0392b",
        emoji: "🎃",
        title: "幸运降临",
        text: "今天是你幸运的一天！",
        background: undefined
    },
    {
        colorFrom: "#f1c40f",
        colorTo: "#f39c12",
        emoji: "😂",
        title: "开心无限",
        text: "笑声连连，开心每一天！",
        background: undefined
    },
    {
        colorFrom: "#1abc9c",
        colorTo: "#16a085",
        emoji: "👍",
        title: "超酷无敌",
        text: "颜值与实力并存！",
        background: undefined
    },
    {
        colorFrom: "#e67e22",
        colorTo: "#d35400",
        emoji: "🍀",
        title: "超级幸运",
        text: "好运加持，一路顺风！",
        background: undefined
    },
    {
        colorFrom: "#e74c3c",
        colorTo: "#c0392b",
        emoji: "🍭",
        title: "有趣爆棚",
        text: "每个细节都超有趣！",
        background: undefined
    },
    {
        colorFrom: "#3498db",
        colorTo: "#2980b9",
        emoji: "🚀",
        title: "超级飞跃",
        text: "人生如火箭，势不可挡！",
        background: undefined
    },
    {
        colorFrom: "#2ecc71",
        colorTo: "#27ae60",
        emoji: "🎉",
        title: "美好时光",
        text: "生活太美好，想要一起庆祝！",
        background: undefined
    },
    {
        colorFrom: "#f39c12",
        colorTo: "#e67e22",
        emoji: "🍕",
        title: "超级美味",
        text: "从口到心，绝对美味！",
        background: undefined
    },
    {
        colorFrom: "#e67e22",
        colorTo: "#d35400",
        emoji: "🍀",
        title: "好运连连",
        text: "每天都在等着好运降临！",
        background: undefined
    },
    {
        colorFrom: "#3498db",
        colorTo: "#2980b9",
        emoji: "✨",
        title: "无限开源",
        text: "每个代码背后都闪烁着希望的光！",
        background: undefined
    },
    {
        colorFrom: "#2fccba",
        colorTo: "#1abc9c",
        emoji: "❤️",
        title: "超级能力",
        text: "每一个开发者都能改变世界！",
        background: undefined
    },
]