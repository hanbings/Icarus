# 随机构建 Bento Box 网格

> 写自一只在 bentobox.js 文件（误以为是 .ts 文件）中使用 TypeScript 类型声明语法而百思不得其解到底哪里出问题的猫

思路一：

思路二：

原始代码：

```typescript
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
```

