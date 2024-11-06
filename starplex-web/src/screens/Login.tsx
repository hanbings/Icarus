import {Button, Card, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, useDisclosure} from "@nextui-org/react";
import {useState} from "react";

export default function LoginScreen() {
    const {isOpen, onOpen, onOpenChange} = useDisclosure();
    const [isisTermsOfServiceOpen, setIsisTermsOfServiceOpen] = useState(false);

    return (
        <div className="bg-[#f9f0b2] h-screen w-screen flex justify-center items-center">
            <Card className="flex flex-col gap-4 w-[420px] p-8">
                <div className="flex flex-col gap-2">
                    <h1 className="text-xl font-bold">进行登录</h1>
                    <p className="text-gray-500">
                        使用 Github 登录意味着您已阅读并同意我们的
                        <a className="text-blue-400 m-1 underline underline-offset-2 cursor-pointer" onClick={() => {
                            setIsisTermsOfServiceOpen(true);
                            onOpen();
                        }}>隐私条款</a>
                        以及
                        <a className="text-blue-400 m-1 underline underline-offset-2 cursor-pointer" onClick={() => {
                            setIsisTermsOfServiceOpen(false);
                            onOpen();
                        }}>用户条款</a>
                        您也可以选择不进行登录，但功能将受限。
                    </p>
                </div>
                <Button className="bg-black text-white">
                    <img src={"github-mark.svg"} className="scale-50" alt="Login with Github"/>
                    使用 Github 登录
                </Button>
                <Button className="bg-gray-300">
                    不登陆的情况下继续
                </Button>
            </Card>
            <Modal isOpen={isOpen} onOpenChange={onOpenChange}>
                <ModalContent>
                    {(onClose) => (
                        <>
                            <ModalHeader className="flex flex-col gap-1">
                                {
                                    isisTermsOfServiceOpen ? "隐私条款" : "用户条款"
                                }
                            </ModalHeader>
                            {
                                isisTermsOfServiceOpen ? (
                                    <ModalBody>
                                        <p>
                                            欢迎使用 Icarus Github Rank，
                                            这是我们
                                            （“我们” 指 Icarus Github Rank 的开发团队以及负责软件能够正常运行的运营团队）
                                            的隐私协议
                                            （“隐私协议” 全称 “Icarus Github Rank 用户隐私协议”，下称 “隐私协议” 或 “协议”），
                                            用于向您（“您” 指代正在阅读本协议的自然人）说明我们如何收集、存储和使用您的隐私信息。
                                            请认真、仔细阅读以下内容，确保您充分理解后选择接受或不接受该政策。
                                        </p>
                                        <h2 className="font-bold">一. 收集和使用信息</h2>
                                        <p>
                                            由于数据统计需要，我们将读取您的 Github 个人主页、解析主页的
                                            README、代码仓库、代码文件和非代码文本并可能使用 Open AI ChatGPT 处理这些内容。
                                        </p>
                                        <h2 className="font-bold">二. 数据安全</h2>
                                        <p>
                                            您的数据不会以任何形式传送至除本服务的服务器外的存储介质，并将在七牛云评审完成后进行永久删除。
                                        </p>
                                        <h2 className="font-bold">三. 第三方共享</h2>
                                        <h2 className="font-bold">四. 责任</h2>
                                        <h2 className="font-bold">五. 隐私协议更新</h2>
                                        <h2 className="font-bold">六. 开源</h2>
                                    </ModalBody>
                                ) : (
                                    <ModalBody>
                                        <p>
                                            ❤️ 感谢使用！
                                        </p>
                                    </ModalBody>
                                )
                            }
                            <ModalFooter>
                                <Button color="danger" variant="light" onPress={onClose}>
                                    关闭
                                </Button>
                                <Button color="primary" onPress={onClose}>
                                    同意
                                </Button>
                            </ModalFooter>
                        </>
                    )}
                </ModalContent>
            </Modal>
        </div>
    )
}