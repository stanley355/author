#[derive(Debug)]
pub(super) enum PromptPayment {
  Student,
  Subscription, 
  MonthlyQuota,
  PaymentRequired 
}