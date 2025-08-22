use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorInstruction {
    pub operation: u8, // 0=add,1=sub,2=mul,3=div,4=mod,5=pow
    pub left: i64,
    pub right: i64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let instruction = CalculatorInstruction::try_from_slice(instruction_data).map_err(
        |_| ProgramError::InvalidInstructionData
    )?;

        let result = match instruction.operation {
        0 => {
            msg!("Addition: {} + {}", instruction.left, instruction.right);
            instruction.left + instruction.right
        },
        1 => {
            msg!("Subtraction: {} - {}", instruction.left, instruction.right);
            instruction.left - instruction.right
        },
        2 => {
            msg!("Multiplication: {} * {}", instruction.left, instruction.right);
            instruction.left * instruction.right
        },
        3 => {
            msg!("Division: {} / {}", instruction.left, instruction.right);
            if instruction.right != 0 {
                instruction.left / instruction.right
            } else {
                msg!("Division by zero is not allowed");
                return Err(ProgramError::InvalidInstructionData);
            }
        },
        4 => {
            msg!("Modulus: {} % {}", instruction.left, instruction.right);
            if instruction.right != 0 {
                instruction.left % instruction.right
            } else {
                msg!("Modulus by zero is not allowed");
                return Err(ProgramError::InvalidInstructionData);
            }
        },
        5 => {
            msg!("Power: {} ^ {}", instruction.left, instruction.right);
            if instruction.right >= 0 {
                instruction.left.pow(instruction.right as u32)
            } else {
                msg!("Negative exponent is not allowed");
                return Err(ProgramError::InvalidInstructionData);
            }
        },
        _ =>{
            msg!("Unknown operation: {}", instruction.operation);
            return Err(ProgramError::InvalidInstructionData);
        }
    };

    msg!("Result = {}", result);

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;  // Import everything from the parent module
    use litesvm::LiteSVM;
    use solana_sdk::{
        instruction::Instruction,
        message::Message,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[test]
    fn test_calculator_program() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();

        svm.airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to airdrop");

        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();

        
        svm.add_program_from_file(
            program_id,
            "target/deploy/byte_calc.so"
        ).expect("Failed to load program");

        
        let instruction_struct = CalculatorInstruction {
            operation: 0, // 0 = add
            left: 10,
            right: 5,
        };
        
        let ix_data = borsh::to_vec(&instruction_struct)
            .expect("Failed to serialize instruction");

        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: ix_data,
        };

        // Build a message and transaction
        let message = Message::new(&[instruction], Some(&payer.pubkey()));
        let recent_blockhash = svm.latest_blockhash();
        let tx = Transaction::new(&[&payer], message, recent_blockhash);

        // Execute
        let result = svm.send_transaction(tx);

        // Print logs to verify output
        println!("Execution result: {:?}", result);
        if let Err(e) = result {
            panic!("Transaction failed: {:?}", e);
        }

        println!("Test completed successfully!");
    }

    #[test]
    fn test_calculator_subtraction() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();

        svm.airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to airdrop");

        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();

        svm.add_program_from_file(
            program_id,
            "target/deploy/byte_calc.so"
        ).expect("Failed to load program");

        
        let instruction_struct = CalculatorInstruction {
            operation: 1,
            left: 20,
            right: 8,
        };
        
        let ix_data = borsh::to_vec(&instruction_struct)
            .expect("Failed to serialize instruction");

        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: ix_data,
        };

        let message = Message::new(&[instruction], Some(&payer.pubkey()));
        let recent_blockhash = svm.latest_blockhash();
        let tx = Transaction::new(&[&payer], message, recent_blockhash);

        let result = svm.send_transaction(tx);
        println!("Subtraction test result: {:?}", result);
        
        assert!(result.is_ok(), "Subtraction test should succeed");
    }

    #[test]
    fn test_calculator_division_by_zero() {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();

        svm.airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to airdrop");

        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();

        svm.add_program_from_file(
            program_id,
            "target/deploy/byte_calc.so"
        ).expect("Failed to load program");

        // Test division by zero: 10 / 0
        let instruction_struct = CalculatorInstruction {
            operation: 3, 
            left: 10,
            right: 0,
        };
        
        let ix_data = borsh::to_vec(&instruction_struct)
            .expect("Failed to serialize instruction");

        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: ix_data,
        };

        let message = Message::new(&[instruction], Some(&payer.pubkey()));
        let recent_blockhash = svm.latest_blockhash();
        let tx = Transaction::new(&[&payer], message, recent_blockhash);

        let result = svm.send_transaction(tx);
        println!("Division by zero test result: {:?}", result);
        
        assert!(result.is_err(), "Division by zero should fail");
    }
}