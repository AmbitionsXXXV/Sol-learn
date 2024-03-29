import { Keypair } from '@solana/web3.js' // 导入Keypair类，用于生成Solana账户的公钥和私钥
import 'dotenv/config' // 导入dotenv/config模块，用于加载环境变量
import { promises as fs } from 'fs' // 导入fs模块的promises API，用于异步文件操作
import path from 'path' // 导入path模块，用于处理文件和目录的路径

// 定义批量生成账户的数量
const NUM_ACCOUNTS = 10 // 你可以根据需要调整这个值

/**
 * 批量生成 Solana 基本账户并保存到 accounts.json 文件
 * @param numAccounts 批量生成账户的数量
 */
const generateAndSaveAccounts = async (numAccounts: number) => {
  const accounts = [] // 创建一个空数组，用于存储生成的账户信息

  for (let i = 0; i < numAccounts; i++) {
    // 使用 Keypair 的 generate 方法生成一个新的密钥对，这是一个 Solana 账户的基础
    const keypair = Keypair.generate()

    accounts.push({
      publicKey: keypair.publicKey.toBase58(), // 公钥用于接收资金，使用 toBase58 方法转换为 Base58 格式，这是 Solana 公钥的常见表示方式
      secretKey: Array.from(keypair.secretKey), // 私钥用于签名交易，是一个 Uint8Array，使用 Array.from 方法转换为数组，以便于 JSON 序列化
    })
  }

  try {
    // 使用 fs.promises API 的 writeFile 方法异步写文件
    await fs.writeFile(
      path.resolve(__dirname, './accounts.json'),
      JSON.stringify(accounts, null, 2), // 使用 JSON.stringify 方法将 accounts 数组转换为JSON格式的字符串，JSON.stringify 的第二个和第三个参数用于格式化输出，使得生成的 JSON 文件更易读
    )
    console.log(
      `✅ Successfully generated and saved ${numAccounts} accounts to accounts.json`, // 如果文件写入成功，打印成功信息
    )
  } catch (err) {
    // 如果文件写入失败，打印错误信息
    console.error('Failed to save accounts:', err)
  }
}

// 调用异步函数，开始生成账户并保存到文件
await generateAndSaveAccounts(NUM_ACCOUNTS)
