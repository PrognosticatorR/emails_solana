use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Mail {
    pub id: String,
    pub from_address: String,
    pub to_address: String,
    pub sent_date: String,
    pub subject: String,
    pub body: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MailAccount {
    pub inbox: Vec<Mail>,
    pub sent: Vec<Mail>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct DataLength {
    pub length: u32,
}
