#[derive(Debug)]
pub enum PromptPayment {
  Student,
  Subscription, 
  Balance, 
  MonthlyQuota,
  PaymentRequired 
}