use crate::error::MailError::NotWritable;
use crate::instruction::MailInstructions;
use crate::state::{DataLength, Mail, MailAccount};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::borsh::get_instance_packed_len;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = MailInstructions::unpack(instruction_data)?;
        match instruction {
            MailInstructions::InitAccount => {
                msg!("Instruction: InitAccount");
                Self::process_init_account(accounts, program_id)
            }
            MailInstructions::SendMail { mail } => {
                msg!("Instruction: SendMail");
                Self::process_send_mail(accounts, mail, program_id)
            }
        }
    }

    fn process_init_account(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let account = next_account_info(accounts_iter)?;

        // Check if account is writable
        if !account.is_writable {
            return Err(NotWritable.into());
        }

        // Check for owner
        if account.owner != program_id {
            return Err(ProgramError::IllegalOwner);
        }

        let welcome_mail = Mail {
            id: String::from("00000000-0000-0000-0000-000000000000"),
            from_address: program_id.to_string(),
            to_address: account.key.to_string(),
            subject: String::from("Welcome to SolMail"),
            body: String::from("This is the start of your private messages on SolMail
            Lorem, ipsum dolor sit amet consectetur adipisicing elit. Quos ut labore, debitis assumenda, dolorem nulla facere soluta exercitationem excepturi provident ipsam reprehenderit repellat quisquam corrupti commodi fugiat iusto quae voluptates!"),
            sent_date: "9/29/2021, 3:58:02 PM".to_string()
          };

        let mail_account = MailAccount {
            sent: vec![welcome_mail],
            inbox: Vec::new(),
        };

        mail_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
        Ok(())
    }

    fn process_send_mail(
        accounts: &[AccountInfo],
        mail: Mail,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let sender_account = next_account_info(accounts_iter)?;
        let receiver_account = next_account_info(accounts_iter)?;
        if !sender_account.is_writable {
            return Err(NotWritable.into());
        }
        if sender_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        if !receiver_account.is_writable {
            return Err(NotWritable.into());
        }
        if receiver_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        let offset: usize = 4;

        // it will get the data length which is required
        let data_length = DataLength::try_from_slice(&sender_account.data.borrow()[..offset])?;
        let mut sender_data;
        if data_length.length > 0 {
            let length =
                usize::try_from(data_length.length + u32::try_from(offset).unwrap()).unwrap();
            sender_data =
                MailAccount::try_from_slice(&sender_account.data.borrow()[offset..length])?;
        } else {
            sender_data = MailAccount {
                inbox: Vec::new(),
                sent: Vec::new(),
            }
        }
        sender_data.sent.push(mail.clone());

        let data_length = DataLength {
            length: u32::try_from(get_instance_packed_len(&sender_data)?).unwrap(),
        };
        data_length.serialize(&mut &mut sender_account.data.borrow_mut()[..offset])?;
        sender_data.serialize(&mut &mut sender_account.data.borrow_mut()[offset..])?;

        let data_length = DataLength::try_from_slice(&receiver_account.data.borrow()[..offset])?;

        let mut receiver_data;
        if data_length.length > 0 {
            let length =
                usize::try_from(data_length.length + u32::try_from(offset).unwrap()).unwrap();
            receiver_data =
                MailAccount::try_from_slice(&receiver_account.data.borrow()[offset..length])?;
        } else {
            receiver_data = MailAccount {
                inbox: Vec::new(),
                sent: Vec::new(),
            }
        }
        receiver_data.inbox.push(mail);

        let data_length = DataLength {
            length: u32::try_from(get_instance_packed_len(&receiver_data)?).unwrap(),
        };
        data_length.serialize(&mut &mut receiver_account.data.borrow_mut()[..offset])?;
        receiver_data.serialize(&mut &mut receiver_account.data.borrow_mut()[offset..])?;
        Ok(())
    }
}
