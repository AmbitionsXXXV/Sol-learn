use std::str::FromStr;

use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;

/// 定义监控错误枚举，用于处理各种可能出现的错误情况
#[derive(Debug, Error)]
pub enum MonitorError {
  #[error("未找到目标程序 ID")]
  ProgramIdNotFound,
  #[error("不支持的交易格式")]
  UnsupportedTransactionFormat,
  #[error("未找到匹配的指令")]
  NoMatchingInstruction,
}

/// 定义 Raydium 指令结构体，用于序列化和反序列化
/// 这个结构体表示 Raydium 协议中的一个具体指令
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RaydiumInstruction {
  pub discriminator: u8,     // 用于区分不同类型指令的标识符
  pub nonce: u8,             // 用于防止重放攻击的随机数
  pub opentime: u64,         // 流动性池开放时间（Unix 时间戳）
  pub init_pc_amount: u64,   // 初始报价代币数量
  pub init_coin_amount: u64, // 初始基础代币数量
}

/// 定义数据版本 2 结构体，可能用于新版本的 Raydium 指令
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SwapIxData {
  pub discriminator: u8,       // 指令类型标识符
  pub amount_in: u64,          // 输入代币数量
  pub minimum_amount_out: u64, // 最小输出代币数量（滑点保护）
}

/// 定义指令数据值枚举，用于存储不同类型的指令数据
#[derive(Debug, Clone)]
pub enum InstructionDataValue {
  AccountsAndData {
    accounts: Vec<String>, // 相关账户地址列表
    data: Option<String>,  // 可选的额外数据
  },
  Amount(u64), // 代币数量
}

/// 指令数据结构体，用于存储从交易中提取的指令信息
#[derive(Debug, Clone)]
pub struct InstructionData {
  pub value: InstructionDataValue,
}

/// 定义 ToPubkey trait，用于将不同类型转换为 Solana 的公钥（Pubkey）
pub trait ToPubkey {
  fn to_pubkey(&self) -> Result<Pubkey>;
}

/// 为 str 类型实现 ToPubkey trait
impl ToPubkey for str {
  fn to_pubkey(&self) -> Result<Pubkey> {
    Pubkey::from_str(self).map_err(|e| e.into())
  }
}

/// 为 String 类型实现 ToPubkey trait
impl ToPubkey for String {
  fn to_pubkey(&self) -> Result<Pubkey> {
    self.as_str().to_pubkey()
  }
}

/// 为 Pubkey 类型实现 ToPubkey trait（直接返回自身）
impl ToPubkey for Pubkey {
  fn to_pubkey(&self) -> Result<Pubkey> {
    Ok(*self)
  }
}

/// 为实现了 ToPubkey trait 的类型的引用实现 ToPubkey trait
impl<T: ToPubkey + ?Sized> ToPubkey for &T {
  fn to_pubkey(&self) -> Result<Pubkey> {
    (*self).to_pubkey()
  }
}